# Architectural Audit Prompt: Designing Data-Intensive Applications (DDIA)

Conduct a systematic, phase-by-phase data engineering and distributed systems audit.

---

## CONTEXT & OBJECTIVE
You are an expert Distributed Systems Architect and Principal Database Engineer. Your task is to audit our codebase against the foundational data-engineering principles of Martin Kleppmann's *Designing Data-Intensive Applications*. 

Your goal is to inspect the ENTIRE codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., missing failure modes, lack of transactional boundaries, unhandled edge cases) or overlaps (e.g., redundant synchronization loops, duplicate state stores, conflicting roles/responsibilities)?
3. **Actionable Recommendations:** How should we complete this Phase to optimize the codebase, ensure zero overlap, and make sure roles and responsibilities are perfectly partitioned?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `Designing-Data-Intensive-Applications.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/Designing-Data-Intensive-Applications.pdf`

Ensure you load this file into your context (using your PDF rendering/reading tool) and analyze it end-to-end. Do not rely solely on the summaries provided in this prompt. Complete, deep understanding is critical.

---

---

## MANDATORY AUDIT SCOPE: CORE PRODUCT ONLY
To keep your audit focused on high-signal production systems and prevent wasting time or context tokens on non-production simulators or loosely-coupled marketing/CRM tools, you MUST adhere to the following scope boundaries:

*   **IN SCOPE (Audit these core production directories and domain crates):**
    *   `apps/engine`
    *   `apps/api`
    *   `apps/app`
    *   `apps/adapters`
    *   `apps/scheduler`
    *   `crates/` (the core domain)

*   **OUT OF SCOPE (Prune and ignore these directories immediately from your search and context):**
    *   `sim-*` (including `sim-runner` or other non-production simulators)
    *   `crm` (Customer Relationship Management)
    *   `meet` (meeting/scheduler tools)
    *   `posthog` (telemetry/analytics integration)
    *   `sign` (signing tools)
    *   `website` (marketing static sites)
    *   `auto` (automation scripts)
    *   `cal` (calendar integrations)

Do not search, review, or analyze files outside the designated production scope. Focus 100% of your structural analyses on these core application components.

## THE METHODOLOGY: DIAGNOSTIC FACT-GATHERING & RESEARCH EXPEDITION
This audit is strictly structured as a **non-destructive, diagnostic fact-gathering and research expedition checklist**. Your primary objective is to discover, map, and document the actual current state of the codebase *before* proposing any plans or writing code.

For every single Phase in the roadmap below, you must systematically search the codebase and classify our current status into one of three distinct categories:
1.  **[COMPLETED]:** The concept/rule is fully implemented, operational, and tested. *You must cite the exact directories, file paths, classes, or lines of code as empirical proof.*
2.  **[PARTIAL]:** The concept/rule is partially implemented, but contains gaps, unhandled edge cases, or missing boundaries. *You must cite the existing partial code and detail exactly what remains incomplete.*
3.  **[NOT STARTED]:** The concept/rule is completely absent from the current codebase.

You must compile this exhaustive coverage audit across all phases *prior* to formulating any Critical Adjustment Plans or writing CI/CD enforcement scripts. This prevents premature modifications and honors our existing structural investments.

---

## MANDATORY SYSTEMIC COVERAGE: NO SHALLOW VERIFICATION ("CHECKING THE BOX")
To prevent the common analytical error of shallow verification, you MUST adhere to the following exhaustive, system-wide coverage and template rules:

1.  **No Shallow Verification:** Finding a single working example, class, or module that correctly implements a concept (such as a single event stream, state update, or validation check) does NOT mean the codebase has "covered" that phase. You must inspect the *entire* in-scope codebase to determine if the pattern is applied systemically, or if there are major gaps, omissions, and competing inconsistent patterns elsewhere.
2.  **The "Golden Blueprint" Principle:**
    *   If you discover a correct, highly robust implementation of a pattern (e.g., a perfect GTT state update, a clean singleton database connector, or a robust schema-enforced event handler), declare that specific implementation as the **"Golden Blueprint"** for that pattern.
    *   You must then audit the rest of the codebase to identify every location where this pattern *should* be applied but is completely missing, or where divergent, inferior structures are used instead (creating overlaps, vulnerabilities, and maintenance friction).
    *   Use this "Golden Blueprint" as the exact template for your **Actionable Recommendations** and your **Automated CI/CD Enforcement Script** to enforce its systemic adoption across all core repositories.
3.  **Full, Agnostic Event-Stream Auditing:**
    *   When auditing event streams, do not satisfy yourself with pointing to a single queue example. You must audit the *full, agnostic implementation of event streams* across their entire data lifecycle: ingestion buffers, deterministic execution ordering, retry/backpressure enforcements, schema validation at every consumer hop, and state synchronization.

## AUDIT ROADMAP (PHASE-BY-PHASE)

