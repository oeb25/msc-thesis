---
tags: subsection
---

After the MIR has been constructed and analyzed, it is ready for being verified.

Each MIR item is generated to MIR individually, but there might be interactions between multiple items which are revenant during verification. To determine which items need to be verified, we do a collection process similar to that in `rustc` [[@woeristerRustcRustcMonomorphize2016]]. By looking at strongly connected components (SCC) in the call graph and among `struct` fields, we can determine which items might be mutually recursive and thus need to be verified together. On the edge of these SCCs are all dependencies of some component of items. We might not need the function bodies for these items, and they can suffice by looking at signatures. As discussed in [[Modes and modularity]], non-pure functions have opaque implementations, and their bodies will not influence their specifications. However, for pure functions, their body should be considered during verification. We compute this selection of clusters of items by a selective transitive closure on the dependency graph.

This collection step aims to determine the smallest group of items to be verified together. As a result, we get to perform verification of smaller programs which is generally faster, and we can do this in parallel with the potential for caching already verified clusters.

As we use Viper [[@mullerViperVerificationInfrastructure2016]] for the verification infrastructure, we must generate programs in the Silver [[@silvercontributersViperprojectSilverDefinition]] language to send to ViperServer [[@viperserver]]. With Viper, we can encode the same principles of modularity by the use of `@vpr fn` and bodiless abstract `@vpr function`s.

In Mist, pure and non-pure functions are treated almost the same during MIR constructions; however, these two types might look very different in the resulting Viper code. Viper generally has two modes: _pure_ and _method_.

In Viper, the `@vpr function` can only contain pure code, and the same goes for any specification such as `@vpr requires`, `@vpr ensures`, `@vpr assert`, `invariant`, and in assignments. Methods, however, consist of statements that might contain pure code but also loops, assignments, and assertions.

The MIR must be conditionally generated based on the context when generating Viper code. Some constructs are inherently procedural and can thus only be included in methods. When generating method code, we reconstruct appropriate Viper constructs from the MIR CFG. Method generation might call into the pure code generation for particular sections, such as specification, when appropriate. When generating pure code, we look at the code in a more dataflow-centric approach, attempting to reconstruct the expression which produces the result of a MIR code path.

When we send the generated Viper code for verification, responses must be mapped back from the Viper-specific source locations to the MIR and then traced back to the source code for diagnostic reporting.
