---
tags: subsection
---

$$
\begin{prooftree}
    \AXC{$\rho \in \T$}
    \AXC{$\texttt{read}(a) \subseteq \leaves(\T)$}
    \BIC{$\T \vdash \rho := a$}
\end{prooftree}
$$
$$
\begin{array}{ccc}
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
$$
$$
\begin{array}{ccc}
\begin{prooftree}
    \AXC{$\rho \in T$}
    \AXC{$\forall \rho.f_i \in T : \leafin{\rho.f_i}{\T}$}
    \BIC{$\T \vdash \fold\; \rho$}
\end{prooftree}
&
\begin{prooftree}
    \AXC{$\leafin{\rho}{\T}$}
    \UIC{$\T \vdash \unfold\; \rho$}
\end{prooftree}
\end{array}
$$
