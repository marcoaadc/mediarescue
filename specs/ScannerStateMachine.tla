--------------------------- MODULE ScannerStateMachine ---------------------------
(*
 * Formal specification of the MediaRescue scanner state machine.
 *
 * Models the lifecycle of a device scan session: reading sectors from a
 * disk/USB drive, detecting file signatures (JPEG, PNG, MP4, etc.),
 * handling pause/resume, device removal, and analysis of results.
 *
 * This spec must pass TLC with ZERO violations before any Rust
 * implementation is written (spec-driven development, Phase 0).
 *)
EXTENDS Integers, FiniteSets, Naturals

CONSTANT MAX_SECTORS

ASSUME MAX_SECTORS \in Nat /\ MAX_SECTORS > 0

VARIABLES
    scanState,          \* Current state of the scanner
    progress,           \* Number of sectors scanned so far
    totalSectors,       \* Total sectors on the device
    badSectors,         \* Set of sector indices that failed I/O
    signaturesFound,    \* Count of file signatures detected
    deviceConnected     \* Whether the device is still connected

vars == <<scanState, progress, totalSectors, badSectors, signaturesFound, deviceConnected>>

States == {"Idle", "Preparing", "Scanning", "Paused", "Analyzing", "Error", "Completed", "Cancelled"}

SectorRange == 0..MAX_SECTORS

-----------------------------------------------------------------------------
(* Type invariant *)

TypeOK ==
    /\ scanState \in States
    /\ progress \in 0..MAX_SECTORS
    /\ totalSectors \in 1..MAX_SECTORS
    /\ badSectors \subseteq 0..(MAX_SECTORS - 1)
    /\ signaturesFound \in Nat
    /\ signaturesFound <= progress
    /\ deviceConnected \in BOOLEAN

-----------------------------------------------------------------------------
(* Initial state *)

Init ==
    /\ scanState = "Idle"
    /\ progress = 0
    /\ totalSectors \in 1..MAX_SECTORS   \* Nondeterministic: any valid device size
    /\ badSectors = {}
    /\ signaturesFound = 0
    /\ deviceConnected = TRUE

-----------------------------------------------------------------------------
(* Transitions *)

(* Idle -> Preparing: user initiates a scan *)
StartScan ==
    /\ scanState = "Idle"
    /\ deviceConnected = TRUE
    /\ scanState' = "Preparing"
    /\ UNCHANGED <<progress, totalSectors, badSectors, signaturesFound, deviceConnected>>

(* Preparing -> Scanning: device opened and ready *)
DeviceReady ==
    /\ scanState = "Preparing"
    /\ deviceConnected = TRUE
    /\ scanState' = "Scanning"
    /\ UNCHANGED <<progress, totalSectors, badSectors, signaturesFound, deviceConnected>>

(* Preparing -> Error: cannot open device *)
DeviceOpenFailed ==
    /\ scanState = "Preparing"
    /\ scanState' = "Error"
    /\ UNCHANGED <<progress, totalSectors, badSectors, signaturesFound, deviceConnected>>

(* Scanning -> Scanning: read one more sector *)
ScanNextSector ==
    /\ scanState = "Scanning"
    /\ progress < totalSectors
    /\ deviceConnected = TRUE
    /\ progress' = progress + 1
    /\ \/ /\ signaturesFound' = signaturesFound + 1   \* Found a signature
       \/ /\ signaturesFound' = signaturesFound        \* No signature found
    /\ \/ /\ badSectors' = badSectors \union {progress}  \* Sector I/O failed
       \/ /\ badSectors' = badSectors                     \* Sector read OK
    /\ scanState' = "Scanning"
    /\ UNCHANGED <<totalSectors, deviceConnected>>

(* Scanning -> Paused: user pauses *)
PauseScan ==
    /\ scanState = "Scanning"
    /\ scanState' = "Paused"
    /\ UNCHANGED <<progress, totalSectors, badSectors, signaturesFound, deviceConnected>>

(* Scanning -> Analyzing: all sectors scanned *)
ScanComplete ==
    /\ scanState = "Scanning"
    /\ progress = totalSectors
    /\ deviceConnected = TRUE
    /\ scanState' = "Analyzing"
    /\ UNCHANGED <<progress, totalSectors, badSectors, signaturesFound, deviceConnected>>

(* Scanning -> Error: device was removed *)
DeviceDisconnected ==
    /\ scanState = "Scanning"
    /\ deviceConnected = FALSE
    /\ scanState' = "Error"
    /\ UNCHANGED <<progress, totalSectors, badSectors, signaturesFound, deviceConnected>>

(* Scanning -> Cancelled: user cancels *)
CancelFromScanning ==
    /\ scanState = "Scanning"
    /\ scanState' = "Cancelled"
    /\ UNCHANGED <<progress, totalSectors, badSectors, signaturesFound, deviceConnected>>

(* Paused -> Scanning: user resumes *)
ResumeScan ==
    /\ scanState = "Paused"
    /\ deviceConnected = TRUE
    /\ scanState' = "Scanning"
    /\ UNCHANGED <<progress, totalSectors, badSectors, signaturesFound, deviceConnected>>

