--------------------------- MODULE RecoveryPipeline ---------------------------
(*
 * RecoveryPipeline - Formal specification of the multi-stage file recovery
 * pipeline for the MediaRescue project.
 *
 * Files move through: Discovered -> Carving -> Carved -> Reconstructing ->
 *   Reconstructed -> Validating -> Valid | Invalid | Failed
 *
 * Multiple workers process files concurrently. A worker picks up a file in a
 * processable resting state, transitions it to the corresponding processing
 * state, and upon completion either advances it to the next resting state or
 * marks it Failed.
 *)

EXTENDS Integers, FiniteSets

CONSTANTS MAX_FILES, MAX_WORKERS

FileIds   == 1..MAX_FILES
WorkerIds == 1..MAX_WORKERS

AllStates == {
    "Discovered", "Carving", "Carved",
    "Reconstructing", "Reconstructed",
    "Validating", "Valid", "Invalid", "Failed"
}

ProcessingStates == {"Carving", "Reconstructing", "Validating"}
TerminalStates   == {"Valid", "Invalid", "Failed"}
RestingStates    == {"Discovered", "Carved", "Reconstructed"}

(* Defines the allowed forward transitions for each state. *)
NextProcessingState(s) ==
    CASE s = "Discovered"    -> "Carving"
      [] s = "Carved"        -> "Reconstructing"
      [] s = "Reconstructed" -> "Validating"

NextRestedState(s) ==
    CASE s = "Carving"        -> "Carved"
      [] s = "Reconstructing" -> "Reconstructed"
      [] s = "Validating"     -> "Valid"

(* Numeric ordering for proving monotonic progression. *)
StateOrd(s) ==
    CASE s = "Discovered"     -> 0
      [] s = "Carving"        -> 1
      [] s = "Carved"         -> 2
      [] s = "Reconstructing" -> 3
      [] s = "Reconstructed"  -> 4
      [] s = "Validating"     -> 5
      [] s = "Valid"          -> 6
      [] s = "Invalid"        -> 6
      [] s = "Failed"         -> 7

-----------------------------------------------------------------------------

VARIABLES
    files,             \* Function FileId -> FileState
    workerAssignment,  \* Function WorkerId -> FileId \union {"idle"}
    completedFiles,    \* Set of file IDs in terminal states
    pipelineActive,    \* Boolean: pipeline still running
    prevFiles          \* Ghost variable: previous file states for progression check

vars == <<files, workerAssignment, completedFiles, pipelineActive, prevFiles>>

-----------------------------------------------------------------------------
(* Helper operators *)

IdleWorkers == {w \in WorkerIds : workerAssignment[w] = "idle"}

AssignedFiles == {workerAssignment[w] : w \in WorkerIds} \ {"idle"}

(* A file is processable if it is in a resting state and not already claimed. *)
ProcessableFiles ==
    {f \in FileIds : files[f] \in RestingStates /\ f \notin AssignedFiles}

(* Which worker (if any) is assigned to a given file. *)
WorkerOf(f) == CHOOSE w \in WorkerIds : workerAssignment[w] = f

BusyWorkers == {w \in WorkerIds : workerAssignment[w] /= "idle"}

AllTerminated == \A f \in FileIds : files[f] \in TerminalStates

-----------------------------------------------------------------------------
(* Initial state *)

Init ==
    /\ files            = [f \in FileIds |-> "Discovered"]
    /\ workerAssignment = [w \in WorkerIds |-> "idle"]
    /\ completedFiles   = {}
    /\ pipelineActive   = TRUE
    /\ prevFiles        = [f \in FileIds |-> "Discovered"]

-----------------------------------------------------------------------------
(* Actions *)

(*
 * AssignWorker: An idle worker picks up a processable file and transitions
 * it from a resting state to the corresponding processing state.
 *)
AssignWorker(w, f) ==
    /\ pipelineActive
    /\ workerAssignment[w] = "idle"
    /\ f \in ProcessableFiles
    /\ prevFiles' = files
    /\ files' = [files EXCEPT ![f] = NextProcessingState(files[f])]
    /\ workerAssignment' = [workerAssignment EXCEPT ![w] = f]
    /\ UNCHANGED <<completedFiles, pipelineActive>>

(*
 * CompleteStage: A worker finishes processing its assigned file and advances
 * the file to the next resting/terminal state. The worker becomes idle.
 *)
CompleteStage(w) ==
    /\ pipelineActive
    /\ workerAssignment[w] /= "idle"
    /\ LET f == workerAssignment[w]
       IN /\ files[f] \in ProcessingStates
          /\ LET nextState == NextRestedState(files[f])
             IN /\ prevFiles' = files
                /\ files' = [files EXCEPT ![f] = nextState]
                /\ workerAssignment' = [workerAssignment EXCEPT ![w] = "idle"]
                /\ completedFiles' =
                       IF nextState \in TerminalStates
                       THEN completedFiles \union {f}
                       ELSE completedFiles
                /\ UNCHANGED pipelineActive

