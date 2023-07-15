---
tags: proof
---

%%Proof of [[Lemma – Meet of well-defined is well-defined]]%%

Most conditions for being well-defined relate to inclusion of places in the leaves. Any constraint imposed by $\inst$ will be in both $\leaves(\T_1)$ and $\leaves(\T_2)$ and thus also in their intersection. From [[Lemma – Meet preserves leaves]], we know that $\leaves(\T_1) \cap \leaves(\T_2) \subseteq \leaves(\T_1 \meet \T_2)$, which says that any of the aforementioned constraints will also be in $\leaves(\T_1 \meet \T_2)$.

The missing constraint is $\rho \in \T$ from $\T \vdash \rho := a$. By the hypothesis we have $\rho \in \T_1$ and $\rho \in \T_2$, which with [[Definition – Folding tree join and meet]], we get that $\rho \in \T_1 \meet \T_2$. $\qed$
