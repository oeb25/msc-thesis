---
tags: definition
---

Let $\inst \in \Inst$ be a FIR instruction and $\T \in \Ts$ be a folding tree, then $\bsem{\inst}(\T)$ is the _backwards-analysis function_ for folding-analysis with signature
$$
\bsems : \Inst \to \Ts \to \Ts.
$$
The function is defined for each FIR instruction like so:
$$
\begin{gathered}
\bsem{\iuse\;\Rho}(\T) = \T \Requires \Rho
\\[1ex]
\bsem{\rho := a}(\T)
= \begin{cases}
	\T \Requires \pread(a) & \text{\normalfont if }\; \rho \in \T\Requires\pread(a), \\
	\T \Requires (\pread(a) \cup \{\rho\}) & \text{\normalfont otherwise} \\
\end{cases}
\\[1ex]
\bsem{\ifold\;\rho}(\T)
= \T \Requires \fields(\rho)
\hspace{8ex}
\bsem{\iunfold\;\rho}(\T)
= \T \requires \rho
\end{gathered}
$$
