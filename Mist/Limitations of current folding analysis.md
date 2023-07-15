---
tags: subsection
---

The formalization of folding analysis [[Folding analysis]] has some limitations in three particular areas:

> [!paragraph] Conflicting capability usage
> 
> Expressions such as `f(s, s.a)` are in violation of well-defined access rules [[Definition – FIR well-defined access rules]], as $\texttt{s} \incompat \texttt{s.a}$.

> [!paragraph] Conflicting capability assignment
> 
> Expressions such as `r.sum = compute_sum(r);` are not allowed but could be by rewriting to `let tmp = compute_sum(r); r.sum = tmp;`, where the analysis would then insert an `unfold r` between the two statements, given that `r` is not consumed in the call.

> [!paragraph] Folding states are binary
> 
> In the current model, a place is either _folded_ or _unfolded_ with nothing in between, leading to some inconsistencies when dealing with `&` and `&mut`, which are mitigated by inspecting the type during insertion of folds and unfolds.

We have considered, but not explored in detail, two approaches to updated models which potentially solve the above three issues:

1. Introduce quantified permissions to the folding tree nodes, such that all ancestors must be at least as unfolded as their decedents. Let $\mathbb{Q} : \places \to \Ts \to \mathbf{Perm}$, the currently unfolded amount of a place in a folding tree such that for any tree $\T$ we have that:
$$
\forall \rho_1 < \rho_2 : \mathbb{Q}(\rho_1, \T) \geq \mathbb{Q}(\rho_2, \T)
$$
  This could allow partial unfoldings, where the well-defined rules could be updated to require _non-zero_ unfolded instead of the current binary choice.
1. Introduce three values of permissions for places: effective $\mathbb{E}$, allowed $\mathbb{A}$ and unfolded $\mathbb{U}$. Effective is the amount of permission we have to a place if we were to consume all other references to that place, allowed is the amount we are allowed to unfold, and unfolded is the amount we currently have unfolded. This would allow us to pass around partial access to places, where $\mathbb{E}(\rho, \T) \geq \mathbb{A}(\rho, \T) \geq \mathbb{U}(\rho, \T)$. An experiment of this is in the compiler, under the name `QRef`, where our $\mathbb{A}$ is partially inferred by the type system and augmented by a `@vpr Perm` following references in Viper, allowing us to know how and when to unfold, as computed by folding analysis, and how much, dynamically tracked by $\mathbb{A}$. The implementation is not yet at a stage where we can evaluate its effectivness.
