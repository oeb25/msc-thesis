---
tags: definition
---

We define the semantics of all FIR instruction (see [[Definition – FIR]]) in terms of inference rules, using

$$
\begin{prooftree}
    \AXC{$\rho \in \dom(\sigma)$}
    \AXC{$\texttt{read}(a) \subseteq \dom(\sigma)$}
    \AXC{$\T \vdash \rho := a$}
    \TIC{$\sem{\rho := a}(\T, \sigma) = \langle \T \requires \rho, \sigma[\rho \leftarrow a] \rangle$}
\end{prooftree}
$$
$$
\begin{prooftree}
    \AXC{$\texttt{read}(a) \subseteq \dom(\sigma)$}
    \AXC{$\T \vdash \texttt{reference } a$}
    \BIC{$\sem{\texttt{reference } a}(\T, \sigma) = \langle \T, \sigma \rangle$}
\end{prooftree}
$$
$$
\begin{prooftree}
    \AXC{$\rho \in \dom(\sigma)$}
    \AXC{$\T \vdash \fold\; \rho$}
    \BIC{$\sem{\fold\; \rho}(\T, \sigma) = \langle \fold(\rho, \T), \sigma' \rangle$}
\end{prooftree}
$$
$$
\begin{prooftree}
    \AXC{$\rho \in \dom(\sigma)$}
    \AXC{$\T \vdash \unfold\; \rho$}
    \BIC{$\sem{\unfold\; \rho}(\T, \sigma) = \langle \unfold(\rho, \T), \sigma' \rangle$}
\end{prooftree}
$$
