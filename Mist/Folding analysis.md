---
tags: section
---

Having defined folding trees along with its operators and properties, we can begin to apply them in the context of programs.

The objective of the analysis, is to look at programs through a series of analyses, potentially perform some mutations through annotations, and end up with a new program which is _sound in terms of access described by foldings_.

The `SumFac` program, listed in [[Figure – SumFac Program]], will be used as the running example for motivating and explaining the concepts presented.

We start by defining the language and semantics in which folding occurs.

![[Figure – SumFac Program]]