### Phase 1: Reliable, Scalable and Maintainable Applications
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter, we have explored some fundamental ways of thinking about data-
intensive applications. These principles will guide us through the rest of the book,
when we dive into deep technical detail.
An application has to meet various requirements in order to be useful. There are
functional requirements  (what it should do, e.g. allow data to be stored, retrieved,
searched and processed in various ways), and some non-functional requirements
(general properties like security, reliability, compliance, scalability, compatibility and
maintainability). In this chapter we discussed reliability, scalability and maintainabil‐
ity in detail.
Reliability means making systems work correctly, even when faults occur. Faults can
be in hardware (typically random and uncorrelated), software (bugs are typically sys‐
tematic and hard to deal with), and humans (who inevitably make mistakes from
time to time). Fault tolerance techniques can hide certain types of fault from the end
user.
Scalability means having strategies for keeping performance good, even when load
increases. In order to discuss scalability, we first need ways of describing load and
performance quantitatively. We briefly looked at Twitter’s home timelines as an
example of describing load, and response time percentiles as a way of measuring per‐
formance. In a scalable system, you can add processing capacity in order to remain
reliable under high load.
Maintainability has many facets, but in essence it’s about making life better for the
engineering and operations teams who need to work with the system. Good abstrac‐
tions can help reduce complexity and make the system easier to modify and adapt for
new use cases. Good operability means having good visibility into the system’s health,
and having effective ways of managing it.
There is unfortunately no quick answer to making applications reliable, scalable or
maintainable. However, there are certain patterns and techniques which keep re-
appearing in various different kinds of application. In the next few chapters we will
take a look at some examples of data systems, and analyze how they work towards
those goals.
Later in the book, in Part III, we will look at patterns for systems that consist of sev‐
eral components working together, such as the one in Figure 1-1.
```
*   *Audit Focus:* Compare the codebase's architecture directly to the book summary in the text block above. Identify current coverage, gaps, overlaps, and optimization routes.

### Phase 2: Data Models and Query Languages
*   *Chapter-Specific Concluding Summary:*
```text
Summary
Data models are a huge subject, and in this chapter we have taken a quick look at a
broad variety of different models. We didn’t have space to go into all the details of
each model, but hopefully the overview has been enough to whet your appetite to
find out more about the model that best fits your application’s requirements.
Historically, data started out being represented as one big tree (the hierarchical
model), but that wasn’t good for representing many-to-many relationships, so the
relational model was invented to solve that problem. More recently, developers found
that some applications don’t fit well in the relational model either. New non-
relational “NoSQL” data stores have diverged in two main directions:
1. Document databases target use cases where data comes in self-contained docu‐
ments, and relationships between one document and another are rare.
2. Graph databases go in the opposite direction, targeting use cases where anything
is potentially related to everything.
All three models (document, relational and graph) are widely used today, and each is
good in its respective domain. One model can be emulated in terms of another
model, but the result is often awkward. That’s why we have different systems for dif‐
ferent purposes, not a single one-size-fits-all solution.
One thing that document and graph databases have in common is that they typically
don’t enforce a schema for the data they store, which can make it easier to adapt
applications to changing requirements.
Each data model comes with its own query language or framework, and we discussed
several examples: SQL, MapReduce, MongoDB’s aggregation pipeline, Cypher,
SPARQL and Datalog. We also touched on CSS and XSL/XPath, which aren’t data‐
base query languages, but have interesting parallels.
Although we have covered a lot of ground, there are still many data models left
unmentioned. To give just a few brief examples:
• Researchers working with genome data often need to perform sequence-
similarity searches , which means taking one very long string (representing a
DNA molecule) and matching it against a large database of strings that are simi‐
lar, but not identical. None of the databases described above can handle this kind
of usage, which is why researchers have written specialized genome database
software like GenBank [46].
• Particle physicists have been doing Big Data -style large scale data analysis for
decades, and projects like the Large Hadron Collider (LHC) now work with hun‐
dreds of petabytes! At such scale, custom solutions are required, to stop the hard‐
ware cost spiraling out of control [47].
• Full-text search is arguably a kind of data model that is frequently used alongside
databases. Information Retrieval is a large specialist subject that we won’t cover
in great detail in this book, but we’ll touch on search indexes in Chapter 3 and
Part III.
We have to leave it there for now. In the next chapter we will discuss some of the
trade-offs that come into play when implementing the data models described in this
chapter.
```
*   *Audit Focus:* Compare the codebase's architecture directly to the book summary in the text block above. Identify current coverage, gaps, overlaps, and optimization routes.

### Phase 3: Storage and Retrieval
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter we tried to get to the bottom of the questions of data storage and
retrieval. What happens when you store some data in a database, and what does the
database do when you query for the data again later?
On a high level, we saw that storage engines fall into two broad categories: those opti‐
mized for transaction processing (OLTP), and those optimized for analytics. There
are big differences between the access patterns in those use cases:
• OLTP systems are typically user-facing, which means that they may see a huge
volume of requests. In order to handle the load, applications usually only touch a
small number of records in each query. The application requests records using
some kind of key, and the storage engine uses an index to find the data for the
requested key. Disk seek time is often the bottleneck here.
• Data warehouses and similar analytic systems are less well-known, because they
are primarily used by business analysts, not by end users. They handle a much
lower volume of queries than OLTP systems, but each query is typically very
demanding, requiring many millions of records to be scanned in a short time.
Disk bandwidth (not seek time) is often the bottleneck here, and column-
oriented storage is an increasingly popular solution for this kind of workload.
On the OLTP side, we saw storage engines from two main schools of thought:
• The log-structured school, which only permits appending to files and deleting
obsolete files, but never updates a file that has been written. Bitcask, SSTables,
LSM-Trees, LevelDB, Cassandra, HBase, Lucene and others belong to this group.
• The update-in-place school, which treats the disk as fixed-size pages which can
be overwritten. B-trees are the biggest example of this philosophy, being used in
all major relational databases and also many non-relational ones.
Log-structured storage engines are a comparatively recent development. Their key
idea is that they systematically turn random-access writes into sequential writes on
disk, which enables higher write throughput due to the performance characteristics
of hard drives and SSDs.
Finishing off the OLTP side, we did a brief tour through some more complicated
indexing structures, and databases that are optimized for keeping all data in memory.
We then took a detour from the internals of storage engines, to look at the high-level
architecture of a typical data warehouse. This background illustrated why analytic
workloads are so different from OLTP: when your queries require sequentially scan‐
ning across a large number of rows, indexes are much less relevant. Instead it
becomes important to encode data very compactly, to minimize the amount of data
that the query needs to read from disk. We discussed how column-oriented storage
helps achieve this goal.
As an application developer, if you’re armed with this knowledge about the internals
of storage engines, you are in a much better position to know which tool is best suited
for your particular application. If you need to adjust a database’s tuning parameters,
this understanding allows you to imagine what effect a higher or a lower value may
have.
Although this chapter couldn’t make you an expert in tuning any one particular stor‐
age engine, it has hopefully equipped you with enough vocabulary and ideas that you
can make sense of the documentation for the database of your choice.
```
*   *Audit Focus:* Compare the codebase's architecture directly to the book summary in the text block above. Identify current coverage, gaps, overlaps, and optimization routes.

