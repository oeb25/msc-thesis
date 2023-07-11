---
tags: proof
---

%%Proof of [[Lemma – Requires properties]]%%

We start expanding the left-hand side by [[Definition – Cut]] and the right-hand side by [[Lemma – Requires closed-form]]
$$
\begin{aligned}
\cut(\rho, \T) \cup \{ \rho \} &\subseteq \leaves(\T \requires \rho) \\
 \{ \rho' \in \leaves(\T) \mid \rho \compat \rho' \} \cup \{ \rho \} &\subseteq \leaves(\{ \rho' \in \T \mid \rho \not< \rho' \} \cup \cover(\rho))
\end{aligned}
$$
Immediately we know that $\rho \in \leaves(\T \requires \rho)$ from the fact that all descendants of $\rho$ are removed and $\rho \in \cover(\rho)$, making $\rho$ a leaf. Next, assume for some place $\rho' \in \leaves(\T)$ that $\rho \compat \rho'$, then we have to show $\rho' \in \leaves(\{ \rho' \in \T \mid \rho \not< \rho' \} \cup \cover(\rho))$. By expanding $\leaves$ by [[Definition – Leaves]], we get that we need to show
$$
\forall \rho'.f_i \in \fields(\rho') : \rho'.f_i \notin \{ \rho' \in \T \mid \rho \not< \rho' \} \cup \cover(\rho)
$$
This amounts to showing two things:

