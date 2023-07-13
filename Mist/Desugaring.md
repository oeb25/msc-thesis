---
tags: subsubsection
---

It is not uncommon for particular language constructs to be nothing but more ergonomic but equivalent versions of other constructs. We call these _syntax sugar_, and the process for lowering these in terms of other constructs _desugaring_.

![[Example – Desugaring for loop]]

Desugaring helps keep fundamental language constructs limited, simplifying subsequent stages while allowing the language to grow ergonomically and independently of the rest of the compiler. As always, when desugaring pointers to the source, expressions should be tagged along with the new expressions for traceability.
