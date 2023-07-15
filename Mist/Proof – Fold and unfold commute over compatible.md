---
tags: proof
---

%%Proof of [[Lemma – Fold and unfold commute over compatible]]%%

Let $\rho_1 \compat \rho_2$, and derive that $\fields(\rho_1) \cap \fields(\rho_2) = \emptyset$. Then we have two choices for foldings for each, leading to four cases:

**Case 1:** Let $\F_1 = \fold\;\rho_1$ and $\F_2 = \fold\;\rho_2$, and assume that $\fields(\rho_1) \subseteq \leaves(\T)$ and $\fields(\rho_2) \subseteq \leaves(\T)$. Performing $\fold(\rho_2, \fold(\rho_1, \T))$ requires $\fields(\rho_1) \subseteq \leaves(\T)$, which holds by assumption, and $\fields(\rho_2) \subseteq \leaves(\fold(\rho_1, \T))$, which by expansion is $\fields(\rho_2) \subseteq \leaves(\T \setminus \fields(\rho_1))$. Since $\fields(\rho_1) \cap \fields(\rho_2) = \emptyset$, we can simplify to $\fields(\rho_2) \subseteq \leaves(\T)$ which holds by assumption, and thus
$$
\begin{aligned}
\fold(\rho_2, \fold(\rho_1, \T))
  &= (\T \setminus \fields(\rho_1)) \setminus \fields(\rho_2) \\
  &= (\T \setminus \fields(\rho_2)) \setminus \fields(\rho_1) \\
  &= \fold(\rho_1, \fold(\rho_2, \T)).
\end{aligned}
$$

**Case 2:** Let $\F_1 = \unfold\;\rho_1$ and $\F_2 = \unfold\;\rho_2$, and assume that $\{\rho_1,\rho_2\} \subseteq \leaves(\T)$. Performing $\unfold(\rho_2, \unfold(\rho_1, \T))$ requires $\rho_1 \in \leaves(\T)$, which holds by assumption, and $\rho_2 \in \leaves(\unfold(\rho_1, \T))$ which expands to $\rho_2 \in \leaves(\T \cup \fields(\rho_1))$. Since the fields of $\rho_1$ and $\rho_2$ are disjoint, then adding the fields of $\rho_1$ to $\T$ will not add any fields of $\rho_2$ preventing it from being unfolded, and thus
$$
\begin{aligned}
\unfold(\rho_2, \unfold(\rho_1, \T))
  &= (\T \cup \fields(\rho_1)) \cup \fields(\rho_2) \\
  &= (\T \cup \fields(\rho_2)) \cup \fields(\rho_1) \\
  &= \unfold(\rho_1, \unfold(\rho_2, \T)).
\end{aligned}
$$

**Case 3:** Let $\F_1 = \fold\;\rho_1$ and $\F_2 = \unfold\;\rho_2$, and assume that $\fields(\rho_1) \cup \{\rho_2\} \subseteq \leaves(\T)$. Performing $\unfold(\rho_2, \fold(\rho_1, \T))$ requires $\fields(\rho_1) \subseteq \leaves(\T)$, which holds by assumption, and $\rho_2 \in \leaves(\fold(\rho_1, \T))$ which expands to $\rho_2 \in \leaves(\T \setminus \fields(\rho_1))$. Since $\rho_1 \compat \rho_2$ then $\rho_2 \notin \fields(\rho_1)$, and thus removing the fields of $\rho_1$ from $\T$ will stop $\rho_2$ from being a leaf, and thus
$$
\begin{aligned}
\unfold(\rho_2, \fold(\rho_1, \T))
  &= (\T \setminus \fields(\rho_1)) \cup \fields(\rho_2) \\
  &= (\T \cup \fields(\rho_2)) \setminus \fields(\rho_1) \\
  &= \fold(\rho_1, \unfold(\rho_2, \T)).
\end{aligned}
$$

**Case 4:** Let $\F_1 = \unfold\;\rho_1$ and $\F_2 = \fold\;\rho_2$, and assume that $\{\rho_1\} \cup \fields(\rho_2) \subseteq \leaves(\T)$. Performing $\fold(\rho_2, \unfold(\rho_1, \T))$ requires $\rho_1 \in \leaves(\T)$, which holds by assumption, and $\fields(\rho_2) \subseteq \leaves(\unfold(\rho_1, \T))$ which expands to $\fields(\rho_2) \subseteq \leaves(\T \cup \fields(\rho_1))$. Since $\rho_1 \compat \rho_2$ then no fields of $\fields(\rho_2)$ will be in $\fields(\rho_1)$, and adding the fields of $\rho_1$ from $\T$ will stop $\fields(\rho_2)$ from being a leaves, and thus
$$
\begin{aligned}
\fold(\rho_2, \unfold(\rho_1, \T))
  &= (\T \cup \fields(\rho_1)) \setminus \fields(\rho_2) \\
  &= (\T \setminus \fields(\rho_2)) \cup \fields(\rho_1) \\
  &= \unfold(\rho_1, \fold(\rho_2, \T)).
\end{aligned}
$$
\qed