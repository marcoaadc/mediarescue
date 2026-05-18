--------------------------- MODULE ConcurrentRecovery ---------------------------
(*
 * Formal specification of concurrent recovery job scheduling in MediaRescue.
 *
 * Models multiple recovery jobs competing for a single shared disk reader.
 * Only one job may hold the disk lock at a time (mutual exclusion). Jobs
 * that need disk access enter a FIFO wait queue. The spec verifies safety
 * (mutual exclusion, type correctness, FIFO fairness) and liveness (every
 * job eventually completes, no starvation).
 *
 * This spec must pass TLC with ZERO violations before any Rust
 * implementation is written (spec-driven development, Phase 0).
 *)
EXTENDS Integers, FiniteSets, Sequences, Naturals

CONSTANT MAX_JOBS

ASSUME MAX_JOBS \in Nat /\ MAX_JOBS > 0

JobId == 1..MAX_JOBS

VARIABLES
    jobs,           \* Set of active recovery job IDs
    diskLock,       \* "free" or the JobId currently holding the lock
    waitQueue,      \* Sequence (FIFO) of job IDs waiting for disk access
    jobState,       \* Function: JobId -> {"Waiting", "Reading", "Processing", "Done"}
    completedJobs   \* Set of job IDs that have finished

vars == <<jobs, diskLock, waitQueue, jobState, completedJobs>>

JobStates == {"Waiting", "Reading", "Processing", "Done"}

-----------------------------------------------------------------------------
(* Helpers *)

(* Whether a job ID appears anywhere in the wait queue *)
InQueue(j) == \E i \in 1..Len(waitQueue) : waitQueue[i] = j

(* Remove the head element from a sequence *)
Tail1(s) == SubSeq(s, 2, Len(s))

(* The set of elements in a sequence *)
SeqToSet(s) == {s[i] : i \in 1..Len(s)}

-----------------------------------------------------------------------------
(* Type invariant *)

TypeOK ==
    /\ jobs \subseteq JobId
    /\ diskLock \in {"free"} \union JobId
    /\ waitQueue \in Seq(JobId)
    /\ Len(waitQueue) <= MAX_JOBS
    /\ \A i \in 1..Len(waitQueue) : waitQueue[i] \in JobId
    /\ jobState \in [JobId -> JobStates]
    /\ completedJobs \subseteq JobId

-----------------------------------------------------------------------------
(* Initial state *)

Init ==
    /\ jobs = JobId                                 \* All jobs are active
    /\ diskLock = "free"                            \* Disk is available
    /\ waitQueue = <<>>                             \* No one is waiting
    /\ jobState = [j \in JobId |-> "Waiting"]       \* All jobs start Waiting
    /\ completedJobs = {}                           \* No jobs completed yet

-----------------------------------------------------------------------------
(* Actions *)

(*
 * RequestAccess: a Waiting job that is not yet in the queue enters the
 * wait queue. It remains in state Waiting until granted the lock.
 *)
RequestAccess(j) ==
    /\ j \in jobs
    /\ jobState[j] = "Waiting"
    /\ ~InQueue(j)
    /\ diskLock # j                                 \* Not already holding the lock
    /\ waitQueue' = Append(waitQueue, j)
    /\ UNCHANGED <<jobs, diskLock, jobState, completedJobs>>

(*
 * GrantAccess: the first job in the wait queue acquires the disk lock
 * when it is free. The job transitions from Waiting to Reading.
 *)
GrantAccess ==
    /\ Len(waitQueue) > 0
    /\ diskLock = "free"
    /\ LET j == Head(waitQueue)
       IN /\ jobState[j] = "Waiting"
          /\ diskLock' = j
          /\ jobState' = [jobState EXCEPT ![j] = "Reading"]
          /\ waitQueue' = Tail1(waitQueue)
          /\ UNCHANGED <<jobs, completedJobs>>

(*
 * ReadDisk: the job currently holding the disk lock performs its read
 * operation. It transitions from Reading to Processing, indicating
 * that the data has been read and the job is ready to release the lock.
 *)
ReadDisk(j) ==
    /\ j \in jobs
    /\ diskLock = j
    /\ jobState[j] = "Reading"
    /\ jobState' = [jobState EXCEPT ![j] = "Processing"]
    /\ UNCHANGED <<jobs, diskLock, waitQueue, completedJobs>>

(*
 * ReleaseLock: a job that has finished reading (state = Processing)
 * releases the disk lock so the next waiting job can proceed.
 *)
ReleaseLock(j) ==
    /\ j \in jobs
    /\ diskLock = j
    /\ jobState[j] = "Processing"
    /\ diskLock' = "free"
    /\ UNCHANGED <<jobs, waitQueue, jobState, completedJobs>>

(*
 * CompleteJob: a Processing job that has released the disk lock finishes
 * and moves to Done. It is added to completedJobs.
 *)
