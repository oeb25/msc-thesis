---
tags: proof
---

%%Proof of [[Lemma – FIR Abstract Semantics is a monotone function]]%%

Let $\T_1, \T_2 \in \Ts$ be folding trees satisfying $\T_1 \smaller \T_2$, and let $\inst \in \Inst$. Then we do a case distinction on $\inst$.

**Case 1** $\iuse\;\Rho$: We have to show $\T_1 \Requires \Rho \smaller \T_2 \Requires \Rho$, which holds immediately since $\Requires$ is monotone by [[Lemma – Requires is monotone]].

**Case 2** $\rho := a$: _todo_.

**Case 3** $\ifold\;\rho$:  We have to show $\T_1 \Requires \fields(\rho) \smaller \T_2 \Requires \fields(\rho)$, which again holds by [[Lemma – Requires is monotone]].

**Case 4** $\iunfold\;\rho$: We have to show $\T_1 \requires \rho \smaller \T_2 \requires \rho$, which again holds by [[Lemma – Requires is monotone]].