### Phase 4: Encoding and Evolution
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter we looked at several ways of turning data structures into bytes on the
network or bytes on disk. We saw how the details of these encodings affects not only
their efficiency, but more importantly also the architecture of applications and your
options for deploying them.
In particular, many services need to support rolling upgrades, where a new version of
a service is gradually deployed to a few nodes at a time, rather than deploying to all
nodes simultaneously. Rolling upgrades allow new versions of a service to be released
without downtime (thus, encouraging frequent small releases over rare big releases),
and make deployments less risky (allowing faulty releases to be detected and rolled
back before they affect a large number of users). These properties are hugely benefi‐
tial for evolvability, the ease of making changes to an application.
During rolling upgrades, or for various other reasons, we must assume that different
nodes are running the different versions of our application’s code. Thus, it is impor‐
tant that all data flowing around the system is encoded in a way which provides back‐
ward compatibility (new code can read old data) and forward compatibility (old code
can read new data).
We discussed several data encoding formats and discussed their compatibility prop‐
erties:
• Programming-language-specific encodings are restricted to a single program‐
ming language, and often fail to provide forward and backward compatibility.
• Textual formats like JSON, XML and CSV are widespread, and their compatibil‐
ity depends on how you use them. They have optional schema languages, which
are sometimes helpful, and sometimes a hindrance. These formats are somewhat
vague about datatypes, so you have to be careful with things like numbers and
binary strings.
• Binary schema-driven formats like Thrift, Protocol Buffers and Avro allow com‐
pact, efficient encoding with clearly defined forward and backward compatibility
semantics. The schema can be useful for documentation and code generation in
statically typed languages. However, they have the downside that data needs to be
decoded before it is human-readable.
Summary | 135
We also discussed several modes of data flow, illustrating different scenarios in which
data encodings are important:
• Databases, where the process writing to the database encodes the data, and the
process reading from the database decodes it.
• RPC and REST APIs, where the client encodes a request, the server decodes the
request and encodes a response, and the client finally decodes the response.
• Asynchronous message-passing (using message brokers or actors), where nodes
communicate by sending each other messages that are encoded by the sender
and decoded by the recipient.
We can conclude that with a bit of care, backward/forward compatibility and rolling
upgrades are quite achievable. May your application’s evolution be rapid and your
deployments be frequent.
```
*   *Audit Focus:* Compare the codebase's architecture directly to the book summary in the text block above. Identify current coverage, gaps, overlaps, and optimization routes.

### Phase 5: Replication
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter, we looked at the issue of replication, that is: keeping a copy of the
same data on several machines. Replication can serve several purposes:
• High availability: keeping the system running, even when one machine (or sev‐
eral machines, or an entire datacenter) goes down.
• Disconnected operation: allowing an application to continue working when there
is a network interruption.
• Latency: placing data geographically close to users, so that users can interact with
it faster.
• Scalability: being able to handle a higher volume of reads than a single machine
could handle, by performing reads on replicas.
Despite being a simple goal — a copy of the same data on several machines — replica‐
tion turns out to be a remarkably tricky problem. It requires carefully thinking about
concurrency and about all the things that can go wrong, and dealing with the conse‐
quences of those faults. At a minimum, we need to deal with unavailable nodes and
network interruptions (and that’s not even considering the more insidious kinds of
fault, such as silent data corruption due to software bugs).
We discussed three main approaches to replication:
1. Single-leader replication : Clients send all writes to a single node (the leader),
which sends a stream of data change events to the other replicas (followers).
Reads can be performed on any replica, but reads from followers might be stale.
2. Multi-leader replication: Clients send each write to one of several leader nodes,
any of which can accept writes. The leaders send streams of data change events to
each other, and to any follower nodes.
3. Leaderless replication : Clients send each write to several nodes, and read from
several nodes in parallel in order to detect and correct nodes with stale data.
Each has advantages and disadvantages. Single-leader replication is popular because
it is fairly easy to understand and there is no conflict resolution to worry about.
Multi-leader and leaderless replication can be more robust in the presence of faulty
nodes, network interruptions and latency spikes  — at the cost of being harder to rea‐
son about, and providing only very weak consistency guarantees.
Replication can be synchronous or asynchronous, which has a profound effect on the
system behavior when there is a fault. Although asynchronous replication can be fast
when the system is running smoothly, it’s important to figure out what happens
when replication lag increases and servers fail. If a leader fails, and you promote an
asynchronously updated follower to be the new leader, recently committed data may
be lost.
We looked at some strange effects that can be caused by replication lag, and we dis‐
cussed a few consistency models which are helpful for deciding how an application
should behave under replication lag:
• Read-after-write consistency : a user should always see data that they submitted
themselves.
• Monotonic reads : after a user has seen the data at one point in time, they
shouldn’t later see the data from some earlier point in time.
• Consistent prefix reads : users should see the data in a state that makes causal
sense, for example seeing a question and its reply in the correct order.
Finally, we discussed the concurrency issues that are inherent in multi-leader and
leaderless replication approaches: because they allow multiple writes to happen con‐
currently, conflicts may occur. We examined an algorithm that a database might use
to determine whether one operation happened before another, or whether they hap‐
pened concurrently. We also touched on methods for resolving conflicts by merging
together concurrent updates.
In the next chapter we will continue looking at data that is distributed across multiple
machines, through the counterpart of replication: splitting a large dataset into parti‐
tions.
```
*   *Audit Focus:* Compare the codebase's architecture directly to the book summary in the text block above. Identify current coverage, gaps, overlaps, and optimization routes.

