---
tags: figure
---

```ungram
Block = BlockId_ (Instruction ';')* 'branch' BlockId_*

Instruction = Place ':=' Expr_
			| 'use' Place*
            | 'fold' Place
            | 'unfold' Place

Place = Local_ Projection*

Projection = '.' Field_ // | '[' Local_ ']'

BlockId_ = '...'
Expr_ = '...'
Field_ = '...'
Local_ = '...'
```

> [!caption] BNF style grammar for FIR.
