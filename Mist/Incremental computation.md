---
tags: section
---

When a software system, such as a compiler, needs to do a lot of work, and we want immediate feedback for all changes, we must employ tricks to cope with the complexity and inherent computation requirements.

One trick is to write _faster code_, and we do so when possible, but this might be at odds with maintainability and might not get us far enough.

Another trick is to _split the system into smaller pieces_, and run those systems in parallel. When done correctly, this can result in a linear speedup of certain code paths based on the number of cores.

A third trick is to split the system into smaller pieces again but then introduce boundaries between the systems where computation can be cached and halted. This trick is _incremental-computation_ and can prevent redoing work of some part of the system if the inputs to that subsystem have not changed or by updating the result with the changes.

Incremental computation can provide _asymptotic speedups_ by entirely skipping expensive work. Additionally, with systems such as Adapton [[@hammerAdaptonComposableDemanddriven2014]] and Salsa [[@matsakisSalsa2023]], it naturally allows for a different system architecture, where subsystems can be expressed as queries of other queries, dynamically forming a graph of dependencies. We can then inspect any part of the system by querying the relevant subsystems in a _demad-driven_ approach. When inputs change, we can again call these queries and expect the system to minimize the amount of unnecessary work to answer the queries again.

How we use incremental computation in the Mist compiler, is explored in detail in [[Compilation structure]].