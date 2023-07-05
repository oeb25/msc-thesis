---
tags: section
---

The larger a program is, the greater the effort for verification becomes for the programmer and the machine checking the code. To aid in this, we often introduce abstractions as a separation between the details of the implementation and the properties we want to derive from those abstractions.

Say we want to compute the sum of a range of numbers. The mathematical expression would be $\sum_{i=n}^{m-1}i$, but if we split it into two sums $\sum_{i=0}^{m-1}i - \sum_{i=0}^{n-1}i$, we can use a well-known equation for the sum up to some number, namely $\sum_{i=0}^{x-1}i={x(x-1)\over2}$, and get $\sum_{i=n}^{m-1}i={m(m-1)\over2}-{n(n-1)\over2}$. Here, we have two formulations of the same operations, where we want to expose the closed-form version as an abstraction but verify that it conforms to the original formulation as proof.

%%This might be done by taking the difference between the sum from zero to the first number and the sum from zero to the second number. There is a well-known closed-form equation for computing such a sum, which we want to apply in both cases to speed up the computation.
$$
\begin{array}{rl|rl}
\texttt{sum\_to}(n) &= \sum_{i=0}^{n-1} i& \texttt{sum\_of}(n,m) &= \sum_{i=n}^{m-1} i \\
&= {n (n-1) \over 2} & &= \texttt{sum\_to}(m) - \texttt{sum\_to}(n)
\end{array}
$$
Here, we have two formulations of the same operations, where we want to expose the more concise version as an abstraction but verify that it conforms to the other formulation as proof.%%
![[Figure – Modularity example]]

[[Figure – Modularity example]] shows the implementation of these two functions in Mist. `fn sum_to` \lineref{1-4} exposes the closed-form equation for the sum to `n`, while `fn sum_of` computes the sum of the range using `fn sum_to`. In the bodies of the function, we compute the sum using the iterative approach, with a loop `inv`ariant \lineref{8-9, 25-26} acting as an induction proof of our goal. At the end of each loop, we assert \lineref{13-14, 30-31} that the iterative result equals the closed-form equation.

Authoring such proofs might be challenging, and having the computer check them might also be computationally expensive. We go about this by splitting the proof burden of the body from the contract of the function, such that externally the function can solely expose the result of the computation.

Modularity benefits programmers, as they can focus on the final results and ignore unrelated proofs and implementation details. For verifies, much the same is true, as they can use prior proofs as facts and, in some cases, _ignore_ function bodies, limiting what they need to show at these abstraction boundaries.

Additionally, this allows functions to be verified independently, speeding up total verification time and allowing for caching of intermediary results.

How modularity plays a role in Mist, is covered more in-depth in [[Modes and modularity]].
