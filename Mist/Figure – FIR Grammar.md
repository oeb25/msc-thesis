---
tags: figure
---

```ungram
Block = BlockId_ (Instruction ';')* 'branch' BlockId_*

Instruction = Place ':=' Expr_
			| 'use' Place*
            | 'fold' Place
            | 'unfold' Place

Place = Slot_ Projection*

Projection = '.' Field_ // | '[' Slot_ ']'

BlockId_ = '...'
Expr_ = '...'
Field_ = '...'
Slot_ = '...'
```

> [!caption] BNF style grammar for FIR.
