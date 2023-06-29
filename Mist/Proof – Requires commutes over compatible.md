---
tags: proof
---

Let $\rho_1 \compat \rho_2$ and let $\rho_3$ be the maximal place in both $\prefix(\rho_1)$ and $\prefix(\rho_2)$, that is
$$\forall \rho_i : \rho_i \in \prefix(\rho_1) \land \rho_i \in \prefix(\rho_2) \implies |\prefix(\rho_3)| \geq |\prefix(\rho_i)|.$$
Now, let $\F^1 = \F^1_1\circ\F^1_2\circ\dots\F^1_n$ be the sequence of foldings satisfying $\F^1(\T) = \T \requires \rho_1$, and $\F^2 = \F^2_1\circ\F^2_2\circ\dots\F^2_m$ be the sequence of foldings satisfying $\F^2(\T) = \T \requires \rho_2$. Then the sequence of foldings $\F^3 = \F^3_1\circ\F^3_2\circ\dots\F^3_q$ satisfying $\F^3(\T) = \T \requires \rho_3$ must be a prefix of both $\F^1$ and $\F^2$ such that
$$
\begin{aligned}
\F^1 &= \F^3 \circ \F^1_{n-q} \circ \dots \circ \F^1_n, \\
\F^2 &= \F^3 \circ \F^2_{m-q} \circ \dots \circ \F^2_m. \\
\end{aligned}
$$
From the maximality of $\rho_3$, we can know that all foldings in $\F^1$ not part of $\F^3$ be foldings of places compatible with all foldings in $\F^2$ not part of $\F^3$ since otherwise $\F^3$ could have been extended. By [[Lemma – Fold and unfold commute over compatible]], we can perform the following transformations:
$$
\begin{aligned}
\T \requires \rho_1 \requires \rho_2
	&= \T \requires \rho_3 \requires \rho_1 \requires \rho_2 \\
	&= \F^3(\T) \requires \rho_1 \requires \rho_2 \\
	&= [\F^1_{n-q}\circ\dots\circ\F^1_n](\F^3(\T)) \requires \rho_2 \\
	&= [\F^2_{m-q}\circ\dots\circ\F^2_m][\F^1_{n-q}\circ\dots\circ\F^1_n](\F^3(\T)) \\
	&= [\F^2_{m-q}\circ\dots\circ\F^2_m\circ\F^1_{n-q}\circ\dots\circ\F^1_n](\F^3(\T)) \\
	&= [\F^2_{m-q}\circ\dots\circ\F^1_{n-q}\circ\F^2_m\circ\dots\circ\F^1_n](\F^3(\T)) \\
	&\;\;\vdots \\
	&= [\F^1_{n-q}\circ\dots\circ\F^1_n\circ\F^2_{m-q}\circ\dots\circ\F^2_m](\F^3(\T)) \\
	&= [\F^1_{n-q}\circ\dots\circ\F^1_n][\F^2_{m-q}\circ\dots\circ\F^2_m](\F^3(\T)) \\
	&= \F^3(\T) \requires \rho_2 \requires \rho_1 \\
	&= \T \requires \rho_3 \requires \rho_2 \requires \rho_1 \\
	&= \T \requires \rho_2 \requires \rho_1 \\
\end{aligned}
$$
