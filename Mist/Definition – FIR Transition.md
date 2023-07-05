---
tags: definition
---

Let $\A$ be an analysis assignment, then $\delta : \Pp \to \Pp \to \Inst$ is the _transition instructions_ which transforms the analysis assignment at one program position into those at another, i.e.
$$
\begin{aligned}
\delta(\phi_i', \phi_j) &= f(\F_1) \;;\; \dots \;;\; f(\F_n) \\
  &\text{\normalfont where } \tinto[\A(\phi_i'), \A(\phi_j)] = \F_n \circ \dots \circ \F_1 \\
\end{aligned}
$$
Where $f$ is the function mapping foldings from functions to instructions, i.e.
$$
f(\fold\;\rho) = \ifold\; \rho, \hspace{3ex} f(\unfold\;\rho) = \iunfold\; \rho.
$$
