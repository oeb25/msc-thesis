---
tags: subsection
---

Fundamentally, a folding tree has two operations: $\unfold$ and $\fold$, which both take a place and a folding tree, and either unfold or fold that place in the tree. The requirements for folding are that the given place is unfolded, and all of its fields are folded. Conversely, the requirement for unfolding is that the given place is folded.

![[Definition – Folding]]

> [!remark]
> To be clear about terminology, we will use _foldings_ to describe a collection of $\fold$'s and $\unfold$'s, while we will use and _folds_ to describe a collection of $\fold$'s and _unfolds_ to mean collections of $\unfold$'s.

![[Figure – Folding tree folding sequence]]

> [!example]
> If we again consider `struct S` from [[Figure – Breaking invariant]], then [[Figure – Folding tree folding sequence]] shows the $\unfold$ and $\fold$ operations transform the tree.
>
> We see that unfolding $s.y$ makes all of the fields of $s.y$ available and folded, while the last fold of $s.y$  removes the fields to have $s.y$ folded. A similar thing happens for the unfolding and folding of $s.y.a$.

As $\fold$ and $\unfold$ are the building blocks for further analysis, it is helpful to formalize some of their properties. The first property characterizes how the leaves of a tree change as folds and unfolds are performed.

![[Lemma – Leaves from folding]]

A detailed proof for [[Lemma – Leaves from folding]] is found in [[Proof – Leaves from folding]].

The following property is that the two functions, $\fold$ and $\unfold$, allow us to undo the actions of the other.

![[Lemma – Fold and unfold are inverse]]

> [!proof]
> Let $\rho$ be a place and $\T$ be a folding tree such that $\rho : (f_1, \dots, f_n) \in \leaves(\T)$. By definition of $\unfold$, $\leaves(\unfold(\rho, \T))$ will contain $\{f_1, \dots, f_n\}$, thus have the conditions met for folding $\rho$. Sequentially this means,
> $$
> \begin{aligned}
> \fold(\rho, \unfold(\rho, \T))
>   &= \fold(\rho, \T \cup \fields(\rho)) \\
>   &= (\T \cup \fields(\rho)) \setminus \fields(\rho) \\
>   &= \T. \\
> \end{aligned}
> $$
> Analogously, we show the other direction as well,
> $$
> \begin{aligned}
> \unfold(\rho, \fold(\rho, \T))
>   &= \unfold(\rho, \T \setminus \fields(\rho)) \\
>   &= (\T \setminus \fields(\rho)) \cup \fields(\rho) \\
>   &= \T. \\
> \end{aligned}
> $$

Another neat property is that $\fold$ and $\unfold$ commute under the inverse, allowing us to undo chains of foldings by reversing the chain and inverting every folding.

![[Proposition – Fold and unfold commute under inverse]]

> [!proof]
> Since function composition "$\circ$" is associative, then
> $$
> \begin{aligned}
> (\mathcal{F}_1 \circ \mathcal{F}_2) \circ (\mathcal{F}_2^{-1} \circ \mathcal{F}_1^{-1})
> &= \mathcal{F}_1 \circ (\mathcal{F}_2 \circ \mathcal{F}_2^{-1}) \circ \mathcal{F}_1^{-1} \\
> &= \mathcal{F}_1 \circ \mathcal{F}_1^{-1} = \text{\normalfont{id}}.
> \end{aligned}
> $$

As well as commuting under the inverse, they also commute when folding or unfolding compatible places.

![[Lemma – Fold and unfold commute over compatible]]

The proof of this [[Lemma – Fold and unfold commute over compatible]] is deferred to [[Proof – Fold and unfold commute over compatible]].

A typical operation on folding trees is transforming an existing tree into a new one with a desired place folded. A sequence of folds and unfolds must be performed to arrive at the desired tree. We call this operation _requires_ and use the notation $\T \requires \rho$ to say that we want the tree $\T$ but with minimal folds and unfolds performed to have $\rho$ folded.

![[Definition – Requires]]

![[Figure – Folding tree requires sequence]]

