---
tags: proof
---

%%Proof of [[Lemma – FIR Abstract Semantics is a monotone function]]%%

Let $\T_1, \T_2 \in \Ts$ be folding trees satisfying $\T_1 \smaller \T_2$, and let $\inst \in \Inst$. Then we do a case distinction on $\inst$.

**Case 1** $\iuse\;\Rho$: We have to show $\T_1 \Requires \Rho \smaller \T_2 \Requires \Rho$, which holds immediately since $\Requires$ is monotone by [[Lemma – Requires is monotone]].

**Case 2** $\rho := a$: As $\bsems$ has two cases for $\rho := a$, there are four cases to consider:

**Case 2.1 $\rho \in \T_1 \Requires \pread(a) \land \rho \in \T_2 \Requires \pread(a)$**: Then $\T_1 \Requires \pread(a) \smaller \T_2 \Requires \pread(a)$ follow immediately from $\Requires$ being monotonic.

**Case 2.2 $\rho \notin \T_1 \Requires \pread(a) \land \rho \notin \T_2 \Requires \pread(a)$**: Then, again, $\T_1 \Requires (\pread(a) \cup \{\rho\}) \smaller \T_2 \Requires (\pread(a) \cup \{\rho\})$ follow immediately from $\Requires$ being monotonic.

**Case 2.3 $\rho \in \T_1 \Requires \pread(a) \land \rho \notin \T_2 \Requires \pread(a)$**: This case can never occur due to [[Lemma – Requires closed-form]].

**Case 2.4 $\rho \notin \T_1 \Requires \pread(a) \land \rho \in \T_2 \Requires \pread(a)$**: Then, again, $\T_1 \Requires (\pread(a) \cup \{\rho\}) \smaller \T_2 \Requires \pread(a)$ follow immediately from $\Requires$ being monotonic. By [[Lemma – Requires closed-form]], [[Lemma – Requires commutative over compatible]], and [[Lemma – Requires is monotone]], we can get $\T_1 \Requires \pread(a) \smaller \T_2 \Requires \pread(a)$. Then by again applying [[Lemma – Requires closed-form]] we can expand to $\{ \rho' \in \T_1 \Requires \pread(a) \mid \rho \not< \rho' \} \cup \cover(\rho) \smaller \T_2 \Requires \pread(a)$. Since we know that $\rho \notin \T_1 \Requires \pread(a)$, then the first set expression simplifies to $\T_1 \Requires \pread(a)$, and since $\rho \in \T_2 \Requires \pread(a)$ and since folding trees are cover-closed, then $\cover(\rho) \subseteq \T_2 \Requires \pread(a)$.

**Case 3** $\ifold\;\rho$:  We have to show $\T_1 \Requires \fields(\rho) \smaller \T_2 \Requires \fields(\rho)$, which again holds by [[Lemma – Requires is monotone]].

**Case 4** $\iunfold\;\rho$: We have to show $\T_1 \requires \rho \smaller \T_2 \requires \rho$, which again holds by [[Lemma – Requires is monotone]].

\qed