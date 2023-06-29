---
tags: subsection
---

In [[Compilation stages]], we introduced the multi-stage structure of the Mist compiler, and specifically the MIR representation (see [[Mid-level IR (MIR)]]). In terms of computing folds and unfolds, this is the stage we are considering. However, instead of considering the complete set of instructions and terminators, we consider a smaller variant that focuses on _how and what_ places are used and omit the details for performing actual computation or verification.

![[Definition – FIR]]

![[Figure – FIR Grammar]]

Although FIR is defined as a separate set of constructs from MIR, it is, in fact, entirely derivable from MIR, leading it to be a projection. The details from MIR that folding analysis is concerned with are read and written places in instructions and terminators.

Like MIR, FIR is a CFG with basic blocks being a sequence of instructions. Each block in FIR ends with a set of blocks that could potentially be branched to; these form the edges in the CFG.

FIR blocks consists of four kinds of abstract instructions, the first instruction being an amalgamation of all read and write instructions of MIR, namely $\langle Place \rangle := \langle Expr \rangle$, also denoted by $\rho := a$, where $a$ is the expression whose result is written to $\rho$. For the most part, the actual value of $a$ is unimportant since the only thing folding considers are the _places mentioned_, which we refer to by $\pread(a)$. The next instruction $\iuse\; \langle Place \rangle^*$, denoted by $\iuse\;\Rho$, represents MIR instructions where a set of places $\Rho$ are read, but nothing is written to.

%%
> [!example]
> The the following snippet, the left will be Mist statements, and the right will be their FIR equivalent.
> ```{.mist .ignoreErrors}
> let b = s.x.v + s.x.u * 5;
>
> if b > 3 {
>
>   inc(s);
>
> }
> //@ align
> B1: tmp := s.x.v + s.x.u ;
>     b := tmp * 5 ;
>     use { b } ;
>     branch { B2, B3 }
> B2: use { inc, s } ;
>     branch { B3 }
> B3: branch { }
> ```
> The program contains three basic blocks, the first containing the first three instructions, which jumps to the next containing `use { inc, s }`. The last basic block is the
>
> Where $\pread$ on the two expressions produces the following:
> $$
> \begin{aligned}
> \pread(\texttt{s.x.v + s.x.u}) &= \{\texttt{s.x.v}, \texttt{s.x.u}\}, \\
> \pread(\texttt{tmp * 5}) &= \{\texttt{tmp}\}. \\
> \end{aligned}
> $$
%%

The remaining two instructions, `fold` and `unfold`, serve a similar purpose as those defined in [[Definition – Folding]] but as FIR instructions have _no impact on the execution of the program_ and are used solely during folding analysis.

![[Example – SubFib FIR]]
