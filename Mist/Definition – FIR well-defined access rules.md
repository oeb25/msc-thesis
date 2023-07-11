---
tags: definition
---

Let $\T \in \Ts$ be a folding tree, and $\inst \in \Inst$ an FIR instruction, then we say that _$\inst$ is well-defined in terms of folding access for $\T$_, denoted by $\T \vdash \inst$, iff it is derivable from the following inference rules.
$$
\begin{gathered}
\begin{array}{cc}
\begin{prooftree}
    \AXC{$\Rho \subseteq \leaves(\T)$}
    \UIC{$\T \vdash \iuse\; \Rho$}
\end{prooftree}
&
\begin{prooftree}
    \AXC{$\rho \in \T$}
    \AXC{$\texttt{read}(a) \subseteq \leaves(\T)$}
    \BIC{$\T \vdash \rho := a$}
\end{prooftree}
\end{array}
\\[1em]
\begin{array}{cc}
\begin{prooftree}
    \AXC{$\fields(\rho) \subseteq \leaves(\T)$}
    \UIC{$\T \vdash \ifold\; \rho$}
\end{prooftree}
&
\begin{prooftree}
    \AXC{$\leafin{\rho}{\T}$}
    \UIC{$\T \vdash \iunfold\; \rho$}
\end{prooftree}
\end{array}
\end{gathered}
$$
Additionally, FIR assumes that both $\Compat\Rho$, for $\iuse\;\Rho$ and $\Compat(\pread(a) \cup \{\rho\})$ for $\rho := a$.