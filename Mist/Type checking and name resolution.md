---
tags: subsubsection
---

The first stage of HIR construction is transforming the semi-structured AST nodes into a representation where all names are converted to types, functions, parameters, variables, and so on, and where the types of these are explicitly associated.

The first step is looking at all top-level constructs, among which are `struct` and `fn` declarations and type `invariant`s, and we call these `Def`. These are initially only processed at the signature level, to register them by name and resolve argument types, but nothing more. Only after these have all been registered, can we start looking a fields of structs and look at the bodies of functions and type invariants. The reason this is done in two distinct steps, is because initially we do not know the dependency graph of `Def`s, and even if we did, cyclic dependencies would still require short-circuiting somehow. Additionally, this plays into how we do modularity and incremental analysis; by limiting dependencies at the definition signatures, we _prevent the analysis of one definition body from influencing that of another_.

The next step is lowering expressions inside the definition bodies. Three concepts generally apply in this process:

- Each variable is given a unique identifier; representing a variable solely by its name is insufficient since Mist allows for shadowing. Each variable is given a _type-id_, either from an explicit type or an inferred from context.
- Each expression is given a unique identifier and a type-id.
- Each explicit type is given a type-id.

When the above three things are created, a mapping to and from the AST node that resulted in its creation is created. This is crucial in supporting traceability throughout the compilation.

Inside a definition body, statements are lowered in order, and variables get declared in the current scope as they are encountered. If a variable has the same name as one already in scope, the old variable gets replaced with the new one for the remainder of this scope.

Expressions are lowered in a _post-order_ fashion, which is to say that the children of an expression are always lowered before the expression itself. In this way, typing information always flows upwards. This is important, as resolving which kind of expression we are dealing with often requires inspecting types of subexpressions.

For typing like this to work consistently, however, we require _type inference_. When any type gets created, it gets a type-id, which gets inserted into a table-like structure, denoted by $\Gamma$, where it might point to a concrete type such as `int`, `bool`, `[int]`, some `struct Person`, or a generic `T`, or it points to some partial constraint type. These unconstrained types are called _free_ types, which are so far undetermined types.

![[Example – List type inference]]

Multiple type-ids might point to the same thing, which is a crucial part of type inference, namely _type unification_. We use Hindly-Milner [[@hindleyPrincipalTypeSchemeObject1969]] [[@milnerTheoryTypePolymorphism1978]] style type inference, something we will not cover in detail, but at its core is built on a collection of inference rules, which state how types are determined, and by exclusion, which configurations are illegal. In such a system, free types are variables in the equations, and unique solutions to these equations will give the concrete types of the free types.

> [!example]
> Continuing [[Example – List type inference]] to the next expression `[1] + list + [2]` \lineref{3}, we see a concatenation of three lists. Two interesting things happen here:
> 
> First, the operator `+` can do both _list concatenation_ and _add numbers_. To figure out which to do, we must look at the operand, where we see that all operands are of a list type. Secondly, we have an inference rule roughly stating
> $$
> \begin{prooftree}
> \AXC{$\Gamma \vdash e_1 : [t]$}
> \AXC{$\Gamma \vdash e_2 : [t]$}
> \RightLabel{\footnotesize Concatenation}
> \BIC{$\Gamma \vdash e_1 + e_2 : [t]$}
> \end{prooftree}
> $$
> In our case, the choice of $e_1$ and $e_2$ would be `[1]` and `list`, where `[1]: [int]` and `list: ['a]`. The symbol $\Gamma$ referrers to the assignment of type variables, and thus under $\Gamma$ we see that `[int]`  and `['a]` should be unified to solve the equation, i.e.
> $$
> \begin{prooftree}
> \AXC{$\Gamma \vdash \keyword{int} = \texttt{'a}$}
> \UIC{$\Gamma \vdash \texttt{[\keyword{int}]} = \texttt{['a]}$}
> \end{prooftree}
> $$
> This registers the unification in $\Gamma$ and propagates up to let `list: [int]`.

Moving on from type inference, when a name is encountered in an expression, that name is looked up the in the current scope, and a new HIR expression is created with a reference to the unique id of the variable. If the name is not in scope, then a special error expression is inserted in its place.

Error expressions are used to fill the holes where expression could not be constructed, such as when missing in the AST. Such expressions are given a special `error` type, which ignores unification, and silently propagates through type checking. When error expressions are created, the type checker may or may not emit an error depending on the context. However, we want type checking to be resilient and continue compilation even in the face of errors. However, specific subsystems later in the pipeline may choose to stop compilation if they see errors in prior stages.
