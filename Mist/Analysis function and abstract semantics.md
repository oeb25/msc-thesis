---
tags: subsection
---

- In many ways this is similar to liveness analysis [[@ahoCompilersPrinciplesTechniques1986#pp. 608]].

![[Definition – FIR Abstract Semantics]]

> [!lemma]
> $$\forall \T \in \Ts : \bsem{\inst}(\T) \vdash \inst$$

$$
\begin{prooftree}
    \AXC{$\bsem{\inst}(\T') = \T$}
    \UIC{$\T \vdash \inst$}
\end{prooftree}
$$