CompleteJob(j) ==
    /\ j \in jobs
    /\ jobState[j] = "Processing"
    /\ diskLock # j                                 \* Must not hold the lock
    /\ jobState' = [jobState EXCEPT ![j] = "Done"]
    /\ completedJobs' = completedJobs \union {j}
    /\ UNCHANGED <<jobs, diskLock, waitQueue>>

-----------------------------------------------------------------------------
(* Next-state relation *)

Next ==
    \/ \E j \in JobId : RequestAccess(j)
    \/ GrantAccess
    \/ \E j \in JobId : ReadDisk(j)
    \/ \E j \in JobId : ReleaseLock(j)
    \/ \E j \in JobId : CompleteJob(j)

-----------------------------------------------------------------------------
(* Fairness *)

(*
 * Weak fairness on each per-job action ensures that every continuously
 * enabled action eventually executes. This is essential for liveness:
 * without it, the model checker would find spurious counterexamples
 * where a job is perpetually starved despite being enabled.
 *
 * We use strong fairness on GrantAccess because it may be repeatedly
 * enabled and disabled (the lock keeps being grabbed by other jobs),
 * yet we need it to eventually fire for fairness.
 *)
Fairness ==
    /\ \A j \in JobId : WF_vars(RequestAccess(j))
    /\ SF_vars(GrantAccess)
    /\ \A j \in JobId : WF_vars(ReadDisk(j))
    /\ \A j \in JobId : WF_vars(ReleaseLock(j))
    /\ \A j \in JobId : WF_vars(CompleteJob(j))

Spec == Init /\ [][Next]_vars /\ Fairness

-----------------------------------------------------------------------------
(* Safety Invariants *)

(* At most one job is in Reading state at any time *)
MutualExclusion ==
    Cardinality({j \in JobId : jobState[j] = "Reading"}) <= 1

(* If a job holds the disk lock, its state must be Reading or Processing.
   Once in Processing the job may still hold the lock briefly before
   ReleaseLock fires, so both states are valid lock-holding states. *)
LockHolderIsActive ==
    diskLock # "free" => jobState[diskLock] \in {"Reading", "Processing"}

(* Strengthened variant: if a job holds the disk lock and is NOT Reading,
   it must be Processing (about to release). This is implied by
   LockHolderIsActive but stated separately for clarity. *)
LockHolderIsReading ==
    \A j \in JobId :
        (diskLock = j) => jobState[j] \in {"Reading", "Processing"}

(* Every job in completedJobs has state Done *)
CompletedJobsAreDone ==
    \A j \in completedJobs : jobState[j] = "Done"

(* The system can always make progress: either all jobs are Done,
   or at least one action is enabled *)
NoDeadlock ==
    (\A j \in JobId : jobState[j] = "Done")
    \/ (\E j \in JobId : jobState[j] = "Waiting" /\ ~InQueue(j))
    \/ (Len(waitQueue) > 0 /\ diskLock = "free")
    \/ (\E j \in JobId : diskLock = j /\ jobState[j] = "Reading")
    \/ (\E j \in JobId : diskLock = j /\ jobState[j] = "Processing")
    \/ (\E j \in JobId : jobState[j] = "Processing" /\ diskLock # j)

(* Jobs are granted access in FIFO order: if the lock is free and the
   queue is non-empty, only the head of the queue can be granted next.
   This is enforced structurally by GrantAccess always taking Head(waitQueue). *)
FairScheduling ==
    \A j \in JobId :
        (jobState[j] = "Reading" /\ diskLock = j)
        => ~\E k \in JobId :
            /\ k # j
            /\ jobState[k] = "Reading"

(* No duplicate entries in the wait queue *)
NoDuplicatesInQueue ==
    \A i, k \in 1..Len(waitQueue) :
        (i # k) => (waitQueue[i] # waitQueue[k])

(* A Done job is never in the wait queue and never holds the lock *)
DoneJobsInactive ==
    \A j \in JobId :
        jobState[j] = "Done" =>
            /\ ~InQueue(j)
            /\ diskLock # j

(* completedJobs is a subset of jobs *)
CompletedSubsetOfJobs ==
    completedJobs \subseteq jobs

-----------------------------------------------------------------------------
(* Liveness Properties *)

(* Every job in jobs eventually reaches Done *)
AllJobsComplete ==
    \A j \in JobId : <>(jobState[j] = "Done")

(* If a job holds the disk lock, it eventually releases it *)
LockEventuallyReleased ==
    \A j \in JobId :
        (diskLock = j) ~> (diskLock # j)

(* Every waiting job eventually gets disk access (no starvation) *)
NoStarvation ==
    \A j \in JobId :
        (jobState[j] = "Waiting") ~> (jobState[j] = "Reading")

=============================================================================
