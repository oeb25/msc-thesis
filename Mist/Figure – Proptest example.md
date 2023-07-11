---
tags: figure
---

```{.mist .ignoreErrors .numberLines}
prop_compose! {
  fn arb_folding_tree()
    (places in prop::collection::vec(arb_place(), 0..10))
    -> FoldingTree
  {
    let mut tree = FoldingTree::default();
    for p in places {
      let _ = tree.require(p);
    }
    tree
  }
}
proptest! {
  #[test]
  fn folding_tree_join_commute(
    t1 in arb_folding_tree(),
    t2 in arb_folding_tree(),
  ) {
    let t1_join_t2 = t1.join(&t2);
    let t2_join_t1 = t2.join(&t1);

    prop_assert!(t1_join_t2 == t2_join_t1);
  }
}
```

> [!caption]
> A simplified extract of a `proptest` test case from the Mist compiler.