(*
 * ValidateInvalid: During the Validating stage a worker may determine the
 * file is invalid rather than valid. The worker becomes idle.
 *)
ValidateInvalid(w) ==
    /\ pipelineActive
    /\ workerAssignment[w] /= "idle"
    /\ LET f == workerAssignment[w]
       IN /\ files[f] = "Validating"
          /\ prevFiles' = files
          /\ files' = [files EXCEPT ![f] = "Invalid"]
          /\ workerAssignment' = [workerAssignment EXCEPT ![w] = "idle"]
          /\ completedFiles' = completedFiles \union {f}
          /\ UNCHANGED pipelineActive

(*
 * FailFile: A worker marks its assigned file as Failed. Can happen in any
 * processing state. The worker becomes idle.
 *)
FailFile(w) ==
    /\ pipelineActive
    /\ workerAssignment[w] /= "idle"
    /\ LET f == workerAssignment[w]
       IN /\ files[f] \in ProcessingStates
          /\ prevFiles' = files
          /\ files' = [files EXCEPT ![f] = "Failed"]
          /\ workerAssignment' = [workerAssignment EXCEPT ![w] = "idle"]
          /\ completedFiles' = completedFiles \union {f}
          /\ UNCHANGED pipelineActive

(*
 * ShutdownPipeline: When every file has reached a terminal state and all
 * workers are idle, the pipeline deactivates.
 *)
ShutdownPipeline ==
    /\ pipelineActive
    /\ AllTerminated
    /\ IdleWorkers = WorkerIds
    /\ pipelineActive' = FALSE
    /\ UNCHANGED <<files, workerAssignment, completedFiles>>
    /\ prevFiles' = files

-----------------------------------------------------------------------------
(* Next-state relation *)

Next ==
    \/ \E w \in WorkerIds, f \in FileIds : AssignWorker(w, f)
    \/ \E w \in WorkerIds : CompleteStage(w)
    \/ \E w \in WorkerIds : ValidateInvalid(w)
    \/ \E w \in WorkerIds : FailFile(w)
    \/ ShutdownPipeline

(* Fairness: every enabled action must eventually be taken. *)
Fairness ==
    /\ \A w \in WorkerIds, f \in FileIds :
           WF_vars(AssignWorker(w, f))
    /\ \A w \in WorkerIds :
           WF_vars(CompleteStage(w))
    /\ \A w \in WorkerIds :
           WF_vars(ValidateInvalid(w))
    \* FailFile intentionally has NO fairness -- failure is possible, not guaranteed.
    /\ WF_vars(ShutdownPipeline)

Spec == Init /\ [][Next]_vars /\ Fairness

-----------------------------------------------------------------------------
(* Safety Invariants *)

TypeOK ==
    /\ files            \in [FileIds -> AllStates]
    /\ workerAssignment \in [WorkerIds -> FileIds \union {"idle"}]
    /\ completedFiles   \subseteq FileIds
    /\ pipelineActive   \in BOOLEAN
    /\ prevFiles        \in [FileIds -> AllStates]

(* File states only move forward -- never regress. *)
FileStateProgression ==
    \A f \in FileIds : StateOrd(files[f]) >= StateOrd(prevFiles[f])

(* No two workers assigned to the same file at the same time. *)
NoDoubleClaim ==
    \A w1, w2 \in WorkerIds :
        (w1 /= w2 /\ workerAssignment[w1] /= "idle")
            => workerAssignment[w1] /= workerAssignment[w2]

(* If a worker is assigned to a file, that file must be in a processing state. *)
WorkerConsistency ==
    \A w \in WorkerIds :
        workerAssignment[w] /= "idle"
            => files[workerAssignment[w]] \in ProcessingStates

(*
 * OutputIntegrity: Any file in a post-carving terminal state must have
 * progressed through every required prior stage. Since StateOrd is monotonic
 * and we only advance through the defined transitions, a file at Valid (ord 6)
 * or Invalid (ord 6) necessarily passed through all prior stages. A Failed
 * file (ord 7) may have failed at any processing stage, which is acceptable.
 * We express this as: a Valid or Invalid file has ord >= 6 (enforced by
 * construction), and was not placed there from Discovered directly.
 * The real guarantee comes from FileStateProgression + the transition
 * definitions, but we add an explicit check that completed means terminal.
 *)
OutputIntegrity ==
    \A f \in completedFiles : files[f] \in TerminalStates

-----------------------------------------------------------------------------
(* Liveness Properties *)

(* Every file eventually reaches a terminal state. *)
AllFilesProcessed ==
    <>(\A f \in FileIds : files[f] \in TerminalStates)

(* No file waits indefinitely -- if a file is in a resting state it will
   eventually leave that state (be picked up by a worker). *)
NoStarvation ==
    \A f \in FileIds :
        \A s \in RestingStates :
            files[f] = s ~> files[f] /= s

(* The pipeline eventually terminates. *)
PipelineTerminates ==
    <>(~pipelineActive)

=============================================================================
