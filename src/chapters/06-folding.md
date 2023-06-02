# Automatic folding of isorecursive structures {#chap:folding}

In Mist, types such as `struct`s and `enum`s are named, allowing them to be referenced inside other types, in function arguments, and local variables. In addition to being a collection of fields, they can also carry logical properties with `invariant`s. From a programmers perspective, these fields and properties can be accessed at any point in the program, without need for any annotation as to when a field or invariant is required. Due to this, we say that the Mist source language has _transparent_ types.^[Todo: This might be nominal types. Investigate further.]

The property of transparency, however, introduces an implicit guarantee about where the invariants of a type holds. This is tricky when you, for example, modify the internals of a struct in a sequence of operations, and part way through the mutation the invariants only partially hold. The lower layers of the compiler therefore works with _isorecursive_ types.

With isorecursive types there is a distinction between having named type and having an unfolded type. To read or update a field of a named type, it must first be _unfolded_. In contrast, when referencing to a named type, it must be _folded_. Whenever a type is folded, its invariant is held internally, and when unfolded that invariant can be subsequently assumed. For unfolded types, their invariant may or may not hold, but when the type is folded, its invariant is asserted.

An example of folding in action, can be seen in \cref{fig:breaking-invariant}. One interesting detail to highlight, is the fact that the invariant on `A` establishes a relationship between `.x` and `.y` \lineref{2}, that after performing the increment of `a.x` \lineref{5} is broken until the increment of `a.y` \lineref{6} has occured. This is fine, since the invariant is first required to hold at the folding point \lineref{7}.

