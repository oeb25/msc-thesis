---
tags: proof
---
%%Proof of [[Lemma – Requires is monotone]]%%

Let $\rho \in \places$ and  $\T_1,\T_2 \in \Ts$ such that $\T_1 \smaller \T_2$. We want to show that $\T_1 \requires \rho \smaller \T_2 \requires \rho$, and start by applying [[Lemma – Requires closed-form]] and simplifying
$$
\begin{aligned}
\{ \rho’ \in \T_1 \mid \rho \not< \rho’ \} \cup \cover(\rho) &\smaller \{ \rho’ \in \T_2 \mid \rho \not< \rho’ \} \cup \cover(\rho) \\
\{ \rho’ \in \T_1 \mid \rho \not< \rho’ \} &\subseteq \{ \rho’ \in \T_2 \mid \rho \not< \rho’ \}
\end{aligned}
$$
Since the condition for being in the set is the same and $\T_1 \subseteq \T_2$, we know the final equality holds and finish the proof.
\qed
