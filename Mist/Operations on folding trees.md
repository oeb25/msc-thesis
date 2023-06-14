---
tags: subsection
---

Fundamentally, a folding tree has two operations: $\unfold$ and $\fold$, which both take a place and a folding tree, and either unfolds or folds that place in the tree. The requirements for folding are that the given place is unfolded  and that all of its fields are folded. Conversely, the requirement for unfolding is that the given place is folded.

![[Definition – Folding]]

![[Figure – Folding tree folding sequence]]

> [!example]
> If we again consider `struct S` from [[Figure – Breaking invariant]], then [[Figure – Folding tree folding sequence]] shows the $\unfold$ and $\fold$ operations transform the tree.
>
> We see that unfolding $.y$ makes all of the fields of $.y$ available and folded, while the last fold of $.y$  removes the fields to have $.y$ folded. A similar thing happens for the unfolding and folding of $.y.a$.

As $\fold$ and $\unfold$ are the building blocks for further analysis, it is helpful to formalize some of the properties they have, the first of which might seem trivial, but is perhaps the most important one.

![[Lemma – Leaves from folding]]

A detailed proof for this lemma is found in [[Appendix – Proof of leaves from folding#Proof of Lemma – Leaves from folding]].

The next property is that the two functions, $\fold$ and $\unfold$, allow us to undo the actions of the other.

> [!lemma]
> The operations $\fold$ and $\unfold$ are inverse functions.^[#todo: Do we need to say that $\rho$ has to be the same, since they are binary, or is that implied?]

> [!proof]
> Let $\rho$ be a place and $\T$ be a folding tree such that $\rho : (f_1, \dots, f_n) \in \leaves(\T)$. By definition of $\unfold$, $\leaves(\unfold(\rho, \T))$ will contain $\{f_1, \dots, f_n\}$, thus have the conditions met for folding $\rho$. Sequentially this means,
> $$
> \begin{aligned}
> \fold(\rho, \unfold(\rho, \T))
>   &= \fold(\rho, \T \cup \{ \rho.f_1, \dots, \rho.f_2 \}) \\
>   &= (\T \cup \{ \rho.f_1, \dots, \rho.f_2 \}) \setminus \{ \rho.f_1, \dots, \rho.f_2 \} \\
>   &= \T \\
> \end{aligned}
> $$

Another neat property, is that $\fold$ and $\unfold$ commutes under the inverse, allowing us to undo chains of foldings by reversing the chain and inverting^[#todo: Which is more correct here, "inverting" or "inversing"?] every folding.

![[Lemma – Fold and unfold are anticommutative]]

> [!proof]
> Assuming function composition "$\circ$" is associative, then
> $$
> \begin{aligned}
> (\mathcal{F}_1 \circ \mathcal{F}_2) \circ (\mathcal{F}_2^{-1} \circ \mathcal{F}_1^{-1})
> &= \mathcal{F}_1 \circ (\mathcal{F}_2 \circ \mathcal{F}_2^{-1}) \circ \mathcal{F}_1^{-1} \\
> &= \mathcal{F}_1 \circ \mathcal{F}_1^{-1} = \text{\normalfont{id}}.
> \end{aligned}
> $$

A common operation on folding trees is transforming an existing tree into a new one with a desired place folded. To do so, a sequence of foldings and unfoldings must be performed to arrive at the desired tree. We call this operation _requires_ and use the notation $\T \requires \rho$ to say that we want the tree $\T$ but with the minimal number of foldings and unfoldings performed to have $\rho$ be folded.

![[Definition – Requires]]

![[Figure – Folding tree requires sequence]]

> [!example]
> To get an idea of the operation, see [[Figure – Folding tree requires sequence]], which again works on `struct S` from [[Figure – Breaking invariant]].
>
> The first transition shows how a single unfolding of $.x$ is performed, to make $.x.u$ available, while the second transition shows the two unfoldings necessary to make $.y.a.g$ available.
>
> The last two transitions show the folding up to make $.x$ and $.y$ folded.

For $\requires$ to be useful, we need to show some properties that it satisfies, but first, we need to establish a way to talk about the remaining leaves after requiring a place.

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

Intuitively, the $\cut$ can be thought of as removing all leaves leading up to $\rho$, and all leaves which are children of $\rho$. This also means that if we cut a field, it won't remove as more than cutting its parent will. In the extreme case, cutting the root of the tree always _removes all leaves_, while cutting anywhere else, does not necessarily do so.

![[Lemma – Folding tree weaken cut]]

> [!proof]
> Let $\rho'$ be an element of $\leaves(T)$, then assume $\prefix(\rho) \not\subseteq \prefix(\rho')$, and, $\prefix(\rho') \not\subseteq \prefix(\rho)$, and recall that $\prefix(\rho) \subset \prefix(\rho.f_i)$ since $\prefix(\rho.f_i) = \prefix(\rho) \cup \{\rho.f_i\}$.
>
> Since $\prefix(\rho')$ is not contained in $\prefix(\rho)$, and that it is a subset of $\prefix(\rho.f_i)$, we can say that $\prefix(\rho') \not\subseteq \prefix(\rho.f_i)$.
>
> By a similar argument, we know that $\prefix(\rho.f_i)$ is larger than $\prefix(\rho)$, and since $\prefix(\rho) \not\subseteq \prefix(\rho')$, then $\prefix(\rho.f_i) \not\subseteq \prefix(\rho')$.
>
> Since $\rho'$ was taken from $\leaves(\T)$ and the properties assumed were the conditions necessary to be in $\cut(\rho, \T)$, and we showed the conditions for being in $\cut(\rho.f_i, \T)$, we can conclude that the subset inclusion holds.

One point of interest regarding $\cut$ is that the set of places it produces _does not contain the provided $\rho$_, and thus does not form the leaves of a valid folding tree. It, however, does remove everything that would violate having $\rho$ be a leaf, and thus is quite useful in specifying perhaps the most important property of $\requires$.

![[Lemma – Requires properties]]

The proof for this is deferred to [[Appendix – Proof of leaves from folding#Proof of Lemma – Requires properties]].

With [[Lemma – Requires properties]] we can ensure that requiring some place in one subtree does not alter the folded places of any unrelated subtree.

> [!example]
> Consider `struct S` from [[Figure – Breaking invariant]] once again. If we require some (potentially nested) field of `s.y`, then it will never fold nor unfold the fields of `s.x`.
>
> This ensures that folding stays isolated to the parts of a structure for which it is relevant.