### Phase 6: Partitioning
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter we explored different ways of partitioning a large dataset into smaller
subsets. This is necessary when you have so much data that storing and processing it
on a single machine is no longer feasible.
The main goal of partitioning is to spread the data and the query load evenly across
multiple machines, avoiding hot spots (nodes with disproportionately high load).
This requires choosing a partitioning scheme that is appropriate to your data, and
rebalancing the partitions from time to time as nodes are added or removed from the
cluster.
We discussed two main approaches to partitioning:
• Key range partitioning , where keys are sorted, and a partition owns all the keys
from some minimum up to some maximum. This has the advantage that effi‐
cient range queries are possible, but there is a risk of hot spots if the application
often accesses keys that are close together in the sorted order.
In this approach, partitions are typically rebalanced dynamically, by splitting the
range into two sub-ranges when a partition gets too big.
• Hash partitioning, where a hash function is applied to each key, and a partition
owns a range of hashes. This destroys the ordering of keys, making range queries
inefficient, but may distribute load more evenly.
When partitioning by hash, it is common to create a fixed number of partitions
in advance, to assign several partitions to each node, and to move entire parti‐
tions from one node to another when nodes are added or removed.
Hybrid approaches are also possible, for example with a compound key: using one
part of the key to identify the partition, and another part for the sort order.
We also discussed the interaction between partitioning and secondary indexes. A sec‐
ondary index also needs to be partitioned, and there are two possibilities for doing
this:
• Document-partitioned index: the secondary indexes are stored in the same parti‐
tion as the primary key and value. This means that only a single partition needs
to be updated on write, but a read of the secondary index requires a scatter/
gather across all partitions.
• Term-partitioned index (global index): the secondary indexes are partitioned sep‐
arately, using the indexed values. An entry in the secondary index may include
records from all partitions of the primary key. When a document is written, sev‐
eral partitions of the secondary index need to be updated; however, a read can be
served from a single partition.
Finally, we discussed techniques for routing queries to the appropriate partition,
which range from simple partition-aware load balancing to sophisticated parallel
query execution engines.
By design, every partition operates mostly independently  — that’s what allows a par‐
titioned database to scale to multiple machines. However, operations that need to
write to several partitions can be difficult to reason about: for example, what happens
if the write to one partition succeeds, but another fails? In the next chapter we will
turn to the topic of transactions, which can make multi-write operations easier to rea‐
son about.
```
*   *Audit Focus:* Compare the codebase's architecture directly to the book summary in the text block above. Identify current coverage, gaps, overlaps, and optimization routes.

### Phase 7: Transactions
*   *Chapter-Specific Concluding Summary:*
```text
Summary
Transactions are an abstraction layer that allow an application to pretend that certain
concurrency problems and certain kinds of hardware and software fault don’t exist. A
large class of errors is reduced down to a simple transaction abort, and the applica‐
tion just needs to try again.
In this chapter we saw many examples of problems that transactions help prevent.
Not all applications are susceptible to all those problems: an application with very
simple access patterns, such as reading and writing only a single record, can probably
manage without transactions. However, for more complex access patterns, transac‐
tions can hugely reduce the number of potential error cases you need to think about.
Many NoSQL systems abandoned transactions in the name of scalability, availability
and performance. Unfortunately this means that applications using such data systems
either need to implement their own transaction management  — which is unlikely,
because it’s hard to implement correctly — or accept that their data is approximate.
Without transactions, various error scenarios (processes crashing, network interrup‐
tions, power outage, disk full, unexpected concurrency, etc) mean that data can
become inconsistent in various ways. For example, denormalized data can easily go
out of sync with the source data. Without transactions, it becomes very difficult to
reason about the effect that complex interacting accesses can have on the database.
In this chapter, we went particularly deep into the topic of concurrency control. We
discussed several widely-used isolation levels, in particular read committed, snapshot
Summary | 257
isolation (sometimes called repeatable read) and serializable. We characterised those
isolation levels by discussing various examples of race conditions:
Dirty reads
One client reads another client’s writes before they have been committed. The
read committed isolation level and stronger levels prevent dirty reads.
Dirty writes
One client overwrites data that another client has written, but not yet committed.
Almost all transaction implementations prevent dirty writes.
Read skew (non-repeatable reads)
A client sees different parts of the database at different points in time. This is
most commonly prevented with snapshot isolation, which allows a transaction to
read from a consistent snapshot at one point in time. It is usually implemented
with multi-version concurrency control (MVCC).
Lost updates
Two clients concurrently perform a read-modify-write cycle. One overwrites the
other’s write without incorporating its changes, so data is lost. Some implemen‐
tations of snapshot isolation prevent this.
Write skew
A transaction reads something, makes a decision based on the value it saw, and
writes the decision to the database. However, by the time the write is made, the
premise of the decision is no longer true. Only serializable isolation prevents this.
Phantom reads
A transaction reads objects that match some search condition. Another client
makes a write that affects the results of that search. Snapshot isolation prevents
straightforward phantom reads, but phantoms in the context of write skew
require special treatment, such as index-range locks.
Weak isolation levels protect against some of those anomalies, and leave you, the
application developer, to handle others manually (e.g. using explicit locking). Only
serializable isolation protects against all of these issues. We discussed three different
approaches to implementing serializable transactions:
1. Literally executing transactions in a serial order. If you can make each transac‐
tion very fast to execute, and the transaction throughput is low enough to process
on a single CPU core, this is a simple and effective option.
2. Two-phase locking. For decades this has been the standard way of implementing
serializability, but many applications avoid using it because of its performance
characteristics.
3. Serializable snapshot isolation, a fairly new algorithm that avoids most of the
downsides of the previous approaches. It uses an optimistic approach, allowing
transactions to proceed without blocking. When a transaction wants to commit,
it is checked, and aborted if the execution was not serializable.
The examples in this chapter used a relational data model. However, as discussed in
“The need for multi-object transactions” on page 222, transactions are a valuable
database feature, no matter which data model is used.
Most of the ideas and algorithms in this chapter apply no matter whether the data‐
base is running on a single machine, or replicated and partitioned across multiple
machines. However, there is an additional set of difficult challenges that arises if you
try to implement transactions in distributed databases. We’ll discuss those in the next
two chapters.
```
*   *Audit Focus:* Compare the codebase's architecture directly to the book summary in the text block above. Identify current coverage, gaps, overlaps, and optimization routes.

### Phase 8: The Trouble with Distributed Systems
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter we have discussed a wide range of problems that can occur in dis‐
tributed systems, including:
• Whenever you try to send a packet over the network, it may be lost or arbitrarily
delayed. Likewise, the reply may be lost or delayed, so if you don’t get a reply,
you have no idea whether the message got through.
• A node’s clock may be significantly out of sync with other nodes (despite your
best efforts to set up NTP), it may suddenly jump forward or back in time, and
relying on it is dangerous because you most likely don’t have a good measure of
your clock’s error interval.
• A process may pause for a substantial amount of time at any point in its execu‐
tion (perhaps due to a stop-the-world garbage collector), be declared dead by
other nodes, and then come back to life again without realizing that it was
paused.
All of these can be summarized as partial failures. Whenever software tries to do any‐
thing involving other nodes, there is the possibility that it may occasionally fail, or
randomly go slow, or not respond at all (and eventually time out). In distributed sys‐
tems, we try to build tolerance of partial failures into software, so that the system as a
whole may continue functioning, even when some of its constituent parts are broken.
To tolerate faults, the first step is to detect the fault, but even that is hard. Most sys‐
tems don’t have an accurate mechanism of detecting whether a node has failed, so
most distributed algorithms rely on timeouts to determine whether a remote node is
still available. However, timeouts can’t distinguish between network and node fail‐
ures, and variable network delay sometimes causes a node to be falsely suspected of
crashing. Moreover, sometimes a node can be in a degraded state: for example, a Gig‐
abit network interface could suddenly drop to 1 kilobit/s throughput due to a driver
bug [90]. Such a system that is ‘limping’, but not dead, can be even more difficult to
deal with than a cleanly failed node.
Once a fault is detected, making a system tolerate it is not easy either: there is no
global variable, no shared memory, no common knowledge or any other kind of
shared state between the machines. Nodes can’t even agree what time it is, let alone
anything more profound. The only way how information can flow from one node to
another is by sending it over the unreliable network. Major decisions cannot be safely
made by a single node, so we require protocols that enlist the help from other nodes
and try to get a majority quorum to agree.
If you’re used to writing software in the idealized mathematical perfection of a single
computer, where the same operation always deterministically returns the same result,
then moving to the messy physical reality of distributed systems can be a bit of a
shock. Conversely, distributed systems engineers will often regard a problem as triv‐
ial if it can be solved on a single computer [ 5], and indeed a single computer can do a
lot nowadays [ 91]. If you can avoid opening Pandora’s box, and simply keep things
on a single machine, it is generally worth doing so.
However, as discussed in the introduction to Part II, scalability is not the only reason
for wanting to use a distributed system. Fault tolerance and low latency (by placing
data geographically close to users) are equally important goals, and those things can‐
not be achieved with a single node.
In this chapter we also went on some tangents to explore whether the unreliability of
networks, clocks and processes is an inevitable law of nature. We saw that it isn’t: it is
possible to give hard real-time response guarantees and bounded delay in networks,
but doing so is very expensive and results in lower utilization of hardware resources.
Most non-safety-critical systems choose cheap and unreliable over expensive and
reliable.
We also touched on supercomputers, which assume reliable components, and thus
have to be stopped and restarted entirely when a component does fail. By contrast,
distributed systems can run forever without being interrupted at the service level,
because all faults and maintenance can be handled at the node level  — at least in
theory. (In practice, if a bad configuration change is rolled out to all nodes, that will
still bring a distributed system to its knees.)
This chapter has been all about problems, and has given us a bleak outlook. In the
next chapter we will move on to solutions, and discuss some algorithms that have
been designed to cope with all the problems in distributed systems.
```
*   *Audit Focus:* Compare the codebase's architecture directly to the book summary in the text block above. Identify current coverage, gaps, overlaps, and optimization routes.

