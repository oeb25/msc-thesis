---
tags: example
---

To see how $\sems$ works, we calculate the semantics of a sequence of instructions taken from `fn compute` in [[Example – SubFib FIR]]:
$$
\newcommand{\steparrow}{\textcolor{Gray400}\Rightarrow\;}
\newcommand{\semstep}[3]{
\sem{&#2}(#1) \\ &=\; \langle#3\rangle
}
\newcommand{\semsteps}[4]{
=\langle#1\rangle\\[1ex]\steparrow\;\sem{&#2}(#1)#3 \\ &=\; \langle#4\rangle
}
\newcommand{\semstepn}[4]{
\\[1ex]\steparrow\;\sem{&#2}(#1)#3 \\ &=\; \langle#4\rangle
}
\begin{aligned}
\semstep
	{\sigma_0, \T_0}
	{\texttt{r} := \texttt{SumFac \{ $\dots$ \}}}
	{\sigma_0[\texttt{r} \leftarrow \eval(\sigma_0, \texttt{SumFac \{ $\dots$ \}})], \T_0 \requires \texttt{r}}
\semsteps
	{\sigma_1, \T_1}
	{\iuse\; \{\texttt{i}, \texttt{m}\}}{}
	{\sigma_1, \T_1}
\semstepn
	{\sigma_1, \T_1}
	{\texttt{r.n} := \texttt{i}}{\;\;\;\lookmarker{(\times)}}
	{\sigma_1[\texttt{r.n} \leftarrow \eval(\sigma_1,\texttt{i})], \T_1 \requires \texttt{r.n}}
\semsteps
	{\sigma_2, \T_2}
	{\texttt{r.sum} := \texttt{r.sum + i}}{\;\;\;\lookmarker{(+)}}
	{\sigma_2[\texttt{r.n} \leftarrow \eval(\sigma_2,\texttt{r.sum + i})], \T_2 \requires \texttt{r.sum}}
\semsteps
	{\sigma_3, \T_3}
	{\texttt{r.fac} := \texttt{r.fac + i}}{}
	{\sigma_3[\texttt{r.n} \leftarrow \eval(\sigma_3,\texttt{r.fac + i})], \T_3 \requires \texttt{r.fac}}
\semsteps
	{\sigma_4, \T_4}
	{\iuse\; \{\texttt{r}\}}{\;\;\;\lookmarker{(\times\times)}}
	{\sigma_4, \T_4}
\end{aligned}
$$
Notice the two steps marked by $\lookmarker{(\times)}$ and $\lookmarker{(\times\times)}$: _neither_ of these executions are _well-defined_, as $\T_1 \vdash \texttt{r.n} := \texttt{i}$ does not hold since $\texttt{r.n} \notin \T_1$, and $\T_4 \vdash \iuse\; \{ \texttt{r} \}$ does not hold since $\texttt{r} \notin \leaves(\T_4)$. Interestingly, the step at $\lookmarker{(+)}$, which requires $\texttt{r.sum} \in \leaves(\T_2)$, is well-defined, since $\T_2 = \T_1 \requires \texttt{r.n}$ makes all fields of $\texttt{r}$ available in $\leaves(\T_2)$.