::: {.figure #fig:breaking-invariant}

:::: {.subfigure width=0.49}

```{.mist .numberLines .ignoreErrors}
struct A { x: int, y: int }
invariant A { self.x == -self.y }
fn inc(a: &mut A) {

    a.x += 1;
    a.y -= 1;

}
```

::::

:::: {.subfigure width=0.49}


```{.mist .ignoreErrors}
struct A { x: int, y: int }
invariant A { self.x == -self.y }
fn inc(a: &mut A) {
    unfold a;
    a.x += 1;
    a.y -= 1;
    fold a;
}
```

::::

\caption{The left program is the input program, with implicit folding and unfolding. The right program is the same program, with with annotated foldings.}

:::

To transform a program from its equirecursive form into its isorecursive form, we must compute the nessesary foldings and unfolding. This is done through a series of analysis, which are covered in this chapter, but as a foundation for these analysis, we introduce a data structure and notation to reason about foldings.

## Folding tree structure

A folding tree is a data structure, denoted by $\T$, that maintains the folding state of places. It works by representing data types as a tree, where nodes are fields of potentially nested `struct`s. The leafs of the tree are all the places that are _folded_, while the internal nodes are _unfolded_ places, both uniquely described by paths from the root.

The data structure fundamentally supports two operations: $\unfold : \Rho \times T \to T$ and $\fold : \Rho \times T \to T$. The first operation, $\unfold(\rho, \T) = \T'$, requires $\rho$ to be folded in $\T$ (i.e. a leaf), denoted by $\leafin{\rho}{\T}$, and ensures that $\rho$ is unfolded in $\T'$ and consequently that all the fields of $\rho$ are accessible in $\T'$. On the other hand, $\fold(\rho, \T) = \T'$ is the inverse of $\unfold$, requiring that all fields of $\rho$ are folded in $\T$, and that none of them are accessible in $\T'$ and $\rho$ is. A demonstration of these operations are visualized in \cref{fig:folding-tree-folding-sequence}. Additionally, we let $\mathcal{F}$ be the space of all foldings and foldings.

::: {.figure #fig:folding-tree-folding-sequence}

:::: {.subfigure width=0.18}

```{.folding-tree root="$\\T$"}
{ x X y X }
```

::::

:::: {.subfigure width=0.18}

```{.folding-tree root="$\\unfold\\;.y$"}
{ x X y { a X b X } }
```

::::

:::: {.subfigure width=0.18}

```{.folding-tree root="$\\unfold\\;.y.a$"}
{ x X y { a { f X g X h X } b X } }
```

::::

:::: {.subfigure width=0.18}

```{.folding-tree root="$\\fold\\;.y.a$"}
{ x X y { a X b X } }
```

::::

:::: {.subfigure width=0.18}

```{.folding-tree root="$\\fold\\;.y$"}
{ x X y X }
```

::::

\caption{A sequence of unfoldings and foldings. Each successive tree operates on the previous one, where the operation performed is indicated by the root.}

:::

While the tree representation gives a good intution for the data structure, and will continue to be how folding trees are visualized, it is more convenient to encode them as a set of rooted-paths. With this, any place $\rho$ in the set $\T$ is a leaf and thus folded, if it is not a prefix of any other place in $\T$. This allows us to formally define the semantics of $\fold$ and $\unfold$:
$$
\begin{gathered}
\begin{prooftree}
    \hypo{\rho: (f_1, \dots, f_n)}
    \hypo{\leafin{\rho}{\T}}
    \infer2{\unfold(\rho, \T) = \T \cup \{ \rho.f_1, \dots, \rho.f_n \}}
\end{prooftree} \vspace{.5em}\\
\begin{prooftree}
    \hypo{\rho: (f_1, \dots, f_n)}
    \hypo{\forall f_i \in \{ f_1, \dots, f_n \} : \leafin{\rho.f_i}{\T}}
    \infer2{\fold(\rho, \T) = \T \setminus \{ \rho.f_1, \dots, \rho.f_n \}}
\end{prooftree}
\end{gathered}
$$
In this notation, $\rho : (f_1,\dots,f_n)$ says that $\rho$ has the fields $f_1,\dots,f_n$. Secondly, $\leafin{\rho}{\T}$ says that $\rho$ is a leaf in $\T$ and thus not a parent of any other element in $\T$, or formally:
$$
\leafin{\rho}{\T} = \rho \in \T \land \forall \rho'.f_i \in \T : \rho \neq \rho'
$$

A common operation on folding trees is transforming an existing tree into a new one with a desired place folded. To do so, a sequence of foldings and unfoldings must be performed to arrive at the desired tree. We call this operation _requires_ and use notation $\T \requires \rho$ to say that we want the tree $\T$ but with the minimal number of foldings and unfoldings to have $\rho$ be folded. To get an idea of the operation, see \cref{fig:folding-tree-requires-sequence}. The operation can be defined recursively like so:
$$
\T \requires \rho = \begin{cases}
    \fold(\rho, \T \requires \rho.f_1 \requires \dots \requires \rho.f_n) & \text{if } \rho : (f_1, \dots, f_n) \land \rho.f_1, \dots, \rho.f_n \in \T \\
    \unfold(\rho', \T \requires \rho') & \text{if } \rho \notin \T \land \rho'.f_i = \rho \\
    \T & \text{if } \leafin{\rho}{\T} \\
\end{cases}
$$

<!-- TODO: Perhaps this is useful later -->
<!-- $$
\T \hat{\requires} \rho.f_i = \begin{cases}
    \T & \text{if } \rho.f_i \in \T \\
    \T \requires \rho.f_i & \text{otherwise}
\end{cases}
$$ -->

::: {.figure #fig:folding-tree-requires-sequence}

:::: {.subfigure width=0.19}

```{.folding-tree root="$\\T$"}
{ x X y X }
```

::::

:::: {.subfigure width=0.19}

```{.folding-tree root="$\\requires .x.u$"}
{ x { u X v X } y X }
```

::::

:::: {.subfigure width=0.19}

```{.folding-tree root="$\\requires .y.a.g$"}
{ x { u X v X } y { a { f X g X h X } b X } }
```

::::

:::: {.subfigure width=0.19}

```{.folding-tree root="$\\requires .x$"}
{ x X y { a { f X g X h X } b X } }
```

::::

:::: {.subfigure width=0.19}

```{.folding-tree root="$\\requires .y$"}
{ x X y X }
```

::::

\caption{A sequence of folding trees showing how $\requires$ can perform the nessesary foldings for making fields accessible. Each successive tree operates on the previous one, where the operation performed is indicated by the root.}

:::

::: {.lemma}

The tree produced by $\T \requires \rho$ will have $\rho$ folded, that is $\leafin{\rho}{(\T \requires \rho)}$.

:::

::: {.proof}

By definition of $\leafin{}{}$ the statement $\leafin{\rho}{(\T \requires \rho)}$ is equivilant to showing two properties: $\rho \in (\T \requires \rho)$ and $\forall \rho'.f_i \in (\T \requires \rho) : \rho \neq \rho'$. We do so by case distinction on $\T \requires \rho$:

**Case 1** If $\rho : (f_1, \dots, f_n) \land \rho.f_1, \dots, \rho.f_n \in \T$ holds, then we know that $\T \requires \rho = \fold(\rho, \T \requires \rho.f_1 \requires \dots \requires \rho.f_n)$. Since we \fold $\rho$ as the last operation, we know that $\rho$ is folded in $\T \requires \rho$ and that $\rho \in (\T \requires \rho)$ holds. Additionally we know that the folding of $\rho$ removes $\rho.f_1, \dots, f_n$ from the tree, which means that the $\forall \rho'.f_i \in (\T \requires \rho) : \rho \neq \rho'$ holds.

**Case 2** If $\rho \notin \T \land \rho'.f_i = \rho$ then we have $\T \requires \rho = \unfold(\rho', \T \requires \rho')$. Given that we \unfold $\rho'$ and that $\rho$ is a field of $\rho'$, then $\rho$ must be in the new tree. Since $\rho \notin \T$, we know by the structure of $\T$ that any field of $\rho$ is not in $\T$, and since $\unfold$ of $\rho'$ only introduces fields of $\rho'$ and these fields are distinct from any field of $\rho$, we can conclude that $\forall \rho''.f_i \in (\T \requires \rho) : \rho \neq \rho''$.

**Case 3** Lastly, if $\leafin{\rho}{\T}$, then we know that $\T \requires \rho = \T$, and given that equality we can derive $\leafin{\rho}{(\T \requires \rho)}$, which is exatly what we wanted to show.

:::

Another property, which is useful for computing _greatest solutions_ discussed in +@sec:folding:folding-analysis, is the fact that the folding tree forms a lattice.

::: {.lemma #lemma:folding-tree-lattice}

The folding tree data structure forms a lattice with the following properties:

1. The bottom element $\bot$ is the tree where the maximal number of unfoldings are performed.
2. The top element $\top$ is the tree where the root is folded.
3. The ordering of the tree, given by $\T_1 \smaller \T_2$ satisfies the property that any leaf in $\T_2$ is in $\T_1$ but not necessarily folded:
    $$
    \T_1 \smaller \T_2 \Leftrightarrow \forall \leafin{\rho}{\T_2} : \rho \in \T_1
    $$
    and is defined by standard set inclusion, that is:
    $$
    \T_1 \smaller \T_2 = \T_1 \supseteq \T_2,
    $$
4. The join $\join$ of $\T_1$ and $\T_2$ ensures that all leafs $\rho$ of $\T_1 \join \T_2$ are leafs in either $\T_1$ or $\T_2$ and at least present in the other. More formally, the following equations describe the property:
    $$
    \begin{gathered}
    \rho \in (\T_1 \join \T_2) = \rho \in \T_1 \land \rho \in \T_2 \\
    \leafin{\rho}{(\T_1 \join \T_2)} = (\leafin{\rho}{\T_1} \land \rho \in \T_2) \lor (\leafin{\rho}{\T_2} \land \rho \in \T_1) \\
    \end{gathered}
    $$
    The tree given by $\T_1 \join \T_2$ is equal to $\T_1 \cap \T_2$.
5. The meet $\meet$ of $\T_1$ and $\T_2$ contains all folded and unfolded places of both $\T_1$ and $\T_2$. It satisfies the following equations:
    $$
    \begin{gathered}
    \rho \in (\T_1 \meet \T_2) = \rho \in \T_1 \lor \rho \in \T_2 \\
    \leafin{\rho}{(\T_1 \meet \T_2)} = (\leafin{\rho}{\T_1} \land (\leafin{\rho}{\T_2} \lor \rho \notin \T_2)) \lor (\leafin{\rho}{\T_2} \land (\leafin{\rho}{\T_1} \lor \rho \notin \T_1))
    \end{gathered}
    $$
    Opposite to to $\join$, the tree given by $\T_1 \meet \T_2$ is equal to $\T_1 \cup \T_2$.

:::

::: {.figure #fig:folding-join-meet}

:::: {.subfigure}

```{.folding-tree root="$\\T_1 \\join \\T_2$"}
{ x X y { a X b X } }
```

::::

:::: {.subfigure}

```{.folding-tree root="$\\T_1$"}
{ x { u X v X } y { a X b X } }
```

::::

:::: {.subfigure}

```{.folding-tree root="$\\T_2$"}
{ x X y { a { f X g X h X } b X } }
```

::::

:::: {.subfigure}

```{.folding-tree root="$\\T_1 \\meet \\T_2$"}
{ x { u X v X } y { a { f X g X h X } b X } }
```

::::

\caption{A visualization of the lattice operations described in \cref{lemma:folding-tree-lattice}, showing the result of $\T_1 \join \T_2$ and  $\T_1 \meet \T_2$.}

:::

More intuitively, the $\meet$ of two trees is the smallest unfolding that contains two trees, while $\join$ gives the largest tree that fits inside two trees.

To see a breif example of this in action, take $\T_1 \join \T_2$ and $\T_1 \meet \T_2$ shown in \cref{fig:folding-join-meet}, and let's consider the nessesary foldings and unfoldings to arrive at both $\T_1$ and $\T_2$. For $\T_1 \join \T_2$ to become $\T_1$ it requires $\unfold\;.x$ and to become $\T_2$ requires $\unfold\;.y.a$. On the other hand, starting at $\T_1 \meet \T_2$ and arriving at $\T_1$ and $\T_2$ requires $\fold\;.x$ and $\fold\;.y.a$ respectively. As indicated by this example, $\meet$ leads to foldings while $\join$ leads to unfoldings.

This leads us to the final bit of notation for foldings trees, which is computing the minimal foldings and unfoldings required to transform one tree into another. We let $\tinto$ be the function that computes the composition of foldings and unfoldings satisfying the following equation:
$$
\begin{aligned}
\tinto[\T_1, \T_2] = \;& \mathcal{F}_n \circ \dots \circ \mathcal{F}_1 \\
\text{such that } [&\mathcal{F}_n \circ \dots \circ \mathcal{F}_1](\T_1) = \T_2
\end{aligned}
$$
Using the above example, we can compute the foldings by way of \tinto:
$$
\begin{gathered}
\begin{aligned}
    \tinto[\T_1 \join \T_2, \T_1] &= \unfold\;.x &
    \tinto[\T_1 \join \T_2, \T_2] &= \unfold\;.y.a \\
    \tinto[\T_1 \meet \T_2, \T_1] &= \fold\;.x &
    \tinto[\T_1 \meet \T_2, \T_2] &= \fold\;.y.a \\
\end{aligned} \\
\begin{aligned}
    \tinto[\T_1 \meet \T_2, \T_1 \join \T_2] &= \fold\;.x \circ \fold\;.y.a \\
    \tinto[\T_1, \T_2] &= \fold\;.x \circ \unfold\;.y.a \\
\end{aligned}
\end{gathered}
$$

::: {.lemma #lemma:tinto-anticommute}

The function $\tinto$ is _anticommutative_:
$$\tinto[\T_1, \T_2] = \tinto[\T_2, \T_1]^{-1}$$

:::

::: {.proof}

Let $\T_1$ and $\T_2$ be arbitrary folding trees, and recall that $\mathcal{F}_1 \circ \mathcal{F}_2 = (\mathcal{F}_2^{-1} \circ \mathcal{F}_1^{-1})^{-1}$, then:
$$
\begin{aligned}
    \T_1 &= \tinto[\T_1, \T_2]^{-1}(\tinto[\T_1, \T_2](\T_1)) \\
         &= \tinto[\T_1, \T_2]^{-1}(\T_2) \\
         &= [\mathcal{F}_n \circ \dots \circ \mathcal{F}_1]^{-1}(\T_2) \\
         &= [\mathcal{F}_1^{-1} \circ \dots \circ \mathcal{F}_n^{-1}](\T_2) \\
         &= \tinto[\T_2, \T_1](\T_2) \\
\end{aligned}
$$
From this we get that $\T_1 = \tinto[\T_2, \T_1](\tinto[\T_1, \T_2](\T_1))$, showing that $\tinto[\T_2, \T_1]$ is the inverse of $\tinto[\T_1, \T_2]$.

:::

With folding trees defined, we can continue to look at how they relate to programs written in Mist.

\vfill

<!-- 2. $\tinto[\T_1 \join \T_2, \T_3] = \tinto[\T_1 \meet \T_2, \T_3]^{-1}$ -->

<!-- In particular, $\join$ is useful for minimizing the number of foldings, in turn minimizing the number of invariant assertions as described in the introduction of +@chap:folding.  -->

<!-- $$
\begin{array}{ccc}
\begin{prooftree}
\hypo{\rho \in \T}
\infer1{\T \requires \rho = \T}
\end{prooftree}
&
\begin{prooftree}
\hypo{\prefixes(\rho) \cap \T = \varnothing}
\hypo{\rho : (f_1, f_2, \dots, f_n)}
\infer2{\T \requires \rho = \fold(\rho, \T \requires \rho.f_1 \requires \rho.f_2 \requires \dots \requires \rho.f_n)}
\end{prooftree}
&
\begin{prooftree}
\hypo{\parents(\rho) \cap \T \neq \varnothing}
\infer1{\T \requires \rho.f_i = \unfold(\rho, \T \requires \rho)}
\end{prooftree}
\end{array}
$$
$$
\begin{array}{cc}
\begin{prooftree}
\hypo{\rho' \in \T}
\hypo{\rho.f_i \in \prefixes(\rho')}
\infer2{\T \requires \rho.f_i^* = \T}
\end{prooftree}
&
\begin{prooftree}
\hypo{\forall \rho' \in \T : \rho.f_i \notin \prefixes(\rho')}
\infer1{\T \requires \rho.f_i^* = \T \requires \rho.f_i}
\end{prooftree}
\end{array}
$$ -->

<!-- $$
T_1 \smaller T_2 = \forall \rho \in T_1 : \prefixes(\rho) \cap T_2 \neq \varnothing
$$
$$
\begin{aligned}
T_1 \join T_2 =
    &\{ \rho \mid \rho \in T_1 \land \texttt{parents}(\rho) \cap T_2 = \varnothing \} \; \cup \\
    &\{ \rho \mid \rho \in T_2 \land \texttt{parents}(\rho) \cap T_1 = \varnothing \}
\end{aligned}
$$ -->

## Operational language

In +@chap:compiler we introduced the multi-stage structure of the Mist compiler, and specifically the MIR representation which is a CFG. In terms of computing foldings and unfoldings, this is the stage we are concerned with, but instead of considering the full set of instructions and terminators, we consider a smaller varient which focuses much more concretly on _how and what_ places are used, and omit the details for performing actual computation or verification.

```{.ungram #fig:minimal-language}
// The grammar for the minimal language

Instruction = Place ':=' Expr
            | ('assert' | 'assume') Expr
            | 'fold' Place
            | 'unfold' Place
            | 'mention' Place

Expr = Operand
     | BorrowKind Place
     | BinaryOp_ Operand Operand
     | UnaryOp_ Operand

BorrowKind = '&' | '&mut'

Operand = 'copy' Place | 'move' Place | Literal_

Place = Slot_ Projection*

Projection = '.' Field_ | '[' Slot_ ']'

BinaryOp_ = '...'
Field_ = '...'
Literal_ = '...'
Mutable_ = '...'
Shared_ = '...'
Slot_ = '...'
UnaryOp_ = '...'
```

## Typing rules with relation to access

$$
\begin{prooftree}
    \hypo{\rho \in \T}
    \hypo{\texttt{read}(a) \subseteq \T}
    \infer2{\T \vdash \rho := a}
\end{prooftree}
$$
$$
\begin{array}{ccc}
\begin{prooftree}
    \hypo{\texttt{read}(a) \subseteq \T}
    \infer1{\T \vdash \texttt{assert } a}
\end{prooftree}
&
\begin{prooftree}
    \hypo{\texttt{read}(a) \subseteq \T}
    \infer1{\T \vdash \texttt{assume } a}
\end{prooftree}
&
\begin{prooftree}
    \hypo{\rho \in \T}
    \infer1{\T \vdash \texttt{mention } \rho}
\end{prooftree}
\end{array}
$$

## Abstract interpretation approach

$$
\alpha(\sigma) \smaller \T \Leftrightarrow \sigma \smaller \gamma(\T)
$$

$$
\fsem{\omega}(\T) = \alpha(\sem{\omega}(\gamma(\T))) \text{ or } \fsem{\omega} = \alpha \circ \sem{\omega} \circ \gamma
$$

## Semantics

$$
\begin{prooftree}
    \hypo{\rho \in \dom(\sigma)}
    \hypo{\texttt{read}(a) \subseteq \dom(\sigma)}
    \hypo{\alpha(\sigma) \smaller \T}
    \hypo{\T \vdash \rho := a}
    \infer4{\sem{\rho := a}(\sigma) = \sigma[\rho \leftarrow a]}
\end{prooftree}
$$

$$
\begin{prooftree}
    \hypo{\texttt{read}(a) \subseteq \dom(\sigma)}
    \infer1{\sem{\texttt{assert } a}(\sigma) = \sigma}
\end{prooftree}
$$

$$
\begin{prooftree}
    \hypo{\rho \in \dom(\sigma)}
    \hypo{\alpha(\sigma) \smaller \T}
    \hypo{\T \vdash \texttt{fold } \rho}
    \hypo{\alpha(\sigma') \smaller \T'}
    \hypo{\fold(\rho, \T) = \T'}
    \infer5{\sem{\texttt{fold } \rho}(\sigma) = \sigma'}
\end{prooftree}
$$

$$
\begin{prooftree}
    \hypo{\rho \in \dom(\sigma)}
    \hypo{\alpha(\sigma) \smaller \T}
    \hypo{\T \vdash \texttt{unfold } \rho}
    \hypo{\alpha(\sigma') \smaller \T'}
    \hypo{\unfold(\rho, \T) = \T'}
    \infer5{\sem{\texttt{unfold } \rho}(\sigma) = \sigma'}
\end{prooftree}
$$

## Abstract semantics

$$
\fsem{\omega} : T \to T
$$

$$
\fsem{\omega}(\A(\phi)) = \A(\phi')
$$

$$
\bsem{\omega}(\fsem{\omega}(\T)) = \T
$$

$$
\begin{aligned}
\fsem{\omega_1; \omega_2}(\A(\phi_1))
    &= \fsem{\omega_2}(\tinto[\A(\phi_1'),\A(\phi_2)]\fsem{\omega_1}(\A(\phi_1))) \\
    &= \fsem{\omega_2}(\tinto[\A(\phi_1'),\A(\phi_2)]\A(\phi_1')) \\
    &= \fsem{\omega_2}(\A(\phi_2)) \\
    &= \A(\phi_2') \\
\end{aligned}
$$

::::: {.figure #fig:analysis-cfg}

:::::: {.subfigure width=0.35}

\begin{tikzpicture}[
    auto,
    box/.style = {rounded corners,color=white,fill=Teal600,
        align=center,inner sep=4mm},
    pp/.style = {align=center},
    tr/.style = {align=center, font=\footnotesize, color=Teal900},
]
    \begin{scope}[node distance=20mm and 5mm]
        \node[box] (b1)                  {$\omega_1$};
        \node[box] (b2) [below=of b1]    {$\omega_2$};
        \node[box] (b3) [below=of b2]    {$\omega_3$};
        \node[box] (b4) [right=of b3]    {$\omega_4$};
    \end{scope}
    \begin{scope}[node distance=-.5mm]
        \node[pp] (pp1) [above=of b1]    {$\phi_1$};
        \node[pp] (pp1') [below=of b1,xshift=3mm]   {$\phi_1'$};
        \node[pp] (pp2) [above=of b2,xshift=3mm]    {$\phi_2$};
        \node[pp] (pp2') [below=of b2,xshift=3mm]   {$\phi_2'$};
        \node[pp] (pp3) [above=of b3,xshift=3mm]    {$\phi_3$};
        \node[pp] (pp3') [below=of b3,xshift=3mm]   {$\phi_3'$};
        \node[pp] (pp4) [above=of b4,xshift=3mm]    {$\phi_4$};
        \node[pp] (pp4') [below=of b4]   {$\phi_4'$};
    \end{scope}

    \begin{scope}[rounded corners,color=Teal600,-latex]
        \draw (b1) -- node[tr] {$\delta(\phi_1', \phi_2)$} (b2);
        \draw (b2) -- node[tr,yshift=-1.5mm] {$\delta(\phi_2', \phi_3)$} (b3);
        \draw (b2.south) |- ([yshift=13mm]b4.north) -- node[tr,yshift=2mm] {$\delta(\phi_2', \phi_4)$} (b4.north);
        \draw (b3.south) -- ++(0,-3mm) -| ([xshift=-3mm]b2.west) node[tr,left] {$\delta(\phi_3', \phi_2)$} |- ([yshift=4mm]b2.north) -- (b2.north);
    \end{scope}
\end{tikzpicture}

::::::

:::::: {.subfigure width=0.55}

$$
\begin{aligned}
    \A(\phi_1) &\larger \bsem{\omega_1}(\A(\phi_2)) \\
    \A(\phi_2) &\larger \bsem{\omega_2}(\A(\phi_3)) \\
    \A(\phi_2) &\larger \bsem{\omega_2}(\A(\phi_4)) \\
    \A(\phi_3) &\larger \bsem{\omega_3}(\A(\phi_2)) \\
\end{aligned}
$$
$$
\begin{gathered}
\bsem{{\color{Teal900} \delta(\phi_i, \phi_j)}}(\A(\phi_j)) = \tinto[\A(\phi_j), \A(\phi_i)](\A(\phi_j)) \\
\bsem{\omega_i}(\A(\phi_i')) = \A(\phi_i)
\end{gathered}
$$

::::::

\caption{An abstract CFG with annotated program positions and constraints.}

:::::

<!-- $$
\fsem{\delta(\phi_i, \phi_j)}(\A(\phi_i)) = \tinto[\A(\phi_i), \A(\phi_j)](\A(\phi_i))
$$ -->

$$
\begin{prooftree}
    % \hypo{\A(\phi_i) \smaller \alpha(\sigma)}
    \infer0{\A(\phi_i) \vdash \sem{\omega_i}(\sigma)}
\end{prooftree}
$$

### Properties of semantics

$$
\begin{prooftree}
    \hypo{\sem{\omega}(\sigma) = \sigma'}
    \hypo{\T = \fsem{\omega}(\T')}
    \infer2{\ssem{\omega}(\sigma, \T) = \langle \sigma', \T' \rangle}
\end{prooftree}
$$
$$
\begin{prooftree}
    \hypo{\ssem{\omega_1}(\sigma, \T) = \langle \sigma', \T' \rangle}
    \hypo{\ssem{\omega_2}(\sigma', \T') = \langle \sigma'', \T'' \rangle}
    \infer2{\ssem{\omega_1 ; \omega_2}(\sigma, \T) = \langle \sigma'', \T'' \rangle}
\end{prooftree}
$$

$$
\begin{array}{ccc}
\begin{prooftree}
    \hypo{\texttt{read}(a) \subseteq \T}
    \hypo{x \in \T'}
    \infer2{\ssem{x := a}(\sigma, \T) = \langle \sigma[x = a], \T' \rangle}
\end{prooftree}
&
\begin{prooftree}
    \hypo{\rho.f_i^* \in \T}
    \hypo{\texttt{read}(a) \subseteq \T}
    \hypo{\rho.f_i \in \T'}
    \infer3{\ssem{\rho.f_i := a}(\sigma, \T) = \langle \sigma[\rho.f_i = a], \T' \rangle}
\end{prooftree}
\end{array}
$$
$$
\begin{array}{ccc}
\begin{prooftree}
    \hypo{S = (f_1, f_2, \dots, f_n)}
    \hypo{\rho: S \in \T}
    \infer2{\ssem{\unfold \; \rho}(\sigma, \T) = \langle \sigma, \unfold(\rho, \T) \rangle}
\end{prooftree}
&
\begin{prooftree}
    \hypo{\rho: (f_1, f_2, \dots, f_n)}
    \hypo{\rho.f_1, \rho.f_2, \dots, \rho.f_n \in \T}
    \infer2{\ssem{\fold \; \rho}(\sigma, \T) = \langle \sigma, \fold(\rho, \T) \rangle}
\end{prooftree}
\end{array}
$$

$$
\begin{prooftree}
    \hypo{\sem{\omega}(\sigma) = \sigma'}
    \hypo{\T= \fsem{\omega}(\T')}
    \infer2{\ssem{\texttt{enhance}(\omega)}(\sigma, \T) = \langle \omega', \T' \rangle}
\end{prooftree}
$$

### Analysis functions

$$
\begin{prooftree}
    \hypo{x \in \dom(\T)}
    \hypo{\texttt{read}(a) \subseteq \dom(\T)}
    \infer2{\fsem{x := a}(\T) = \T \hat{\requires} \texttt{read}(a)}
\end{prooftree}
$$
$$
\begin{prooftree}
    \hypo{\rho.f_i \in \dom(\T)}
    \hypo{\texttt{read}(a) \subseteq \dom(\T)}
    \infer2{\fsem{\rho.f_i := a}(\T) = \T \requires \rho.f_i^* \hat{\requires} \texttt{read}(a)}
\end{prooftree}
$$

$$
\begin{prooftree}
    \hypo{\texttt{read}(a) \subseteq \dom(\T)}
    \infer1{\fsem{\texttt{assert } a}(\T) = \T \hat{\requires} \texttt{read}(a)}
\end{prooftree}
$$

### Proof

$$
\begin{prooftree}
    \infer0{\fsem{\omega}(\T) = \T' \implies \sem{\omega}(\sigma) = \sigma'}
\end{prooftree}
$$

$$
\begin{prooftree}
    \hypo{\sem{\omega_2}(\sem{\omega_1}(\sigma)) = \sigma'}
    \infer1{\sem{\omega_1 ; \omega_2}(\sigma) = \sigma'}
\end{prooftree}
$$


## Challanges

```mist
struct T { f: int }

fn f(x: &T) {
    let y = x;
    assert x.f == y.f;
}
```

## The folding analysis {#sec:folding:folding-analysis}