Firstly, $\rho'.f_i \notin \{ \rho' \in \T \mid \rho \not< \rho' \}$ requires $\rho'.f_i \notin \T$, and since $\rho' \in \leaves(\T)$ and $\rho'.f_i \in \fields(\rho')$, then $\rho'.f_i \notin \T$.

Secondly, to show $\rho'.f_i \notin \cover(\rho)$, remember that $\rho'.f_i \in \cover(\rho) \implies \rho' \in \cover(\rho)$, which by contraposition says that if $\rho' \notin \cover(\rho)$, then $\rho'.f_i \notin \cover(\rho)$. Thus we only need to consider cases where $\rho' \in \cover(\rho)$.

Since $\rho' \in \cover(\rho)$ and $\rho' \compat \rho$, then $\rho' \not< \rho$ and thus $\rho'$ must be a sibling of some ancestor of $\rho$. $\rho'.f_i$ will thus be a child of a sibling of a ancestor of $\rho$, which excludes it from being a sibling of a ancestor of $\rho$, showing $\rho'.f_i \notin \cover(\rho)$.

Having shown that a place, $\rho'$ arbitrarily chosen from the left-hand side of the initial equation, satisfies the conditions for being in the right-hand side, then we have showed that all places in the left set are included in the right. $\qed$

%%---

We split the proof goal to show these two properties:
$$
\begin{aligned}
\rho &\in \leaves(\T) & (1) \\
\cut(\T, \rho) &\subseteq \leaves(\T) & (2) \\
\end{aligned}
$$
We do the proof by case distinction on $\T \requires \rho$, and induction on $\rho$:

**Case 1**: Assume $\rho \in \leaves(\T)$, and, $\rho \in \T$, then $\leaves(\T \requires \rho) = \leaves(\T)$. Goal $(1)$ immediately follows, so what we need to show is
$$
\cut(\T, \rho) \subseteq \leaves(\T).
$$
Since $\cut(\T,\rho)$ is limited to leaves in $\T$, we know that the set will never contain elements outside of $\leaves(\T)$, concluding goal $(2)$.

**Case 2**: Assume $\rho \notin \leaves(\T)$, $\rho \in \T$, and, $\fields(\rho) \subseteq \T$, and let $\T_i = \T \requires \rho.f_1 \requires \dots \requires \rho.f_i$, then we need to show
$$
\cut(\T, \rho) \cup \{ \rho \} \subseteq \leaves(\fold(\rho, \T_n)).
$$
Now compute $\T_1 = \T \requires \rho.f_1$, which by induction gives us
$$
\begin{aligned}
\rho.f_1 &\in \leaves(\T_1) & (3) \\
\cut(\T, \rho.f_1) &\subseteq \leaves(\T_1) & (4) \\
\end{aligned}
$$
By [[Lemma – Folding tree weaken cut]] we can weaken $(4)$ to get
$$
\cut(\T, \rho) \subseteq \leaves(\T_1)
$$
Now we perform induction on the rest of the fields, where the previous case was the base case, and thus we assume that the following holds for field $i \geq 1$
$$
\begin{aligned}
\{\rho.f_1, \dots, \rho.f_i \} &\subseteq \leaves(\T_i) & (5) \\
\cut(\T, \rho) &\subseteq \leaves(\T_i) & (6) \\
\end{aligned}
$$
Then we compute $\T_{i+1} = \T_i \requires \rho.f_{i+1}$, which by induction says that
$$
\begin{aligned}
\rho.f_{i+1} &\in \leaves(\T_{i+1}) & (7) \\
\cut(\T, \rho.f_{i+1}) &\subseteq \leaves(\T_{i+1}) & (8) \\
\end{aligned}
$$
By $(7,8)$ we can derive that all prior fields of $\rho$ will still be in $\leaves(\T_{i+1})$, since $\prefix(\rho.f_u)$ will never be contained in $\prefix(\rho.f_v)$, and vice versa, for all $u,v$, and thus $\{\rho.f_1,\dots,\rho.f_i,\rho.f_{i+1}\} \subseteq \leaves(\T_{i+1})$. And again we can apply [[Lemma – Folding tree weaken cut]] on $(8)$ to weaken the property to get $\cut(\T, \rho) \subseteq \leaves(\T_{i+1})$.

By completing the induction on the fields, we end up with
$$
\begin{aligned}
\fields(\rho) &\subseteq \leaves(\T_n) & (9) \\
\cut(\T, \rho) &\subseteq \leaves(\T_n) & (10) \\
\end{aligned}
$$
Using [[Lemma – Leaves from folding]], with $(9)$ giving us the necessary conditions to perform $\fold\;\rho$ on $\T_n$, we show $(1)$. What remains to be shown is $(2)$, which after applying [[Lemma – Leaves from folding]] is
$$
\cut(\T, \rho) \subseteq (\leaves(\T_n) \setminus \fields(\rho)) \cup \{\rho\}
$$
From $(10)$ we know that the left-hand side is contained in $\leaves(\T_n)$, and since fields of $\rho$ will not satisfy $\prefix(\rho) \not\subseteq \prefix(\rho')$, no entries from $\fields(\rho)$ will be in $\cut(\T, \rho)$, thus showing $(2)$.

**Case 3:** Assume $\rho \notin \leaves(\T)$, $\rho \notin \T$, and, $\rho = \rho^*.f_i$ (that is, $\rho^*$ is the parent of $\rho$), and let $\T' = \T \requires \rho^*$, then we need to show
$$
\cut(\T, \rho) \cup \{ \rho \} \subseteq \leaves(\unfold(\rho^*, \T')).
$$
By induction on $\rho^*$, we know that
$$
\begin{aligned}
\rho^* &\in \leaves(\T') & (11) \\
\cut(\T, \rho^*) &\subseteq \leaves(\T') & (12) \\
\end{aligned}
$$
By $(11)$ we have the necessary condition met to perform $\unfold\;\rho^*$, and can by apply [[Lemma – Leaves from folding]] to alter the goal to
$$
\cut(\T, \rho) \cup \{\rho\} \subseteq (\leaves(\T') \setminus \{\rho^*\})\cup\fields(\rho^*).
$$
Since $\rho \in \fields(\rho^*)$ we get $(1)$. Next, we look at the two cases of elements $\rho' \in \cut(\T, \rho)$: If $\rho' = \rho^*$, then it will be excluded since $\prefix(\rho^*) \subseteq \prefix(\rho)$, satisfying $\subseteq$ in the right-hand side. Otherwise, $\rho' \neq \rho^*$, then we can use $(12)$ to show that it is included in $\leaves(\T')$. Thus showing $(2)$. \qed%%

%%---

We do the proof by case distinction on $\T \requires \rho$, and induction on $\rho$:

**Case 1**: Assume $\rho \in \leaves(\T) \land \rho \in \T$, then $\leaves(\T \requires \rho) = \leaves(\T)$, so what we need to show is $$\{ \rho' \mid \rho' \in \leaves(\T) \land \rho \notin \prefix(\rho') \} \cup \{ \rho \} \subseteq \leaves(\T)$$
From the assumption we know that $\rho \in \leaves(\T)$, and since the first set expression is limited to entries in $\leaves(\T)$, we know that $\{ \rho' \mid \rho' \in \leaves(\T) \land \rho \notin \prefix(\rho') \} \subseteq \leaves(\T)$.

**Case 2**: Assume $\rho \notin \leaves(\T)$, $\rho \in \T$, and, $\fields(\rho) \subseteq \T$, and let $\T' = \T \requires \rho.f_1 \requires \dots \requires \rho.f_n$, then we need to show
$$
\{ \rho' \mid \rho' \in \leaves(\T) \land \rho \notin \prefix(\rho') \} \cup \{ \rho \} \subseteq \leaves(\fold(\rho, \T')).
$$
By induction on $\rho$, we know that
$$
\begin{gathered}
\fields(\rho) \subseteq \leaves(\T'), \text{ and} \\
\{ \rho' \mid \rho' \in \leaves(\T) \land \rho.f_i \notin \prefix(\rho') \} \subseteq \leaves(\T') \\
\end{gathered}
$$
which makes $\T'$ satisfy the requirements for $\fold\;\rho$, and since the last operation performed is $\fold\;\rho$, and by [[Lemma – Leaves from folding]], we directly conclude that $\rho \in \leaves(\fold(\rho, \T'))$. Additionally, we can substitute $\leaves(\fold(\rho, \T'))$ for the alternative definition shown in [[Lemma – Leaves from folding]], and with the assumption that $\rho \notin \leaves(\T)$, it leaves us with:
$$
\{ \rho' \mid \rho' \in \leaves(\T) \land \rho \notin \prefix(\rho') \} \subseteq \leaves(\T') \setminus \fields(\rho).
$$
If we use $\{ \rho' \mid \rho' \in \leaves(\T) \land \rho.f_i \notin \prefix(\rho') \} \subseteq \leaves(\T')$, then we can get:
$$
\{ \rho' \mid \rho' \in \leaves(\T) \land \rho \notin \prefix(\rho') \} \subseteq \{ \rho' \mid \rho' \in \leaves(\T) \land \rho.f_i \notin \prefix(\rho') \} \setminus \fields(\rho).
$$
By expanding this to universal quantification form we get
$$
\forall \rho' \in \leaves(\T), \rho.f_i \in \fields(\rho) : \rho \notin \prefix(\rho') \implies \rho.f_i \notin \prefix(\rho') \land \rho' \notin \fields(\rho).
$$
From $\rho \notin \prefix(\rho')$ and assumption $\rho \notin \leaves(\T)$, we can derive $\fields(\rho) \cap \prefix(\rho') = \emptyset$, and since $\rho.f_i \in \fields(\rho)$ then $\rho.f_i \notin \prefix(\rho')$. Finally we know that if $\rho \notin \prefix(\rho')$, then $rho'$ cannot be a field of $\rho$, and thus $\rho' \notin \fields(\rho)$.

**Case 3:** Assume $\rho \notin \leaves(\T)$, $\rho \notin \T$, and, $\rho = \rho^*.f_i$ (that is, $\rho^*$ is the parent of $\rho$), and let $\T' = \T \requires \rho^*$, then we need to show
$$
\{ \rho' \mid \rho' \in \leaves(\T) \land \rho \notin \prefix(\rho') \} \cup \{ \rho \} \subseteq \leaves(\unfold(\rho^*, \T')).
$$
Since $\T'$ will have $\rho^*$ folded, we know that $\T'$ meets the requirements to perform $\unfold\;\rho$, and can thus use the equality from [[Lemma – Leaves from folding]]:
$$
\{ \rho' \mid \rho' \in \leaves(\T) \land \rho \notin \prefix(\rho') \} \cup \{ \rho \} \subseteq (\leaves(\T') \setminus \{\rho^*\}) \cup \fields(\rho^*).
$$
As $\rho \in \fields(\rho^*)$, we can remove both from each side of the equation, and insert the in-equality given by induction to get
$$
\{ \rho' \mid \rho' \in \leaves(\T) \land \rho \notin \prefix(\rho') \} \subseteq \{ \rho' \mid \rho' \in \leaves(\T) \land \rho^* \notin \prefix(\rho') \} \setminus \{\rho^*\}.
$$
Since%%
