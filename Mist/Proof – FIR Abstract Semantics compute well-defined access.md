---
tags: proof
---

%%Proof of [[Lemma – FIR Abstract Semantics compute well-defined access]]%%

Let $\T \in \Ts$, then we show by case distinction of $\inst \in \Inst$.

**Case 1** $\iuse\;\Rho$: Then $\bsem{\iuse\;\Rho}(\T) = \T \Requires \Rho$ ensuring $\Rho \subseteq \leaves(\Rho)$ by [[Lemma – Requires properties]] and [[Lemma – Requires commutative over compatible]] since $\Compat\Rho$. This gives us $\bsem{\iuse\;\Rho}(\T) \vdash \iuse\;\Rho$.

**Case 2** $\rho := a$: _todo_.

**Case 3** $\ifold\;\rho$:  Then $\bsem{\ifold\;\rho}(\T) = \T \Requires \fields(\rho)$ ensuring $\fields(\rho) \subseteq \leaves(\T)$ which gives us $\bsem{\ifold\;\rho}(\T) \vdash \ifold\;\rho$.

**Case 4** $\iunfold\;\rho$: Then $\bsem{\iunfold\;\rho}(\T) = \T \requires \rho$ ensures that $\rho \in \leaves(\T)$ which gives us $\bsem{\iunfold\;\rho}(\T) \vdash \iunfold\;\rho$.