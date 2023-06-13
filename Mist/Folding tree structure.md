---
tags: section
---

A folding tree is a data structure, denoted by $\T$, that maintains the folding state of places. It works by representing data types as a tree, where nodes are fields of potentially nested `struct`s. The leaves of the tree are all the places that are _folded_, while the internal nodes are _unfolded_ places, both uniquely described by paths from the root.

![[Figure – A visualization of the maximally unfolded tree given by `S`]]

![[Definition – Folding tree]]

![[Example – Folding tree]]

When working with folding trees, it is useful to be able to refer to the set of folded places. For this we introduce a function $\leaves$ which computes this.

![[Definition – Leaves]]

> [!example]
> Consider again `struct S` from [[Figure – Breaking invariant]] and $\T_s$ from [[Example – Folding tree]], then we can compute the folded places in $\T_s$ as:
> $$\leaves(\T_s) = \{ s.x.u,\; s.x.v\;, s.y.a.f,\; s.y.a.g,\; s.y.a.h,\; s.y.b \}.$$

## Operations on folding trees

Fundamentally a folding tree has two operations: $\unfold$ and $\fold$, which both take a place and a folding tree, and either unfolds or folds that place in the tree. The requirements for folding, is that the given place is unfolded and that all of it's fields are folded. Conversely, the requirements for unfolding is that the given place is folded.

%%The data structure fundamentally supports two operations: $\unfold : \Rho \times T \to T$ and $\fold : \Rho \times T \to T$. The first operation, $\unfold(\rho, \T) = \T'$, requires $\rho$ to be folded in $\T$ (i.e. a leaf), denoted by $\leafin{\rho}{\T}$, and ensures that $\rho$ is unfolded in $\T'$ and consequently that all the fields of $\rho$ are accessible in $\T'$.^[This notation is borrowed from [[@comonTreeAutomataTechniques1997]].] On the other hand, $\fold(\rho, \T) = \T'$ is the inverse of $\unfold$, requiring that all fields of $\rho$ are folded in $\T$ and that none of them are accessible in $\T'$ and $\rho$ is. A demonstration of these operations is visualised in [[Figure – Folding tree folding sequence]]. Additionally, we let $\mathcal{F}$ be the space of all foldings and foldings.%%

![[Definition – Folding]]

![[Figure – Folding tree folding sequence]]

> [!example]
> If we again consider `struct S` from [[Figure – Breaking invariant]], then [[Figure – Folding tree folding sequence]] shows the $\unfold$ and $\fold$ operations transform the tree.
>
> We see that unfolding $.y$ makes all of the fields of $.y$ available and folded, while the last fold of $.y$  removes the fields to have $.y$ folded. A similar thing happens for the unfolding and folding of $.y.a$.

As $\fold$ and $\unfold$ are the building blocks for further analysis, it is helpful to formalise some of the properties they have, the first of which might seem trivial, but is perhaps the most important one.

![[Lemma – Leaves from folding]]