### Phase 9: Consistency and Consensus
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter we examined the topics of consistency and consensus from several dif‐
ferent angles. We looked in depth at linearizability, a popular consistency model: its
goal is to make replicated data appear as though there was only a single copy, and to
make all operations act on it atomically. Although linearizability is appealing because
it is easy to understand  — it makes a database behave like a variable in a single-
threaded program  — it has the downside of being slow, especially in environments
with large network delays.
We also explored causality, which imposes an ordering on events in a system (what
happened before what, based on cause and effect). Unlike linearizability, which puts
all operations in a single, totally ordered timeline, causality provides us with a weaker
consistency model: some things can be concurrent, so the version history is like a
timeline with branching and merging. Causal consistency does not have the coordi‐
nation overhead of linearizability, and is much less sensitive to network problems.
However, even if we capture the causal ordering (for example using Lamport time‐
stamps), we saw that some things cannot be implemented this way: in “Timestamp
ordering is not sufficient” on page 337 we considered the example of ensuring that a
username is unique, and rejecting concurrent registrations for the same username. If
one node is going to accept a registration, it needs to somehow know that another
Summary | 363
node isn’t concurrently in the process of registering the same name. This problem led
us towards consensus.
We saw that achieving consensus means to decide something in a way that all nodes
agree what was decided, and such that the decision is irrevocable. With some digging,
it turns out that a wide range of problems are actually reducible to consensus, and
equivalent to each other (in the sense that if you have a solution for one of them, you
can easily transform it into a solution for one of the others):
Linearizable compare-and-set register
The register needs to atomically decide whether to set its value, based on whether
its current value equals the parameter given in the operation.
Atomic transaction commit
A database must decide whether to commit or abort a distributed transaction.
Total order broadcast
The messaging system must decide on the order in which to deliver messages.
Locks and leases
When several clients are racing to grab a lock or lease, the lock decides which one
successfully acquired it.
Membership/coordination service
Given a failure detector (e.g. timeouts), the system must decide which nodes are
alive, and which should be considered dead because their session timed out.
Uniqueness constraint
When several transactions concurrently try to create conflicting records with the
same key, the constraint must decide which one to allow and which should fail
with a constraint violation.
All of these are straightforward if you only have a single node, or if you are willing to
assign the decision-making capability to a single node. This is what happens in a
single-leader database: all the power to make decisions is vested in the leader, which
is why such databases are able to provide linearizable operations, uniqueness con‐
straints, a totally ordered replication log, and more.
However, if that single leader fails, or if a network interruption makes the leader
unreachable, such a system becomes unable to make any progress. There are three
ways of handling that situation:
1. Wait for the leader to recover, and accept that the system will be blocked in the
meantime. Many XA/JTA transaction coordinators choose this option. This
approach does not solve consensus because it does not satisfy the termination
property: if the leader does not recover, the system can be blocked forever.
2. Manually failover by having humans choose a new leader node, and reconfigur‐
ing the system to use it. Many relational databases take this approach. It is a kind
of consensus by “act of God” — the human operator, outside of the computer
system, makes the decision. The speed of failover is limited by the speed at which
humans can act, which is generally slower than computers.
3. Use an algorithm to automatically choose a new leader. This requires a consen‐
sus algorithm; any system that performs automatic failover without using a pro‐
ven consensus algorithm is likely to behave badly in adverse network conditions
[107].
Although a single-leader database can provide linearizability without executing a
consensus algorithm on every write, it still requires consensus to maintain its leader‐
ship and for leadership changes. Thus, in some sense, having a leader only “kicks the
can down the road”: consensus is still required, only in a different place, and less fre‐
quently. The good news is that fault-tolerant algorithms and systems for consensus
exist, and we briefly discussed them in this chapter.
Tools like ZooKeeper play an important role in providing an “outsourced” consen‐
sus, failure detection and membership service that applications can use. It’s not easy
to use, but it is much better than trying to develop your own algorithms that can
withstand all the problems discussed in Chapter 8. If you find yourself wanting to do
one of those things that is reducible to consensus, and you want it to be fault-tolerant,
then it is advisable to use something like ZooKeeper.
Nevertheless, not every system necessarily requires consensus: for example, leaderless
and multi-leader replication systems typically do not use global consensus. The con‐
flicts that occur in these systems (see “Handling write conflicts” on page 164) are a
consequence of not having consensus across different leaders. But maybe that’s ok:
maybe we simply need to cope without linearizability, and learn to work better with
data that has branching and merging version histories.
This chapter referenced a large body of research on the theory of distributed systems.
Although the theoretical papers and proofs are not always easy to understand, and
sometimes make unrealistic assumptions, they are incredibly valuable for informing
practical work in this field: they help us reason about what can and cannot be done,
and help us find the counter-intuitive ways in which distributed systems are often
flawed. If you have the time, the
```
*   *Audit Focus:* Compare the codebase's architecture directly to the book summary in the text block above. Identify current coverage, gaps, overlaps, and optimization routes.

### Phase 10: Batch Processing
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter we explored the topic of batch processing. We started by looking at
Unix tools such as awk, grep and sort, and we saw how the design philosophy of
those tools is carried forward into MapReduce and more recent dataflow engines.
Some of those design principles include: inputs are immutable, outputs are intended
to become the input to another (as yet unknown) program, and complex problems
are solved by composing small tools that “do one thing well”.
In the Unix world, the uniform interface that allows one program to be composed
with another is files and pipes; in MapReduce, that interface is a distributed filesys‐
tem. We saw that dataflow engines add their own pipe-like data transport mecha‐
nisms to avoid materializing intermediate state to the distributed filesystem, but the
initial input and the final output of a job is still usually HDFS.
The the two main problem that distributed batch processing frameworks need to
solve are partitioning and fault tolerance:
Partitioning
In MapReduce, mappers are partitioned according to input file blocks. The out‐
put of mappers is re-partitioned, sorted and merged into a configurable number
of reducer partitions. The purpose of this process is to bring all the related data,
e.g. all the records with the same key, together in the same place.
Post-MapReduce dataflow engines try to avoid sorting unless it is required, but
they otherwise take a broadly similar approach to partitioning.
Fault tolerance
MapReduce frequently writes to disk, which makes it easy for a job to recover
from an individual failed task without restarting the entire job, but which slows
down execution in the failure-free case. Dataflow engines perform less materiali‐
zation of intermediate state and keep more in memory, which means that they
need to recompute more data if a node fails. Deterministic operators reduce the
amount of data that needs to be recomputed.
We discussed several join algorithms for MapReduce, most of which are also inter‐
nally used in MPP databases and dataflow engines. They also provide a good illustra‐
tion of how partitioned algorithms work:
Sort-merge joins
Each of the inputs being joined goes through a mapper that extracts the join key.
By partitioning, sorting and merging, all the records with the same key end up
going to the same call of the reduce function. This function can then output the
joined records.
Broadcast hash joins
One of the two join inputs is small, so it is not partitioned and it can be entirely
loaded into a hash table. Thus, you can start a mapper for each partition of the
large join input, load the hash table for the small input into each mapper, and
Summary | 417
then scan over the large input one record at a time, querying the hash table for
each record.
Partitioned hash joins
If the two join inputs are partitioned in the same way (using the same key, same
hash function, and same number of partitions), then the hash-table approach can
be used independently for each partition.
Distributed batch processing engines have a deliberately restricted programming
model: callback functions (such as mappers and reducers) are assumed to be stateless,
and to have no externally visible side-effects besides their designated output. This
allows the framework to hide some of the hard distributed systems problems: in the
face of crashes and network issues, tasks can be retried safely, and the output from
any failed tasks is discarded. If several tasks for a partition succeed, only one of them
actually makes its output visible.
Therefore, your code in a batch processing job does not need to worry about imple‐
menting fault tolerance mechanisms: the framework can guarantee that the final out‐
put of a job is the same as if no faults had occurred, even though in reality various
tasks perhaps had to be retried. These reliable semantics are much stronger than what
you usually have in online services that handle user requests, and that write to data‐
bases as a side-effect of processing a request.
The distinguishing feature of a batch processing jobs is that it reads some input data
and produces some output data, without modifying the input  — in other words, the
output is derived from the input. Crucially, the input data is bounded: it has a known,
fixed size  — for example, it consists of a set of log files at some point in time, or a
snapshot of a database’s contents. Because it is bounded, a job knows when it has fin‐
ished reading the entire input, and so a job eventually completes when it is done.
In the next chapter, we will turn to stream processing, in which the input is unboun‐
ded — that is, you still have a job, but its inputs are never-ending streams of data. In
this case, a job is never complete, because at any time there may still be more work
coming in. We shall see that stream and batch processing are similar at some level,
but the assumption of unbounded streams also changes a lot about how we build sys‐
tems.
```
*   *Audit Focus:* Compare the codebase's architecture directly to the book summary in the text block above. Identify current coverage, gaps, overlaps, and optimization routes.

