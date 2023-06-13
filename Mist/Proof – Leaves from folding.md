---
tags: proof
---

We prove the two statements separately.
$$
\begin{aligned}
&\leaves(\fold(\rho, \T)) \\
  = \;&\text{[By [[Definition – Folding]]]} \\
	  &\leaves(\T \setminus \fields(\rho)) \\
  = \;&\text{[By [[Definition – Leaves]]]} \\
	  &\{ \rho' \mid \rho' \in (\T \setminus \fields(\rho)) \land \fields(\rho') \not\subseteq (\T \setminus \fields(\rho)) \} \\
  = \;&\text{[Distribute "$\in$" over "$\setminus$"]} \\
	  &\{ \rho' \mid \rho' \in \T \land \rho' \notin \fields(\rho) \land \fields(\rho') \not\subseteq (\T \setminus \fields(\rho)) \} \\
  = \;&\text{[Move $\rho' \notin \fields(\rho)$ out to removal of $\fields(\rho)$]} \\
      &\{ \rho' \mid \rho' \in \T \land \fields(\rho') \not\subseteq (\T \setminus \fields(\rho)) \} \setminus \fields(\rho) \\
  = \;&\text{[For $\rho' = \rho$, $\fields(\rho') \not\subseteq (\T \setminus \fields(\rho))$ evaluates to true, so move $\rho$ out]} \\
      &(\{ \rho' \mid \rho' \in \T \land \fields(\rho') \not\subseteq \T \} \cup \{\rho\}) \setminus \fields(\rho) \\
  = \;&\text{[By [[Definition – Folding]]]} \\
      &(\leaves(\T) \cup \{\rho\}) \setminus \fields(\rho) \\
\end{aligned}
$$
$$
\begin{aligned}
&\leaves(\unfold(\rho, \T)) \\
= \;&\text{[By [[Definition – Folding]]]} \\
	&\leaves(\T \cup \fields(\rho)) \\
= \;&\text{[By [[Definition – Leaves]]]} \\
	&\{ \rho' \mid \rho' \in (\T \cup \fields(\rho)) \land \fields(\rho') \not\subseteq (\T \cup \fields(\rho)) \} \\
= \;&\text{[Split set in two, by inclusion in either $\T$ or $\fields(\rho)$]} \\
	&\{ \rho' \mid \rho' \in \T \land \fields(\rho') \not\subseteq (\T \cup \fields(\rho)) \} \\
	&\cup \{ \rho' \mid \rho' \in \fields(\rho) \land \fields(\rho') \not\subseteq (\T \cup \fields(\rho)) \} \\
= \;&\text{[Since $\fields(\rho) \subseteq \leaves(\T)$, and $\rho \neq \rho'$, the last set becomes $\fields(\rho)$]} \\
	&\{ \rho' \mid \rho' \in \T \land \fields(\rho') \not\subseteq (\T \cup \fields(\rho)) \} \\
	&\cup \fields(\rho) \\
= \;&\text{[For $\rho' = \rho$, $\fields(\rho') \not\subseteq (\T \cup \fields(\rho))$ will be false,} \\&\; \text{and will be the only case where union with $\fields(\rho)$ makes a difference]} \\
	&(\{ \rho' \mid \rho' \in \T \land \fields(\rho') \not\subseteq \T) \} \setminus \{\rho\}) \cup \fields(\rho) \\
= \;&\text{[By [[Definition – Leaves]]]} \\
	&(\leaves(\T) \setminus \{\rho\}) \cup \fields(\rho) \\
\end{aligned}
$$
