---
tags: proof
---

%%Proof of [[Lemma – FIR Abstract Semantics compute well-defined access]]%%

Let $\T \in \Ts$, then we show by case distinction of $\inst \in \Inst$.

**Case 1** $\iuse\;\Rho$: Then $\bsem{\iuse\;\Rho}(\T) = \T \Requires \Rho$ ensuring $\Rho \subseteq \leaves(\Rho)$ by [[Lemma – Requires properties]] and [[Lemma – Requires commutative over compatible]] since $\Compat\Rho$. This gives us $\bsem{\iuse\;\Rho}(\T) \vdash \iuse\;\Rho$.

**Case 2** $\rho := a$: We have two cases. First, assume $\rho \in \T \Requires \pread(a)$, then we must show $\rho \in \T \Requires \pread(a)$, which holds by assumption, and $\pread(a) \subseteq \leaves(\T)$, which we get from [[Lemma – Requires properties]]. For the second case, assume $\rho \notin \T \Requires \pread(a)$, then we must show $\rho \in \T \Requires (\pread(a) \cup \{\rho\})$ and $\pread(a) \subseteq \T \Requires (\pread(a) \cup \{\rho\})$ which holds by [[Lemma – Requires properties]] and [[Lemma – Requires commutative over compatible]].

**Case 3** $\ifold\;\rho$:  Then $\bsem{\ifold\;\rho}(\T) = \T \Requires \fields(\rho)$ ensuring $\fields(\rho) \subseteq \leaves(\T)$ which gives us $\bsem{\ifold\;\rho}(\T) \vdash \ifold\;\rho$.

**Case 4** $\iunfold\;\rho$: Then $\bsem{\iunfold\;\rho}(\T) = \T \requires \rho$ ensures that $\rho \in \leaves(\T)$ which gives us $\bsem{\iunfold\;\rho}(\T) \vdash \iunfold\;\rho$.