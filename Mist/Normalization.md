---
tags: subsubsection
---

As much as possible, we want HIR lowering to be deterministic. Thus, we use arena allocators, giving us deterministic identification of expressions, types, and variables (and memory locality as a side benefit). As mentioned in [[Type checking and name resolution]], each `Def` is lowered into HIR independently for the sake of modularity, and thus identification is local to each `Def`.

However, in the later stages of the compiler, we want to break this modularity slightly and work with interlinked definitions simultaneously. To support this, after the HIR has been constructed and type-checked, we do a pass over the HIR, _normalizing_ it.

Normalization does two things: Interning fully qualified types and rebuilding the expressions to be independent of their origin `Def`.

During type unification, many type-ids would have been unified to resolve to the same concrete type, and across definitions, many type-ids would resolve to the same concrete type but would not have been unified. Thus to bridge this gap, each type-id is replaced by a fully qualified and globally interned representation of the resolved concrete type.

The rebuilt expressions now carry the interned concrete types and a pointer back to their `Def`-specific expression id for traceability.