A detailed proof for this lemma is found in [[Appendix – Proof of leaves from folding#Proof of Lemma – Leaves from folding]].

The next property, is that the two functions, $\fold$ and $\unfold$, allow us to undo the actions of the other.

> [!lemma]
> The operations $\fold$ and $\unfold$ are inverse functions.^[Todo: Do we need to say that $\rho$ has to be the same, since they are binary, or is that implied?]

> [!proof]
> Let $\rho$ be a place and $\T$ be a folding tree such that $\rho : (f_1, \dots, f_n) \in \leaves(\T)$. By definition of $\unfold$, $\leaves(\unfold(\rho, \T))$ will contain $\{f_1, \dots, f_n\}$, thus have the conditions met for folding $\rho$. Sequentially this means,
> $$
> \begin{aligned}
> \fold(\rho, \unfold(\rho, \T))
> 	&= \fold(\rho, \T \cup \{ \rho.f_1, \dots, \rho.f_2 \}) \\
> 	&= (\T \cup \{ \rho.f_1, \dots, \rho.f_2 \}) \setminus \{ \rho.f_1, \dots, \rho.f_2 \} \\
> 	&= \T \\
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

For $\requires$ to be useful, we need to show some properties that it satisfies, but first, we need to establish way to talk about the remaining leaves after requiring a place.

![[Definition – Cut]]

> [!example]
> Consider again [[Figure – Folding tree requires sequence]] and lets look at cutting places in the trees:
> $$
> \begin{aligned}
> \cut(s.x,     \T_2) &= \{s.y\}, \\
> \cut(s.y.a,   \T_3) &= \{s.x.u,\;s.x.v,\;s.y.b\}, \\
> \cut(s.y.a.f, \T_3) &= \{s.x.u,\;s.x.v,\;s.y.a.g,\;s.y.a.h\;,\;s.y.b\}, \\
> \cut(s.y.a.f, \T_5) &= \{s.x\}. \\
> \end{aligned}
> $$

Intuitively, the $\cut$ can be thought of as removing all leaves leading up to $\rho$, and all leaves which are children of $\rho$. This also means that if we cut a field, it wont remove as much as cutting its parent. In the extreme case, cutting the root of the tree always _removes all leaves_, while cutting anywhere else, does not necessarily do so.

![[Lemma – Folding tree weaken cut]]

> [!proof]
> Let $\rho'$ be an element of $\leaves(T)$, then assume $\prefix(\rho) \not\subseteq \prefix(\rho')$, and, $\prefix(\rho') \not\subseteq \prefix(\rho)$, and recall that $\prefix(\rho) \subset \prefix(\rho.f_i)$ since $\prefix(\rho.f_i) = \prefix(\rho) \cup \{\rho.f_i\}$.
> 
> Since $\prefix(\rho')$ is not contained in $\prefix(\rho)$, and that it is a subset of $\prefix(\rho.f_i)$, we can say that $\prefix(\rho') \not\subseteq \prefix(\rho.f_i)$.
> 
> By a similar argument, we know that $\prefix(\rho.f_i)$ is larger than $\prefix(\rho)$, and since $\prefix(\rho) \not\subseteq \prefix(\rho')$, then $\prefix(\rho.f_i) \not\subseteq \prefix(\rho')$.
> 
> Since $\rho'$ was taken from $\leaves(\T)$ and the properties assumed were the conditions necessary to be in $\cut(\rho, \T)$, and we showed the conditions for being in $\cut(\rho.f_i, \T)$, we can conclude that the subset inclusion holds.

One point of interest regarding $\cut$, is that the set of places it produces _does not contain the provided $\rho$_, and thus does not form the leaves of a valid folding tree. It, however, does remove everything that would be in violation of having $\rho$ be a leaf, and thus is quite useful in specifying the perhaps the most important the property of $\requires$.

![[Lemma – Requires properties]]

The proof for this is deferred to [[Appendix – Proof of leaves from folding#Proof of Lemma – Requires properties]].

## Ordering of folding trees

The goal of folding trees is to formalise the state of foldings at specific program points, and thus it is crucial that we not only have ways to mutate them, but also have a way of relating one folding tree to another.

The first step for doing so, is defining a _partial order_ for folding trees.

![[Definition – Folding tree order]]

> [!lemma]
> The operator $\smaller$ defines a partial order for $\Ts$.

> [!proof]
> To show this, we use the fact that $\smaller$ is defined in terms of $\subseteq$ which forms a partial order.

---

Another property, which is useful for computing _greatest solutions_ discussed in [[Folding analysis]], is the fact that the folding tree forms a lattice.

For this, we first need to define the principle of ordering for folding trees.

![[Definition – Folding tree order]]

![[Lemma – Folding tree lattice]]

![[Figure – Folding meet join]]

More intuitively, the $\join$ of two trees is the smallest unfolding that contains two trees, while $\meet$ gives the largest tree that fits inside two trees.

To see a brief example of this in action, take $\T_1 \meet \T_2$ and $\T_1 \join \T_2$ shown in \cref{fig:folding-meet-join}, and let's consider the necessary foldings and unfoldings to arrive at both $\T_1$ and $\T_2$. For $\T_1 \meet \T_2$ to become $\T_1$ it requires $\unfold\;.x$ and to become $\T_2$ requires $\unfold\;.y.a$. On the other hand, starting at $\T_1 \join \T_2$ and arriving at $\T_1$ and $\T_2$ requires $\fold\;.x$ and $\fold\;.y.a$ respectively. As indicated by this example, $\join$ leads to foldings while $\meet$ leads to unfoldings.

This leads us to the final bit of notation for foldings trees, which is computing the minimal foldings and unfoldings required to transform one tree into another. We let $\tinto$ be the function that computes the composition of foldings and unfoldings satisfying the following equation:
$$
\begin{aligned}
\tinto[\T_1, \T_2] = \;& \mathcal{F}_n \circ \dots \circ \mathcal{F}_1 \\
\text{such that } [&\mathcal{F}_n \circ \dots \circ \mathcal{F}_1](\T_1) = \T_2
\end{aligned}
$$

![[Definition – Folding tree transition]]

Using the above example, we can compute the foldings by way of $\tinto$:
$$
\begin{gathered}
\begin{aligned}
    \tinto[\T_1 \meet \T_2, \T_1] &= \unfold\;.x &
    \tinto[\T_1 \meet \T_2, \T_2] &= \unfold\;.y.a \\
    \tinto[\T_1 \join \T_2, \T_1] &= \fold\;.x &
    \tinto[\T_1 \join \T_2, \T_2] &= \fold\;.y.a \\
\end{aligned} \\
\begin{aligned}
    \tinto[\T_1 \join \T_2, \T_1 \meet \T_2] &= \fold\;.x \circ \fold\;.y.a \\
    \tinto[\T_1, \T_2] &= \fold\;.x \circ \unfold\;.y.a \\
\end{aligned}
\end{gathered}
$$

> [!lemma]
> The function $\tinto$ is _anticommutative_:
> $$\tinto[\T_1, \T_2] = \tinto[\T_2, \T_1]^{-1}$$

> [!proof]
> Let $\T_1$ and $\T_2$ be arbitrary folding trees, and recall that $\mathcal{F}_1 \circ \mathcal{F}_2 = (\mathcal{F}_2^{-1} \circ \mathcal{F}_1^{-1})^{-1}$, then:
> $$
> \begin{aligned}
>   \T_1 &= \tinto[\T_1, \T_2]^{-1}(\tinto[\T_1, \T_2](\T_1)) \\
>        &= \tinto[\T_1, \T_2]^{-1}(\T_2) \\
>        &= [\mathcal{F}_n \circ \dots \circ \mathcal{F}_1]^{-1}(\T_2) \\
>        &= [\mathcal{F}_1^{-1} \circ \dots \circ \mathcal{F}_n^{-1}](\T_2) \\
>        &= \tinto[\T_2, \T_1](\T_2) \\
> \end{aligned}
> $$
> From this we get that $\T_1 = \tinto[\T_2, \T_1](\tinto[\T_1, \T_2](\T_1))$, showing that $\tinto[\T_2, \T_1]$ is the inverse of $\tinto[\T_1, \T_2]$.

With folding trees defined, we can continue to look at how they relate to programs written in Mist.
