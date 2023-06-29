---
tags: definition
---

Let $\inst \in \Inst$ be a FIR instruction and $\T \in \Ts$ be a folding tree, then $\bsem{\inst}(\T)$ is the _backwards-analysis function_ for folding analysis with signature
$$
\bsems : \Inst \to \Ts \to \Ts.
$$

$$
\begin{gathered}
\begin{array}{cc}
\bsem{\iuse\; \Rho}(\T) = \T \Requires \Rho
&
\bsem{\rho := a}(\T) = \begin{cases}
	\T \Requires \pread(a) & \text{\normalfont if } \rho \in \T \Requires \pread(a) \\
	\T \Requires \pread(a) \requires \rho & \text{\normalfont otherwise}
\end{cases}
\end{array}
\\[1ex]
\begin{array}{cc}
\bsem{\ifold\;\rho}(\T) = \unfold(\rho, \T)
&
\bsem{\iunfold\;\rho}(\T) = \T \requires \rho
\end{array}
\end{gathered}
$$
