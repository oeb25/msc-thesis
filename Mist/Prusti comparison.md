---
tags: paragraph
title: Prusti
---

is a verification frontend for Rust. [[@astrauskasLeveragingRustTypes2019]] Mist's and Prusti's overlap is quite considerable: both use Viper for verification, are written in Rust, and use a MIR CFG representation for lowering. What sets Prusti apart is that it is a verifier for an existing language (Rust) and is thus constrained to work within that environment. Mist, however, has the liberty to explore designs outside of these constraints but does not get to leverage the ecosystem that comes from integrating an existing compiler and community. [[@astrauskasPrustiProjectFormal2022]] Prusti uses _Place Capability Sets_ (PCS), which are almost isomorphic to Mist folding trees [[Folding tree structure]]. Prusti computes necessary PCS operations to satisfy constraints at different program points, as was done in [[Folding analysis]]. What we do, in addition, is formalize the analysis and procedure for this transformation. In a later paper, Dylan Wolff [[@Wolff2020ExtendedPC]] introduced _Extended Place Capabilities Summary_ (EPCS), formalizing PCS and EPCS and exposing them programmatically for other analyses to ingest.

%%
| Prusti | Mist |
|:---:|:-----------:|
| PCS | $\leaves$ |
| sub-place | (strict) $\prefix$ |
| _pack_ | $\fold$ |
| _unpack_ | $\unfold$ |
%%
