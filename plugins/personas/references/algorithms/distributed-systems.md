# Distributed Systems

Reference for the `algorithmist` persona. Everything here is harder than it looks and already solved in a component you can adopt. The job is to recognize which guarantee you need — ordering, agreement, convergence, exactly-once effects — and pick the component that provides it. Implementing consensus yourself is out of scope.

## When you see this → reach for

| Problem shape | Reach for | Trade-off | Use |
|---|---|---|---|
| Several nodes must agree on one value or leader | consensus (Raft, Paxos), via a service | latency of a quorum round-trip | etcd, ZooKeeper, Consul, or your database's transactions |
| Only one worker should do X at a time | lease-based lock **plus fencing tokens** | a lease alone is unsafe under pauses | etcd or ZooKeeper locks; storage that rejects stale tokens |
| Edits on many replicas or offline clients must merge without conflicts | CRDTs | metadata overhead; merge semantics fixed by the type | Automerge, Yjs (`yrs` in Rust) |
| Order events causally across nodes | Lamport clocks (total order consistent with causality) or vector clocks (detect concurrency) | vector size grows with participants | a few lines; built into many stores |
| Timestamps that respect causality and stay close to wall time | hybrid logical clocks | needs loosely synchronized clocks | used in CockroachDB-style databases |
| Unique IDs without coordination, sortable by time | UUIDv7 (RFC 9562) or Snowflake-style IDs | leaks creation time; not secret | `uuid` crate (v7), database functions |
| Retries must not double-apply | idempotency keys + dedup store | storage and TTL for keys | request-ID tables; "exactly once" = at-least-once + idempotence |
| Update a DB and publish an event atomically | transactional outbox | a relay process to run | an outbox table + a poller or CDC |
| Find which replicas diverged, cheaply | Merkle trees (anti-entropy) | tree upkeep | as in Dynamo-style stores |
| Spread keys over nodes | consistent / rendezvous hashing | — | see [hashing-and-caching.md](./hashing-and-caching.md) |
| Coordinate a multi-service transaction | saga (compensating actions); avoid distributed 2PC | compensations must be designed | workflow engines (Temporal and similar) |

## Reach-for notes

- **Consensus:** Raft (Ongaro and Ousterhout, 2014) was designed to be understandable. Paxos (Lamport) is the classic. Both are notoriously subtle to implement correctly; adopt etcd or ZooKeeper, or lean on a single database's transactions. See [raft.github.io](https://raft.github.io/).
- **Locks with leases** fail when a holder pauses (GC, swap, network) past its lease and then writes. The fix is a monotonically increasing *fencing token* that the storage checks and uses to reject stale writers (Kleppmann, [How to do distributed locking](https://martin.kleppmann.com/2016/02/08/how-to-do-distributed-locking.html)). If the storage can't check tokens, the lock is only an efficiency hint.
- **CRDTs** (Shapiro et al., SSS 2011) converge without coordination: counters, sets, registers, sequences for collaborative text. Pick the type whose merge semantics match what users expect; "last writer wins" silently drops edits.
- **Clocks:** Lamport (CACM 1978) orders events consistently with causality but can't tell you whether two events were concurrent. Vector clocks can. HLCs (Kulkarni et al., OPODIS 2014) add physical time, so timestamps stay meaningful to humans. Never order distributed events by wall clock alone.
- **IDs:** RFC 9562 defines UUIDv7 (time-ordered, index-friendly) and warns that UUIDs must not be used as security capabilities. Use a CSPRNG token for those (see [cryptography.md](./cryptography.md)).

## Classic failure modes

- Treating a lease or TTL lock as mutual exclusion without fencing.
- Wall-clock ordering across machines.
- "Exactly once" delivery claims without idempotent consumers.
- Dual writes (DB, then queue) with no outbox; a crash between them loses or duplicates events.
- Home-grown consensus or leader election.
- Retries without idempotency keys on non-idempotent operations.

**Depth:** Kleppmann, *Designing Data-Intensive Applications*; Lamport, "Time, Clocks, and the Ordering of Events in a Distributed System" (CACM 1978); [raft.github.io](https://raft.github.io/); Shapiro et al., "Conflict-free Replicated Data Types" (SSS 2011).
