---
tags: definition
---

Let $\T$ be a folding tree, and $\inst$ an FIR instruction, then we say that _$\inst$ is well-defined in terms of folding access for $\T$_, denoted by $\T \vdash \inst$, iff it is derivable from the following inference rules.
$$
\begin{gathered}
\begin{array}{ccc}
\begin{prooftree}
    \AXC{$\rho \in \T$}
    \AXC{$\texttt{read}(a) \subseteq \leaves(\T)$}
    \BIC{$\T \vdash \rho := a$}
\end{prooftree}
&
\begin{prooftree}
    \AXC{$\texttt{read}(a) \subseteq \leaves(\T)$}
    \UIC{$\T \vdash \texttt{reference } a$}
\end{prooftree}
&
\begin{prooftree}
    \AXC{$\leafin{\rho}{\T}$}
    \UIC{$\T \vdash \texttt{mention } \rho$}
\end{prooftree}
\end{array}
\\[1em]
\begin{array}{cc}
\begin{prooftree}
    \AXC{$\fields(\rho) \subseteq \leaves(\T)$}
    \UIC{$\T \vdash \fold\; \rho$}
\end{prooftree}
&
\begin{prooftree}
    \AXC{$\leafin{\rho}{\T}$}
    \UIC{$\T \vdash \unfold\; \rho$}
\end{prooftree}
\end{array}
\end{gathered}
$$
