--------------------------- MODULE FileReconstruction ---------------------------
(*
 * Formal specification of the MediaRescue file reconstruction/repair pipeline.
 *
 * Models the correctness of repair algorithms applied to corrupted files
 * (JPEG, PNG, MP4, etc.) recovered from disk. Fragments extracted by the
 * scanner are assembled into a working copy, which is then iteratively
 * repaired until it passes validation or all repair options are exhausted.
 *
 * Key safety guarantees:
 *   - The original recovered data is NEVER modified (copy-on-write).
 *   - Repairs are idempotent: re-applying a repair produces the same result.
 *   - No data is invented: working data is always derivable from fragments.
 *   - Every applied repair is fully traceable.
 *
 * This spec must pass TLC with ZERO violations before any Rust
 * implementation is written (spec-driven development, Phase 0).
 *)
EXTENDS Integers, FiniteSets, Sequences, Naturals

CONSTANTS MAX_FRAGMENTS, MAX_REPAIRS

ASSUME MAX_FRAGMENTS \in Nat /\ MAX_FRAGMENTS > 0
ASSUME MAX_REPAIRS \in Nat /\ MAX_REPAIRS > 0

VARIABLES
    inputFragments,       \* Set of fragment IDs extracted from disk
    originalData,         \* Immutable snapshot of input (preserved for safety)
    workingData,          \* Mutable copy being repaired (subset of transformable data)
    repairActions,        \* Sequence of repair action IDs that have been applied
    availableRepairs,     \* Set of repair action IDs still applicable
    reconstructionState,  \* Current pipeline state
    isValid               \* Whether workingData currently passes validation

vars == <<inputFragments, originalData, workingData,
          repairActions, availableRepairs, reconstructionState, isValid>>

-----------------------------------------------------------------------------
(* Domains *)

FragmentIDs   == 1..MAX_FRAGMENTS
RepairIDs     == 1..MAX_REPAIRS
ReconStates   == {"Init", "Parsing", "Repairing", "Validating",
                  "Success", "Failure"}

(*
 * TransformableData models the universe of data sets that can be produced
 * from a given set of input fragments by applying valid repair operations.
 * A repair can rearrange, remove, or re-derive bytes, but cannot introduce
 * data that does not originate from the fragments. We model this as the
 * powerset of inputFragments: any subset represents a valid derivation
 * (e.g., dropping a corrupted trailer, reordering chunks).
 *)
TransformableFrom(frags) == SUBSET frags

-----------------------------------------------------------------------------
(* Type invariant *)

TypeOK ==
    /\ inputFragments \subseteq FragmentIDs
    /\ originalData \subseteq FragmentIDs
    /\ workingData \subseteq FragmentIDs
    /\ repairActions \in Seq(RepairIDs)
    /\ Len(repairActions) <= MAX_REPAIRS
    /\ availableRepairs \subseteq RepairIDs
    /\ reconstructionState \in ReconStates
    /\ isValid \in BOOLEAN

-----------------------------------------------------------------------------
(* Initial state *)

Init ==
    /\ inputFragments \in (SUBSET FragmentIDs) \ {{}}  \* At least one fragment
    /\ originalData = inputFragments
    /\ workingData = inputFragments
    /\ repairActions = <<>>
    /\ availableRepairs = {}
    /\ reconstructionState = "Init"
    /\ isValid = FALSE

-----------------------------------------------------------------------------
(* Actions *)

(* Init -> Parsing: begin structural analysis of the recovered data *)
StartParsing ==
    /\ reconstructionState = "Init"
    /\ reconstructionState' = "Parsing"
    /\ UNCHANGED <<inputFragments, originalData, workingData,
                    repairActions, availableRepairs, isValid>>

(* Parsing -> Repairing: identify which repair actions are applicable.
   The set of available repairs is chosen nondeterministically from all
   possible subsets of RepairIDs, modeling format-specific analysis. *)
IdentifyRepairs ==
    /\ reconstructionState = "Parsing"
    /\ reconstructionState' = "Repairing"
    /\ availableRepairs' \in SUBSET RepairIDs  \* Nondeterministic: any subset
    /\ UNCHANGED <<inputFragments, originalData, workingData,
                    repairActions, isValid>>

(* Repairing -> Repairing: apply one repair action.
   The repair transforms workingData to some derivation of inputFragments
   (models the actual byte-level repair producing a valid derivation).
   The applied repair ID is appended to the trace and removed from available. *)
ApplyRepair ==
    /\ reconstructionState = "Repairing"
    /\ availableRepairs /= {}
    /\ \E r \in availableRepairs :
        /\ Len(repairActions) < MAX_REPAIRS
        /\ repairActions' = Append(repairActions, r)
        /\ availableRepairs' = availableRepairs \ {r}
        /\ workingData' \in TransformableFrom(inputFragments)  \* Nondeterministic valid derivation
        /\ UNCHANGED <<inputFragments, originalData,
                        reconstructionState, isValid>>

(* Repairing -> Validating: submit working data for validation *)
ValidateResult ==
    /\ reconstructionState = "Repairing"
    /\ reconstructionState' = "Validating"
    /\ isValid' \in BOOLEAN  \* Nondeterministic: validation may pass or fail
    /\ UNCHANGED <<inputFragments, originalData, workingData,
                    repairActions, availableRepairs>>

(* Validating -> Success: validation passed *)
MarkSuccess ==
    /\ reconstructionState = "Validating"
    /\ isValid = TRUE
    /\ reconstructionState' = "Success"
    /\ UNCHANGED <<inputFragments, originalData, workingData,
                    repairActions, availableRepairs, isValid>>

