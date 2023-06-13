---
tags: figure
---

```ungram
Instruction = Place ':=' Expr
            | 'reference' Expr
            | 'mention' Place
            | 'fold' Place
            | 'unfold' Place

Expr = Operand*
     | BorrowKind Place

BorrowKind = '&' | '&mut'

Operand = 'copy' Place | 'move' Place

Place = Slot_ Projection*

Projection = '.' Field_ // | '[' Slot_ ']'

BinaryOp_ = '...'
Field_ = '...'
Mutable_ = '...'
Shared_ = '...'
Slot_ = '...'
UnaryOp_ = '...'
```

> [!caption] BNF style grammar for FIR.