### Phase 11: Stream Processing
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter we have discussed event streams, what purposes they serve and how to
process them. In some ways, stream processing is very much like the batch processing
we discussed in Chapter 10, but done continuously on an unbounded (never-ending)
stream rather than on a fixed-size input. From this perspective, message brokers and
event logs serve as the streaming equivalent of a filesystem.
We spent some time comparing two types of message broker:
AMQP/JMS-style message brokers
The broker assigns individual messages to consumers, and consumers acknowl‐
edge individual messages when they have been successfully processed. Messages
are deleted from the broker once they have been acknowledged. This approach is
appropriate as an asynchronous form of RPC (see also “Message passing data
flow” on page 132), for example in a task queue, where the exact order of mes‐
sage processing is not important, and where there is no need to go back and read
old messages again after they have been processed.
Log-based message brokers
The broker assigns all messages in a partition to the same consumer node, and
always delivers messages in the same order. Parallelism is achieved through par‐
titioning, and consumers track their progress by checkpointing the offset of the
last message they have processed. The broker retains messages on disk, so it is
possible to jump back and re-read old messages if necessary.
The log-based approach has similarities to replication logs found in databases (see
Chapter 5 ) and log-structured storage engines (see Chapter 3 ). We saw that this
approach is especially appropriate for stream processing systems that consume input
streams and generate derived state or derived output streams.
In terms of where streams come from, we discussed several possibilities: user activity
events, sensors providing periodic readings, and data feeds (e.g. market data in
finance) are naturally represented as streams. We saw that it can also be useful to
think of the writes to a database as a stream: it can capture the changelog, i.e. the his‐
tory of all changes made to a database, either implicitly through change data capture
or explicitly through event sourcing. Log compaction allows the stream to retain a
full copy of the contents of a database.
Representing databases as streams opens up powerful opportunities for integrating
systems. You can keep derived data systems such as search indexes, caches and ana‐
lytics systems continually up-to-date by consuming the log of changes and applying
them to the derived system. You can even build fresh views onto existing data by
starting from scratch and consuming the log of changes from the beginning all the
way to the present.
The facilities for maintaining state as streams and replaying messages are also the
basis for the techniques that enable streaming joins and fault tolerance in various
stream processing frameworks. We discussed several purposes of stream processing,
including searching for event patterns (complex event processing), computing win‐
Summary | 463
dowed aggregations (stream analytics), and keeping derived data systems up-to-date
(materialized views).
We then discussed the difficulties of reasoning about time in a stream processor,
including the distinction between processing time and event timestamps, and the
problem of dealing with straggler events that arrive after you thought your window
was complete.
We distinguished three types of join that may appear in stream processes:
Stream-stream joins
Matching two events that occur within some window of time  — for example, two
actions taken by the same user within 30 minutes of each other.
Stream-table joins
One input stream consists of activity events, while the other is a database change‐
log. The changelog keeps a local copy of the database up-to-date, while activity
events query the database and output an enriched activity event.
Table-table joins
Both input streams are database changelogs. In this case, every change on one
side is joined with the latest state of the other side. The result is a stream of
changes to the materialized view of the two tables.
Finally, we discussed techniques for achieving fault tolerance and exactly-once
semantics in a stream processor. As with batch processing, we need to discard the
partial output of any failed tasks. However, since a stream process is long-running
and produces output continuously, we can’t simply discard all output. Instead, a
finer-grained recovery mechanism can be used, based on microbatching, checkpoint‐
ing, transactions, or idempotent writes.
```
*   *Audit Focus:* Compare the codebase's architecture directly to the book summary in the text block above. Identify current coverage, gaps, overlaps, and optimization routes.

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
