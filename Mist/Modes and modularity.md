---
tags: section
---

In the previous section, we saw the functions `@values` and `@height` marked with _pure_, which limits and requires them to satisfy specific properties but enables them to be used in more contexts, such as `req`, `ens`, and `inv`, where non-pure functions are not allowed. We can summarise constraints to these: _The result of the function must be computed without any side effects or mutations_ and _the function cannot perform any external mutations_.

![[Example – BTree – Size]]

[[Example – BTree – Size]] introduces another pure function and new concept. The `dec` annotation on `@size` requires the function _terminates_ by requiring any recursive call to `@size` to be _smaller_ than this call. The notion of smaller is given by the accompanying `height(b)` to mean that any tree passed in a recursive call to `@size` must have a height less than `b`. `@height` is a pure function and thus can be called in `dec`. As the tree's height is never negative, and the height of recursive calls forever decreases, we know any recursive call sequence must end. Additionally, any other expression in the function must also be terminating for the function to be decreasing.

All functions so far have taken immutable references. However, Mist allows passing _mutable references_ as well. These are references where the function may change the contents, but ultimately, the object must still be intact at the end of the function body.

The function `@increment` found in [[Example – BTree – Increment]], takes such a mutable reference to a `BTree` and recursively increments all values by some `amount`.

![[Example – BTree – Increment]]

The postcondition of `@increment` ensures that all values previously in the tree will now be in the mutated tree but offset by `amount`. The keyword `old` in `old(values(b))` is critical here, as it refers to the value of `b` _before execution_ of the function. 

Non-pure functions, like `@increment`, are opaque in that as long as their body satisfies the postcondition, then its implementation cannot influence other parts of the code. The _mirroring of the tree_ exemplifies this at the end of the function $\lineref{55}$. Nothing about the internals of a non-pure function can be assumed externally, and nothing states that `@increment` must preserve the order in the tree, so mirroring the tree is completely fine as it satisfies the contract, while perhaps unexpected.

We have now defined quite a few functions for `BTree`, which come together in one final property.

![[Example – BTree – Complete lemma]]

[[Example – BTree – Complete lemma]] contains a _ghost_ function, which, like pure, imposes certain constraints on where it can be called and what it must satisfy, which are:

- Ghost code can only call ghost and pure code.
- A value that depends on the value of ghost code is also ghost code.
- Ghost code must be terminating.

Ghost code is tracked in the type system and by the surrounding context. For those familiar with information flow analysis [[@denningCertificationProgramsSecure1977]] [[@volpanoSoundTypeSystem1996]], propagation and detection of leaking ghost code follow similar principles.

In the case of `@lemma_complete`, the function expresses the result we want to show in the postcondition, and then a _proof_ is located on the function. Proofs in Mist are like function bodies and are used not for computing results but rather to _show how to arrive at the goal_. The proof shown in `@lemma_complete` is a structural induction proof on the tree, where the prover needed a little help to show a particular case of $2 \cdot 2^n = 2^{n+1}$ in the base case, and two uses of the induction hypothesis in the induction step. The block returns nothing, but the body enables the verifier to finish the proof for us.

Ghost code is also more powerful in that it can express properties _without_ giving evidence or proof of them. _Abstract ghost code_, as such patterns are called, is helpful in quite a few cases, for example, when prototyping a theory or if some theorem is known to be true and we wish to depend on the result without redoing the proof.

![[Example – BTree – Ghost clone and flip]]

[[Example – BTree – Ghost clone and flip]] is an example of how abstract ghost code can be used. The first function, `@clone`, takes a `BTree` and produces a new one containing the same values. We _could_ write the implementation for `@clone`, but we can also choose not to if all we care about is satisfied by its postcondition. The function `@flip` intends to flip a `BTree` on its head and satisfies many of the same things as `@clone`; however, _cannot be implemented_ in general since flipping a binary tree is not within the capabilities of `BTree`, since the height of a `BTree` can never be negative.

To illustrate the effects of introducing unsound code in the form of abstract ghost functions, consider [[Example – BTree – Unsoundness]]. Here, the function aptly named `@unsoundess` takes a tree of non-zero height, flips it, and suddenly enters a world where everything is possible, highlighted by the non-failing assertion that $1=2$ $\lineref{87}$. If we were to remove the call `@flip(b)` $\lineref{86}$, the assertion would have failed as expected, but invoking the function allows us to believe in the postcondition of the function, that the height of `p` in-fact the negative of the height of `b`.

![[Example – BTree – Unsoundness]]



%%
```mist
struct Tuple[T, S] {
  fst: T,
  snd: S,
}

invariant[T] Tuple[T, T] {
  false
}
```


- https://dafny.org/latest/DafnyRef/DafnyRef#sec-function-by-method
- `ghost fn lemma() proof {}`
- Non-pure functions have opaque implementations
%%