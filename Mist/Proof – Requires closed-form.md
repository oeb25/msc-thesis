---
tags: proof
---
%%Proof of [[Lemma – Requires closed-form]]%%

Let $\rho \in \places$ be a place, and $\T \in \Ts$ be a folding tree, then we examine the three cases of $\requires$ from [[Definition – Requires]]. We do so by structural induction on the cases, with **Case 1** being the base case.

**Case 1 $\rho \in \leaves(\T)$:** Then $\T \requires \rho = \T$ and we need to show $\T = \{ \rho' \in \T \mid \rho \not< \rho' \} \cup \cover(\rho)$. No places will matched $\rho < \rho'$, thus no places will be removed from $\T$. Finally, the union with $\cover(\rho)$ will add nothing new to $\T$, since $\T$ is cover-closed.

**Case 2 $\rho \notin \leaves(\T) \land \rho \in \T \land \fields(\rho) \subseteq \T$:** Then we need to show $\T \requires \rho = \fold(\rho, \T \Requires \fields(\rho)) = \{ \rho' \in \T \mid \rho \not< \rho' \} \cup \cover(\rho)$. By induction, we know that the requires performed in $\T \Requires \fields(\rho)$ will satisfy the closed-form formulation. Ultimately, the set produced will be
$$
\T \Requires \fields(\rho) = \{ \rho' \in \T \mid \fields(\rho) \cap \prefix(\rho') \neq \emptyset \} \cup \bigcup_{\rho' \in \fields(\rho)} \cover(\rho')
$$
By containing the union of all cover sets of all fields, we know that all fields at in $\T \Requires \fields(\rho)$, and since the condition $\fields(\rho) \cap \prefix(\rho') \neq \emptyset$ has removed all descendants of the fields of $\rho$, then $\fields(\rho) \subseteq \leaves(\T \Requires \fields(\rho))$, making $\fold\;\rho$ valid. Performing this will remove $\fields(\rho)$ thus $\rho \in \leaves(\fold(\rho, \T \Requires \fields(\rho)))$ and since the tree is cover-closed, $\cover(\rho)$ will be in the tree.

**Case 3 $\rho \notin \leaves(\T) \land \rho \notin \T \land \rho = \rho^*.f_i$:** Then we need to show $\T \requires \rho = \unfold(\rho^*, \T \requires \rho^*) = \{ \rho' \in \T \mid \rho \not< \rho' \} \cup \cover(\rho)$. By induction, we know that the requires performed in $\T \requires \rho^*$ will satisfy the closed-form formulation, and produce the set
$$
\T \requires \rho^* = \{ \rho' \in \T \mid \rho^* \not< \rho' \} \cup \cover(\rho^*)
$$
Since we know that $\rho \notin \T$ and $\T$ is cover closed, $\rho^*$ will have no descendants in $\T$ and thus we can simplify the above to
$$
\T \requires \rho^* = \T \cup \cover(\rho^*)
$$
By then expanding $\unfold(\rho^*, \T \cup \cover(\rho^*))$ we get the following
$$
\unfold(\rho^*, \T \cup \cover(\rho^*)) = \T \cup \cover(\rho^*) \cup \fields(\rho^*)
$$
The set $\cover(\rho)$ contains everything in $\cover(\rho^*)$ with the addition of the fields of the parent, thus $\cover(\rho^*) \cup \fields(\rho^*) = \cover(\rho)$, giving us $\T \requires \rho = \T \cup \cover(\rho)$.

---

Having shown the three cases of $\requires$ all result in producing the set $\{ \rho' \in \T \mid \rho \not< \rho' \} \cup \cover(\rho)$, then we have shown that the closed-form formulation captures the set produced by $\requires$. $\qed$
