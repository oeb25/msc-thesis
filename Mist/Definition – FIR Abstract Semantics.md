---
tags: definition
---

Let $\inst \in \Inst$ be a FIR instruction and $\T \in \Ts$ be a folding tree, then $\bsem{\inst}(\T)$ is the _backwards-analysis function_ for folding-analysis with signature
$$
\bsems : \Inst \to \Ts \to \Ts.
$$
The function is defined for each FIR instruction
$$
\begin{gathered}
\bsem{\iuse\;\Rho}(\T)
= \begin{cases}
	\T \Requires \Rho & \text{if }\; \Compat\Rho, \\
	\text{\normalfont undefined} & \text{\normalfont otherwise}
\end{cases}
\\[1ex]
\bsem{\rho := a}(\T)
= \begin{cases}
	\T \Requires \pread(a) & \text{if }\; \Compat\pread(a) \land \rho \in \T\Requires\pread(a), \\
	\T \Requires \pread(a) \cup \{\rho\} & \text{if } \Compat(\pread(a) \cup \{\rho\}) \land \rho \notin \T, \\
	\text{\normalfont undefined} & \text{if } \exists\rho' \in \pread(a) : \rho' < \rho, \\
	\text{\normalfont undefined} & \text{\normalfont otherwise}
\end{cases}
\\[1ex]
\bsem{\ifold\;\rho}(\T)
= \T \Requires \fields(\rho)
\hspace{8ex}
\bsem{\iunfold\;\rho}(\T)
= \T \requires \rho
\end{gathered}
$$