> [!example]
> [[Figure – Folding tree requires sequence]], which again works on `struct S` from [[Figure – Breaking invariant]], applies the requires operation of folding trees in a sequence.
>
> The first transition shows how performing an unfold of $.x$ makes $.x.u$ available, while the second transition shows the two unfolds necessary to make $.y.a.g$ available.
>
> The last two transitions show the folding up to make $.x$ and $.y$ folded.

One interesting fact about the requires operator, and something crucial for $\Requires$, is that if two places are compatible, it does not matter in which order we require them.

![[Lemma – Requires commutative over compatible]]

The proof for this lemma is deferred to [[Proof – Requires commutes over compatible]].

We can increase the applicability of $\requires$ by giving a closed-form formulation of the set produced by applying it.

![[Lemma – Requires closed-form]]

The proof of [[Lemma – Requires closed-form]] is deferred to [[Proof – Requires closed-form]].

Next, for $\requires$ to be helpful, we need to show some properties that it satisfies, but first, we need to establish a notion of leaves remaining after requiring a place.

![[Definition – Cut]]

> [!example]
> Consider again [[Figure – Folding tree requires sequence]] and let us look at cutting places in the trees:
> $$
> \begin{aligned}
> \cut(s.x,     \T_2) &= \{s.y\}, \\
> \cut(s.y.a,   \T_3) &= \{s.x.u,\;s.x.v,\;s.y.b\}, \\
> \cut(s.y.a.f, \T_3) &= \{s.x.u,\;s.x.v,\;s.y.a.g,\;s.y.a.h\;,\;s.y.b\}, \\
> \cut(s.y.a.f, \T_5) &= \{s.x\}. \\
> \end{aligned}
> $$

Intuitively, $\cut$ can be thought of as removing all leaves leading up to $\rho$ and all leaves which are children of $\rho$. This also means that cutting a field will not remove more than cutting its parent will. In the extreme case, cutting the root of the tree always _removes all leaves_, while cutting anywhere else does not necessarily do so.

![[Lemma – Folding tree weaken cut]]

> [!proof]
> Let $\rho.f_i$ be a field of $\rho$, and let $\T \in \Ts$ where $\rho \notin \T$. Then we can rephrase the definition of $\cut$ and alter the proof goal like so
> $$
> \begin{gathered}
> \cut(\rho, \T) = \{ \rho' \in \leaves(\T) \mid \rho \compat \rho' \} = \leaves(\T) \setminus \{ \rho' \in \T \mid \rho \incompat \rho' \}, \\[1ex]
> \leaves(\T) \setminus \{ \rho' \in \T \mid \rho \incompat \rho' \} \subseteq \leaves(\T) \setminus \{ \rho' \in \T \mid \rho.f_i \incompat \rho' \}.
> \end{gathered}
> $$
> This altered goal amounts to showing
> $$
> \{ \rho' \in \T \mid \rho \incompat \rho' \} \supseteq \{ \rho' \in \T \mid \rho.f_i \incompat \rho' \}.
> $$
> From here, let $\rho' \in \T$ be arbitrarily chosen (as a reminder $\rho' \neq \rho$,) then we must show $\rho' \incompat \rho.f_i \implies \rho' \incompat \rho$. By assuming $\rho' \incompat \rho.f_i$ we get that $\rho' < \rho.f_i$ or $\rho.f_i < \rho'$ from [[Definition – Compatible]]. By transitivity, we get that $\rho' < \rho$ from the first case or $\rho < \rho'$ from the second. These give us $\rho \incompat \rho'$, which concludes the proof.

One point of interest regarding $\cut$ is that the set of places it produces removes everything that would violate having $\rho$ be a leaf. Thus, it is quite valuable for specifying an essential property of $\requires$.

![[Lemma – Requires properties]]

The proof for this is deferred to [[Proof – Requires properties]]. With this lemma, we can ensure that requiring some place in one subtree does not alter the folded places of any unrelated subtree.

> [!example]
> Consider `struct S` from [[Figure – Breaking invariant]] once again. If we require some (potentially nested) field of `s.y,` then it will never fold nor unfold the fields of `s.x`.
>
> This ensures that folding is local to the part of the tree for which it is relevant.
