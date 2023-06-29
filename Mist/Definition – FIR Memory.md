---
tags: definition
---

Let $\Value$ be the set of concrete values and let
$$\sigma : \places \hookrightarrow \Value$$
be a memory giving values to certain places, where $\dom(\sigma) \subseteq \places$ is the domain of $\sigma$, and $\Mem$ is the set of all memories.

Additionally, we let $\sigma[\rho \leftarrow v] \in \Mem$ be an update of $\sigma$ setting a place $\rho$ to value $v$, such that
$$
\sigma[\rho \leftarrow v](\rho') = \begin{cases}
	v & \text{\normalfont if } \rho = \rho' \\
	\sigma(\rho') & \text{\normalfont otherwise}
\end{cases}
$$
Finally, let $\eval : \Expr \to \Mem \hookrightarrow \Value$ be the function that evaluates an expression $a$ in a memory $\sigma$ given that $\pread(a) \subseteq \dom(\sigma)$.
