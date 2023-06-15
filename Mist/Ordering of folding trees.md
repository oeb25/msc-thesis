---
tags: subsection
---

The goal of folding trees is to formalize the state of foldings at specific program points, and thus, we must not only have ways to mutate them but also have ways of relating one folding tree to another.

The first step is defining a _partial order_ for folding trees.

![[Definition – Folding tree order]]

![[Lemma – Folding tree partial order]]

> [!proof]
> To show this, we refer the fact that $\smaller$ is defined in terms of $\subseteq$ which forms a partial order.

With an ordering established, we define how to combine two trees by creating a new tree that fits within both or one that contains both trees. Such trees are especially useful for reasoning about the most unfolded tree, which does not unfold more than the two other trees.

![[Definition – Folding tree join and meet]]

![[Lemma – Folding tree join and meet are least upper bound and greatest upper bound]]

> [!proof]
> The operators are defined in terms of $\cup$ and $\cap$, which compute the least upper bound and greatest lower bound w.r.t. $\subseteq$, and thus so will $\join$ and $\meet$.

![[Figure – Folding meet join]]

> [!example]
> Consider the trees in [[Figure – Folding meet join]]. The tree $\T_1$ is more unfolded than $\T_2$ on $.x$ while being more folded on $.y.a$. On the left side, $\T_1 \meet \T_2$ is the most unfolded tree, which is still less unfolded than both $\T_1$ and $\T_2$. Similarly, on the right side, $\T_1 \join \T_2$ is the most folded tree, which does not fold any unfolded places of either $\T_1$ or $\T_2$.

The operators allow us to construct new trees, but when doing so, the resulting trees must still be folding trees.

![[Lemma – Folding tree join and meet are closed]]

> [!proof]
> The condition for being a folding tree is that the set must be closed under prefix in accordance to [[Definition – Folding tree]]. To show this, let $\T_1$ and $\T_2$ be arbitrary folding trees, then we can assume that
> $$
> \forall \rho \in \T_1 : \prefix(\rho) \subseteq \T_1, \;\text{ and, }\; \forall \rho \in \T_2 : \prefix(\rho) \subseteq \T_2.
> $$
> What we need to show is that
>  $$
> \forall \rho \in \T_1 \join \T_2 : \prefix(\rho) \subseteq \T_1 \join \T_2, \;\text{ and, }\; \forall \rho \in \T_1 \meet \T_2 : \prefix(\rho) \subseteq \T_1 \meet \T_2.
> $$
> For the first let $\rho_1$ be an element of $\T_1 \join \T_2$, then we know that $\rho_1$ is an element of $\T_1$ or $\T_2$. Without loss of generality assume $\rho_1 \in \T_1$, and we have $\prefix(\rho_1) \subseteq \T_1$ by the initial assumption. Then with the fact that $\T_1 \smaller \T_1 \join \T_2$, we can say that $\prefix(\rho_1) \subseteq \T_1 \join \T_2$ by transitivity.
>
> Next, let $\rho_2$ be an element of $\T_1 \meet \T_2$, which means that $\rho_2$ must be an element of both $\T_1$ and $\T_2$, thus giving us $\prefix(\rho_2) \subseteq \T_1$ and $\prefix(\rho_2) \subseteq \T_2$. Combining these two, we get that $\prefix(\rho_2) \subseteq \T_1 \cap \T_2$, which by [[Definition – Folding tree join and meet]] shows $\prefix(\rho_2) \subseteq \T_1 \meet \T_2$.

When working with these operators, it is helpful to be able to refer to _the most and the least folded tree_.

![[Definition – Folding tree top and bottom]]

> [!remark]
> Remember from [[Definition – Place]] that $\places$ is potentially infinite, which would make $\top$ an infinite set, meaning that it is impractical to represent fully in practice. Luckily, it satisfies two algebraic properties which make it simple to use in most cases:
> $$
> \begin{array}{ccc}
> \;\;\;\;&\top \join \T = \top, \text{ and, } \top \meet \T = \T & \forall \T \in \Ts.
> \end{array}
> $$

Having defined the ordering, upper and lower bounds, $\bot$ and $\top$, we have the necessary requirements to put it all together.

![[Lemma – Folding tree lattice]]

> [!proof]
> The requirements are showed in [[Lemma – Folding tree partial order]], [[Lemma – Folding tree join and meet are least upper bound and greatest upper bound]], and, [[Lemma – Folding tree join and meet are closed]].

Folding trees forming a lattice makes using the structure as the analysis domain possible, explored later in [[Computing solutions]].

%%

> [!example]
> To see a brief example of this in action, take $\T_1 \meet \T_2$ and $\T_1 \join \T_2$ shown in [[Figure – Folding meet join]], and let us consider the necessary foldings and unfoldings to arrive at both $\T_1$ and $\T_2$. For $\T_1 \meet \T_2$ to become $\T_1$ it requires $\unfold\;.x$ and to become $\T_2$ requires $\unfold\;.y.a$. On the other hand, starting at $\T_1 \join \T_2$ and arriving at $\T_1$ and $\T_2$ requires $\fold\;.x$ and $\fold\;.y.a$ respectively. This example indicates that $\join$ leads to foldings while $\meet$ leads to unfoldings.

%%

This leads us to the final bit of notation for folding trees: computing the minimal foldings and unfoldings required to transform one tree into another.

![[Definition – Folding tree transition]]

> [!example]
> Using the trees from [[Figure – Folding meet join]], we can compute the foldings by way of $\tinto$:
> $$
> \begin{gathered}
> \begin{aligned}
>     \tinto[\T_1 \meet \T_2, \T_1] &= \unfold\;.x &
>     \tinto[\T_1 \meet \T_2, \T_2] &= \unfold\;.y.a \\
>     \tinto[\T_1 \join \T_2, \T_1] &= \fold\;.x &
>     \tinto[\T_1 \join \T_2, \T_2] &= \fold\;.y.a \\
> \end{aligned} \\
> \begin{aligned}
>     \tinto[\T_1 \join \T_2, \T_1 \meet \T_2] &= \fold\;.x \circ \fold\;.y.a \\
>     \tinto[\T_1, \T_2] &= \fold\;.x \circ \unfold\;.y.a \\
> \end{aligned}
> \end{gathered}
> $$

This allows us to transition foldings from one program point into those of another, but it also allows us to go backward due to the invertible property of $\fold$ and $\unfold$.

> [!lemma]
> The function $\tinto$ is _anticommutative_ [[@bourbakiElementsMathematicsChapters2009#pp. 482]]:
> $$\tinto[\T_1, \T_2] = \tinto[\T_2, \T_1]^{-1}$$

> [!proof]
> Let $\T_1$ and $\T_2$ be arbitrary folding trees, and recall that $\mathcal{F}_1 \circ \mathcal{F}_2 = (\mathcal{F}_2^{-1} \circ \mathcal{F}_1^{-1})^{-1}$ from [[Lemma – Fold and unfold commute under inverse]], then:
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
