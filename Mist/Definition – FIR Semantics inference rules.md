---
tags: definition
---

We define the semantics of all FIR instructions (see [[Definition – FIR]]) in terms of four inference rules:
$$
\begin{gathered}
\begin{array}{cc}
\begin{prooftree}
    \AXC{$\Rho \subseteq \dom(\sigma)$}
    \AXC{$\T \vdash \iuse\; \Rho$}
    \BIC{$\sem{\iuse\; \Rho}(\sigma, \T) = \langle \sigma, \T \rangle$}
\end{prooftree}
&
\begin{prooftree}
    \AXC{$\texttt{read}(a) \subseteq \dom(\sigma)$}
    \AXC{$\T \vdash \rho := a$}
    \BIC{$\sem{\rho := a}(\sigma, \T) = \langle \sigma[\rho \leftarrow \eval(a, \sigma)], \T \requires \rho \rangle$}
\end{prooftree}
\end{array}
\\[1em]
\begin{array}{cc}
\begin{prooftree}
    \AXC{$\rho \in \dom(\sigma)$}
    \AXC{$\T \vdash \ifold\; \rho$}
    \BIC{$\sem{\ifold\; \rho}(\sigma, \T) = \langle \sigma, \fold(\rho, \T) \rangle$}
\end{prooftree}
&
\begin{prooftree}
    \AXC{$\rho \in \dom(\sigma)$}
    \AXC{$\T \vdash \iunfold\; \rho$}
    \BIC{$\sem{\iunfold\; \rho}(\sigma, \T) = \langle \sigma, \unfold(\rho, \T) \rangle$}
\end{prooftree}
\end{array}
\end{gathered}
$$
