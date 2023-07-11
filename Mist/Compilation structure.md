---
tags: section
---

The compilation starts at the source file and is transformed through a series of phases and analyses, ultimately resulting in editor diagnostics, such as syntax, type, and verification errors. Each stage can depend on the results of previous stages and, in a deterministic fashion, produce artifacts for later stages to use. These artifacts are composed of _entities_. Entities are uniquely identifiable compilation pieces, such as a source file, a parsed source file, a function definition, the MIR of some function or type invariant body, and so on.

![[Figure – Complication pipeline]]

[[Figure – Complication pipeline]] shows how the editor interacts with the language server (see [[Language server integration]]) to update a source file and how that source file is the origin of the rest of the compilation. Going down, we see compilation move through stages, the horizontal grouping explored in [[Compilation stages]]. Each stage creates a collection of new entities, which depend on entities from a mixture of the current and prior stages.

Entities form the basis of how data moves through the compiler and, together with functions that depend on these entities, form a dynamic dependency graph for a compilation. When computing functions, we cache the latest results based on the given set of entities in a memorized fashion. Caching allows us to reuse results in multiple functions and, crucially, skip recomputing functions if the direct dependencies did not change, even though some indirect dependencies might have. Introducing separation boundaries between stages minimizes direct dependencies and thus is essential for reducing overall recomputation on changes.

To orchestrate this computation, we use Salsa [[@matsakisSalsa2023]], a framework for incremental computation, also used in `rust-analyzer`. At the center of Salsa is a database of current and cached states, with tracked functions and structs.

![[Figure – Salsa SourceFile example]]


[[Figure – Salsa SourceFile example]] shows how to model a simplified version of the Mist compiler using Salsa. All inputs and output of the `#[salsa::tracked]` functions are tracked `struct`s, and under the hood, any call to these functions will coordinate with the database to construct and _self-adjust_ the dependency graph. Salsa then saturates the graph results and splits the computations onto multiple threads when queries are parallelizable.

![[Figure – Salsa Driver example]]

[[Figure – Salsa Driver example]] shows how the functions composing the compiler can be used to create a small program that verifies a Mist program. `fn drive` invokes the different functions to produce verification results. Meanwhile, `fn main` first creates and then alters the source file, giving it to `fn drive` in each case. We compute everything from scratch in the first iteration, but we reuse everything we can from the last iteration in the second. For example, even though the whitespace of `fn f` changed, it did not alter the `Item` it produced. Thus, we can reuse everything from that point on.

One crucial detail excluded from the above examples is the tracking of origins as data gets transformed. We call these _source maps_ and create one or more for each distinct stage of compilation. For the syntax tree, these are mappings of source nodes to text spans; in HIR, these are mappings of expressions to source nodes; in MIR, these are mappings of instructions to expressions; and finally, in VPR, these are mappings of Viper expressions to instructions. These mappings allow us to trace where a particular entity originated at any stage in computation. Source maps are a crucial part of the infrastructure, enabling us to display errors in expected locations.
