---
tags: example
---

The following listing contains an abridged version of the FIR for `fn compute(n: int)` from [[Figure – SumFac Program]].
```{.mist .ignoreErrors}
B0: r := struct SumFac { n: 0, sum: 0, fac: 1 } ;
    branch { B1 }
B1: use { i, m } ;
    branch { B2, B3, B4 }
//@ break
B2: use { r.n, i } ;
	branch { }
//@ align
B3: r.n := i ;
    r.sum := r.sum + i ;
    r.fac := r.fac * i ;
    branch { B1 }
//@ break
B4: use { r } ;
    branch { }
```

The basic-block `B0` \lineref{22 in [[Figure – SumFac Program]]} is the initialisation of the result `r`, containing one outgoing edge to `B1` \lineref{24 in [[Figure – SumFac Program]]} which is the condition-block for the loop. This block as three outgoing edges:

- `B2` is the loop invariant \lineref{25 in [[Figure – SumFac Program]]} and has no out-going edges.
- `B3` is the loop body \lineref{26 in [[Figure – SumFac Program]]} and has an edge back to `B1`.
- `B4` is the exit block \lineref{28 in [[Figure – SumFac Program]]}, which returns `r`, and branches nowhere.
