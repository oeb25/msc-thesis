---
tags: section
title: "Case study 1: Reverse sort"
---

The first program we will study contains the function `fn rev_sort` and several functions to support it. The implementation of this is split between [[Figure – Reverse sort (Part 1) – Mist]] and [[Figure – Reverse sort (Part 2) – Mist]].

![[Figure – Reverse sort (Part 1) – Mist]]

[[Figure – Reverse sort (Part 1) – Mist]] contains four `ghost` functions. The first `pure ghost fn reversed` expresses the property of two lists being the reverse of each other if they are mirrored over the middle of the list. We use this property in the next function, `pure ghost fn reverse`, to ensure that the result of the function is the reverse of the given list.

We define `@reverse` in a particular way, such that it is callable in a pure context while giving implementing it using a non-pure iterative approach. We do this in two steps: first, define the postcondition of the function, and then give a `proof` body by way of a terminating induction proof, producing a value that satisfies the postcondition.

By separating the implementation of the specification, we get to express the function contract in the most natural and efficient way while deferring the proof burden to a distinct step. Thus, callers of the function will completely ignore the proof of the function and trust the function specification.

The remaining two functions allow us to relate the elements of reversed lists. The first, `pure ghost fn same_elements`, is a property similar to `reversed`, checking that two lists contain the same elements by converting them to `MultiSet`'s. The second, `ghost fn lemma_reverse_same_elements`, is a lemma, which checks that a reversed list contains the same elements as the original. For this lemma, an empty `proof {}` body is given, letting the verifier prove this automatically.

![[Figure – Reverse sort visualisation]]

The remainder of the program, [[Figure – Reverse sort (Part 2) – Mist]], contains the three functions involved in implementing the sorting algorithm.

The idea of the sorting algorithm is to have sorted progressively larger and larger prefixes of the given list by looking for elements smaller among those not yet in the prefix and reversing the sublist up to that element, moving it to just after the sorted prefix, and thus extending the sorted prefix. This process repeats until the prefix covers the entire list.

[[Figure – Reverse sort visualisation]] shows how the algorithm might sort a list of six elements. As we perform steps, the prefix grow longer and longer until the entire list is sorted.

Roughly computing the complexity for sorting a list of size $n$, this algorithm performs two nested loops with $O(n^2)$ iterations of the inner loop, each iteration potentially performing a reversal in $O(n)$, totaling in a runtime of $O(n^3)$. So to be clear, we in no way claim this algorithm to be _efficient_, but we claim _correctness_, as proved by the implementation.

The `fn rev_sort` is implemented as two loops. The compiler can assert this terminates as the loops are over bounded finite intervals, and all calls, namely to `@lemma_reverse_same_elements` and `@reverse`, are terminating functions.

The two loops contain invariants that maintain that the elements of the in-progress sorted list `res` contains the same elements as `s` and are sorted up to some index and that all elements of this prefix are smaller than the remainder of the list. The inner loop also maintains that the first element, not in the prefix, is the largest seen so far in this iteration. To help the verifier show that the elements are preserved, the `@lemma_reverse_same_elements` is invoked on the reversed list.

![[Figure – Reverse sort (Part 2) – Mist]]

This case study examined how `pure` and `ghost` can be used and combined to form expressive specifications for programs.