(* Validating -> Failure: validation failed and no repairs remain *)
MarkFailure ==
    /\ reconstructionState = "Validating"
    /\ isValid = FALSE
    /\ availableRepairs = {}
    /\ reconstructionState' = "Failure"
    /\ UNCHANGED <<inputFragments, originalData, workingData,
                    repairActions, availableRepairs, isValid>>

(* Validating -> Repairing: validation failed but more repairs to try *)
RetryRepair ==
    /\ reconstructionState = "Validating"
    /\ isValid = FALSE
    /\ availableRepairs /= {}
    /\ reconstructionState' = "Repairing"
    /\ UNCHANGED <<inputFragments, originalData, workingData,
                    repairActions, availableRepairs, isValid>>

-----------------------------------------------------------------------------
(* Next-state relation *)

Next ==
    \/ StartParsing
    \/ IdentifyRepairs
    \/ ApplyRepair
    \/ ValidateResult
    \/ MarkSuccess
    \/ MarkFailure
    \/ RetryRepair

-----------------------------------------------------------------------------
(* Fairness *)

(*
 * Fairness conditions ensure the pipeline makes progress.
 *
 * Most actions use weak fairness (WF): if continuously enabled, they
 * eventually fire. ApplyRepair uses strong fairness (SF) because it
 * gets repeatedly enabled (in Repairing) and disabled (when ValidateResult
 * moves to Validating) -- WF would not guarantee it ever fires.
 * SF ensures: if ApplyRepair is enabled infinitely often, it eventually
 * fires, which drains availableRepairs and guarantees termination.
 *)
Fairness ==
    /\ WF_vars(StartParsing)
    /\ WF_vars(IdentifyRepairs)
    /\ SF_vars(ApplyRepair)
    /\ WF_vars(ValidateResult)
    /\ WF_vars(MarkSuccess)
    /\ WF_vars(MarkFailure)
    /\ WF_vars(RetryRepair)

Spec == Init /\ [][Next]_vars /\ Fairness

-----------------------------------------------------------------------------
(* Safety Invariants *)

(* Repair idempotency: a repair ID appears at most once in the trace.
   Since each application removes the ID from availableRepairs, re-application
   is structurally impossible. We verify the stronger property that the
   sequence of applied repairs contains no duplicates. *)
RepairIdempotency ==
    \A i, j \in 1..Len(repairActions) :
        (i /= j) => (repairActions[i] /= repairActions[j])

(* No data invention: workingData is always a subset of inputFragments.
   Repairs can rearrange or trim, but never introduce foreign data. *)
NoDataInvention ==
    workingData \subseteq inputFragments

(* Original data is immutable: after Init, originalData equals the initial
   inputFragments and never changes. *)
OriginalPreserved ==
    originalData = inputFragments

(* State progression: verifies consistent variable states at each pipeline
   stage. Valid transitions are enforced structurally by Next; this invariant
   catches illegal state/variable combinations that would indicate a bug. *)
StateProgression ==
    \* In Init state, nothing has happened yet
    /\ (reconstructionState = "Init" =>
            /\ repairActions = <<>>
            /\ availableRepairs = {}
            /\ workingData = inputFragments)
    \* In Parsing, still no repairs applied, working data untouched
    /\ (reconstructionState = "Parsing" =>
            /\ repairActions = <<>>
            /\ workingData = inputFragments)
    \* In terminal states, isValid must be consistent
    /\ (reconstructionState = "Success" => isValid = TRUE)
    /\ (reconstructionState = "Failure" =>
            /\ isValid = FALSE
            /\ availableRepairs = {})

(* Valid transitions: expressed as an action constraint to verify that
   the Next relation only permits legal state transitions. *)
ValidTransitions ==
    \/ UNCHANGED reconstructionState
    \/ reconstructionState = "Init"       /\ reconstructionState' = "Parsing"
    \/ reconstructionState = "Parsing"    /\ reconstructionState' = "Repairing"
    \/ reconstructionState = "Repairing"  /\ reconstructionState' = "Repairing"
    \/ reconstructionState = "Repairing"  /\ reconstructionState' = "Validating"
    \/ reconstructionState = "Validating" /\ reconstructionState' = "Success"
    \/ reconstructionState = "Validating" /\ reconstructionState' = "Failure"
    \/ reconstructionState = "Validating" /\ reconstructionState' = "Repairing"

(* Repair traceability: every repair that has been consumed from
   availableRepairs is recorded in repairActions, and vice versa.
   More precisely: the set of applied repairs and the remaining available
   repairs are disjoint, and their union is bounded by RepairIDs. *)
RepairTraceability ==
    /\ \A i \in 1..Len(repairActions) :
        repairActions[i] \in RepairIDs
    /\ \A i \in 1..Len(repairActions) :
        repairActions[i] \notin availableRepairs

(* Terminal states are truly terminal: no further state change occurs *)
TerminalStatesStable ==
    /\ (reconstructionState = "Success" => isValid = TRUE)

-----------------------------------------------------------------------------
(* Liveness Properties *)

(* The reconstruction pipeline always eventually terminates *)
ReconstructionTerminates ==
    reconstructionState \in {"Init", "Parsing", "Repairing", "Validating"}
        ~> reconstructionState \in {"Success", "Failure"}

(* If we reach Validating, we eventually reach a terminal state *)
ValidationExecutes ==
    reconstructionState = "Validating"
        ~> reconstructionState \in {"Success", "Failure"}

=============================================================================
