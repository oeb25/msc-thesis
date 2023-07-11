---
tags: definition
---

Let $\places$ be the (possibly infinite) set of all places inductively defined such that
$$
\begin{aligned}
\locals &\subseteq \places, \\
\{ \rho.f_i \in \fields(\rho) \mid \rho \in \places \} &\subseteq \places. \\
\end{aligned}
$$
Where $\fields(\rho)$ is the set of places of the fields of $\rho$. Formally:
$$
\fields(\rho) = \{ \rho.f_i \mid t = \texttt{typeOf}(\rho) \land f_i \in \texttt{typeField}(t) \}
$$
For places with types without fields, such as `int` and `bool`, we define $\fields(\rho) = \emptyset$.

Then any place $\rho$ starts with a local $s$ followed by a sequence of well-typed fields $f_1,\dots,f_n$, denoted by $\rho = s.f_1.\cdots .f_n$.
