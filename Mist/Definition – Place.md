---
tags: definition
---

Let $\places$ be the (possibly infinite) set of all places inductively defined such that
$$
\begin{aligned}
\locals &\subseteq \places, \\
\{ \rho.f_i \mid \rho \in \places \land \texttt{t} = \texttt{typeOf}(\rho) \land f_i \in \fields(\texttt{t}) \} &\subseteq \places. \\
\end{aligned}
$$
Then any place $\rho$ starts with a local $s$ followed by a sequence of well-typed fields $f_1,\dots,f_n$, denoted by $\rho = s.f_1.\cdots .f_n$.