(* Paused -> Cancelled: user cancels from paused state *)
CancelFromPaused ==
    /\ scanState = "Paused"
    /\ scanState' = "Cancelled"
    /\ UNCHANGED <<progress, totalSectors, badSectors, signaturesFound, deviceConnected>>

(* Analyzing -> Completed: analysis finished successfully *)
AnalysisDone ==
    /\ scanState = "Analyzing"
    /\ scanState' = "Completed"
    /\ UNCHANGED <<progress, totalSectors, badSectors, signaturesFound, deviceConnected>>

(* Analyzing -> Error: analysis failed *)
AnalysisFailed ==
    /\ scanState = "Analyzing"
    /\ scanState' = "Error"
    /\ UNCHANGED <<progress, totalSectors, badSectors, signaturesFound, deviceConnected>>

(* Error | Completed | Cancelled -> Idle: reset *)
Reset ==
    /\ scanState \in {"Error", "Completed", "Cancelled"}
    /\ scanState' = "Idle"
    /\ progress' = 0
    /\ totalSectors' \in 1..MAX_SECTORS   \* Next scan may use a different device
    /\ badSectors' = {}
    /\ signaturesFound' = 0
    /\ deviceConnected' = TRUE

(* Environment action: device can be unplugged at any time during Scanning or Preparing *)
DeviceRemoval ==
    /\ scanState \in {"Scanning", "Preparing", "Paused"}
    /\ deviceConnected = TRUE
    /\ deviceConnected' = FALSE
    /\ UNCHANGED <<scanState, progress, totalSectors, badSectors, signaturesFound>>

-----------------------------------------------------------------------------
(* Next-state relation *)

Next ==
    \/ StartScan
    \/ DeviceReady
    \/ DeviceOpenFailed
    \/ ScanNextSector
    \/ PauseScan
    \/ ScanComplete
    \/ DeviceDisconnected
    \/ CancelFromScanning
    \/ ResumeScan
    \/ CancelFromPaused
    \/ AnalysisDone
    \/ AnalysisFailed
    \/ Reset
    \/ DeviceRemoval

-----------------------------------------------------------------------------
(* Fairness *)

(* Weak fairness on scan actions ensures the scanner keeps making progress
   when enabled, which is necessary for the liveness properties.

   WF on ScanNextSector/ScanComplete ensures scanning progresses to completion.
   WF on DeviceDisconnected ensures device removal is detected.
   WF on AnalysisDone/AnalysisFailed ensures analysis terminates.
   WF on DeviceReady/DeviceOpenFailed ensures preparation terminates.
   WF on ResumeScan ensures paused scans eventually resume when device is connected.
   WF on CancelFromPaused prevents deadlock when device is removed while paused
   (ResumeScan is disabled when deviceConnected=FALSE, so cancel is the only exit).
   WF on Reset enables return to Idle from terminal states.
   SF on ScanNextSector/ScanComplete: strong fairness prevents infinite pause/resume
   loops from starving sector reads — if scanning is enabled infinitely often, it
   must eventually make progress. *)
Fairness ==
    /\ SF_vars(ScanNextSector)
    /\ SF_vars(ScanComplete)
    /\ WF_vars(DeviceDisconnected)
    /\ WF_vars(AnalysisDone)
    /\ WF_vars(AnalysisFailed)
    /\ WF_vars(DeviceReady)
    /\ WF_vars(DeviceOpenFailed)
    /\ WF_vars(ResumeScan)
    /\ WF_vars(CancelFromPaused)
    /\ WF_vars(Reset)

Spec == Init /\ [][Next]_vars /\ Fairness

-----------------------------------------------------------------------------
(* Safety Invariants *)

(* Progress can never exceed totalSectors *)
ProgressBounded ==
    progress <= totalSectors

(* Only valid state transitions can occur — encoded structurally by Next,
   but we additionally verify no unreachable state combinations appear. *)
NoIllegalTransition ==
    \* If we are Idle, progress must be 0 and no bad sectors recorded
    /\ (scanState = "Idle" => (progress = 0 /\ badSectors = {} /\ signaturesFound = 0))
    \* If we are Preparing, no sectors have been scanned yet
    /\ (scanState = "Preparing" => progress = 0)

(* Pausing preserves all scan data — progress, badSectors, signaturesFound
   remain unchanged. This is verified structurally by PauseScan's UNCHANGED
   clause. We add an invariant that in Paused state, the data is consistent. *)
PausedPreservesState ==
    scanState = "Paused" =>
        /\ progress <= totalSectors
        /\ signaturesFound <= progress
        /\ badSectors \subseteq 0..(progress - 1)

(* Bad sectors can only be from sectors we have actually scanned *)
BadSectorsValid ==
    badSectors \subseteq 0..(progress - 1)

-----------------------------------------------------------------------------
(* Liveness Properties *)

(* From Scanning, the scanner eventually reaches a terminal-ish state *)
ScanEventuallyTerminates ==
    scanState = "Scanning" ~> scanState \in {"Completed", "Error", "Cancelled"}

(* If the device is removed while scanning, we eventually reach Error *)
DeviceRemovalHandled ==
    (scanState = "Scanning" /\ deviceConnected = FALSE) ~> scanState = "Error"

=============================================================================
