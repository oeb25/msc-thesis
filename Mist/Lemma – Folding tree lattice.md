---
tags: lemma
---

The folding tree data structure forms a lattice with the following properties:

1. The bottom element $\bot$ is the tree where the maximal number of unfoldings are performed.
2. The top element $\top$ is the tree where the root is folded.
3. The ordering of the tree, given by $\T_1 \larger \T_2$ satisfies the property that any leaf in $\T_2$ is in $\T_1$ but not necessarily folded:
    $$
    \T_1 \larger \T_2 \Leftrightarrow \forall \leafin{\rho}{\T_2} : \rho \in \T_1
    $$
    and is defined by standard set inclusion, that is:
    $$
    \T_1 \larger \T_2 = \T_1 \supseteq \T_2,
    $$
4. The meet $\meet$ of $\T_1$ and $\T_2$ ensures that all leafs $\rho$ of $\T_1 \meet \T_2$ are leafs in either $\T_1$ or $\T_2$ and at least present in the other. More formally, the following equations describe the property:
    $$
    \begin{gathered}
    \rho \in (\T_1 \meet \T_2) = \rho \in \T_1 \land \rho \in \T_2 \\
    \leafin{\rho}{\T_1 \meet \T_2} = (\leafin{\rho}{\T_1} \land \rho \in \T_2) \lor (\leafin{\rho}{\T_2} \land \rho \in \T_1) \\
    \end{gathered}
    $$
    The tree given by $\T_1 \meet \T_2$ is equal to $\T_1 \cap \T_2$.
5. The join $\join$ of $\T_1$ and $\T_2$ contains all folded and unfolded places of both $\T_1$ and $\T_2$. It satisfies the following equations:
    $$
    \begin{gathered}
    \rho \in (\T_1 \join \T_2) = \rho \in \T_1 \lor \rho \in \T_2 \\
    \begin{aligned}
    \leafin{\rho}{\T_1 \join \T_2} =\;
        & (\leafin{\rho}{\T_1} \land (\leafin{\rho}{\T_2} \lor \rho \notin \T_2)) \\ \lor
        & (\leafin{\rho}{\T_2} \land (\leafin{\rho}{\T_1} \lor \rho \notin \T_1))
    \end{aligned}
    \end{gathered}
    $$
    Opposite to to $\meet$, the tree given by $\T_1 \join \T_2$ is equal to $\T_1 \cup \T_2$.
