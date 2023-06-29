---
tags: definition
---

We define the Folding-level IR (FIR) to be a projection of MIR which encodes places written to and places read from, into a sequence of instructions.

FIR is composed of blocks of FIR instructions, which are connected by unconditional (potentially branching and cyclic) jumps terminating the blocks.

The grammar for FIR is defined in [[Figure – FIR Grammar]].
