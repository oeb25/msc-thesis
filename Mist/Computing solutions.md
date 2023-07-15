---
tags: subsection
---

To perform the complete analysis, we first need to be specific about program points, as unique locations just _before_ and _after_ instructions. Each program point has two variants: one preceding the instruction and one immediately succeeding. For some preceding program point $\pp_i$, the corresponding succeeding program point will be $\pp_i'$. For referring to values found at each program point, we introduce an _analysis assignment_.

![[Definition – Analysis assignment]]

Using the analysis assignment, we can list the constraints that must be satisfied. As mentioned in [[Analysis function and abstract semantics]], we are looking for the least restrictive folding state at each program point, also known as the _greatest-solution_. The _monotone framework_ [[@nielsonProgramAnalysisAppetizer2020#p. 41]] encapsulates exactly such analyses, where the analysis function $\bsems$ is monotonic, see [[Lemma – FIR Abstract Semantics is a monotone function]], and where the analysis domain $\Ts$ forms a complete-lattice, see [[Lemma – Folding tree lattice]].

The system of constraints will contain one equation for every edge in the CFG, including those internally between FIR instructions, and one to specify the requirement at the end of the program.

![[Definition – System of constraints]]

![[Figure – CFG with annotated analysis]]

> [!example]
> [[Figure – CFG with annotated analysis]] shows a visualisation of the CFG of the [[Example – SubFib Semantics]] FIR program. The CFG on the left has annotated program points for the entry and exit of every basic block. On the right, we see seven equations, six for every edge in the CFG and one for the initial value $\T_\diamond$ for the otherwise unbounded `B4`. 

> [!remark]
> The initial value, $\T_\diamond$, is chosen to be the folding state _at the end of the function_, determined by the parameters and return type, where references and the returned value must be folded.

We can use many approaches and algorithms to solve these constraints with different performance characteristics [[@nielsonProgramAnalysisAppetizer2020#p. 59]], and the choice can significantly impact the efficiency of the analysis. Regardless of choice, all will arrive at the same result. In this chapter, however, we only care about the final analysis.

Using the monotone framework, we can depend on the properties shown regarding analyses performed in this framework.

![[Proposition – Monotone the computes greatest solution]]

> [!proof]
> This follows from [[@nielsonProgramAnalysisAppetizer2020#Proposition~3.28]], which says that the algorithm computes the _least solution_. We, however, are working with the _dual lattice_ [[@nielsonProgramAnalysisAppetizer2020#Section~3.4]], but that is known to compute the greatest solution by flipping the order.

Using [[Proposition – Monotone the computes greatest solution]], we get the folding states for all preceding program points, but the folding states for succeeding program points remain to be computed. These folding states, however, are entirely determined by computing the forward semantics on the preceding folding states. We can express this by the following equation, where $\sigma$ can be chosen arbitrarily:
$$
\sem{\inst_i}(\sigma, \A(\pp_i)) = \langle\sigma',\A(\pp_i')\rangle
$$

At this point, all program points have a folding state computed, but the folding state succeeding one block might not equal those preceding all subsequent blocks. We compute the necessary transitions between the two folding states to solve this discrepancy.

![[Definition – FIR Transition]]

> [!remark]
> Regarding the semantics $\sems$, the function $\delta$ takes the foldings at one program point, which might be predetermined by some preceding instruction, and transforms them into those expected at another program point. In terms of evaluation, $\delta$ satisfies the following equation for any choice of $\sigma$
> $$ \sem{\delta(\phi_i, \phi_j)}(\sigma, \T) = \langle \sigma, \tinto[\A(\phi_j), \A(\phi_i)](\T) \rangle$$

These transitions are inserted between program positions as necessary, such that when moving from one program point to another, the foldings and instructions executed along the way will give us the folding state at the destination.
