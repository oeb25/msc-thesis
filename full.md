
# Introduction {#introduction}

## Contributions {#contributions}
- We present in \cref{the-mist-programming-language} a new programming language, Mist, which supports formal verification constructs with a high emphasis on usability.
- We present in \cref{the-mist-compiler} an implementation of the compilation infrastructure for Mist based on incremental compilation and verification, with an accompanying implementation of the Language Server Protocol.
- In \cref{automatic-folding-of-isorecursive-structures} we formalize an algorithm for computing folding- and unfolding-points for isorecursive structures, and give a formally verified implementation of the algorithm in Mist.

We conclude with related and future work in \cref{conclusion}.


# The Mist Programming Language {#the-mist-programming-language}
- Mist is a Rust deriviative with Viper concepts, which is to say that it looks like Rust, but without many features including life-times and traits, and with additional annotations to specify logical properties and invariants.
- The grammar for the language is in \cref{appendix-the-mist-grammar}.
- Supports limited type inference, requiring method signatures to be annotated, but with full inference inside of method bodies.
- It has an ownership system, which is checked using quantifiable permissions in Viper.
- Mist uses a _norminal type system_ [see @pierceTypesProgrammingLanguages2002, pp. 251-54].


## Types in Mist {#types-in-mist}

# The Mist Compiler {#the-mist-compiler}
The compiler for Mist is written in Rust and is heavily inspired by the architecture of both `rustc` and `rust-analyzer`.


## Compilation stages {#compilation-stages}


::: {.figure #figure-complication-pipeline}

```tikz
\usetikzlibrary{positioning}
\usetikzlibrary{fit,backgrounds}
\usetikzlibrary{patterns}

\pgfdeclarepatternformonly{soft crosshatch}{\pgfqpoint{-1pt}{-1pt}}{\pgfqpoint{4pt}{4pt}}{\pgfqpoint{3pt}{3pt}}%
{
  \pgfsetstrokeopacity{0.3}
  \pgfsetlinewidth{0.4pt}
  \pgfpathmoveto{\pgfqpoint{3.1pt}{0pt}}
  \pgfpathlineto{\pgfqpoint{0pt}{3.1pt}}
  \pgfpathmoveto{\pgfqpoint{0pt}{0pt}}
  \pgfpathlineto{\pgfqpoint{3.1pt}{3.1pt}}
  \pgfusepath{stroke}
}

\begin{document}
\begin{tikzpicture}[
    auto,
    node distance=1cm and 1.6cm,
    wrapper/.style 2 args = {
	    local bounding box=localbb,
	    execute at end scope = {
		    \begin{scope}
			    \node[
				    inner sep = #1,
				    fit=(localbb),
				    draw,
				    dashed,
				    rounded corners,
			    ] (title) {};
			    \node[above=of title,xshift=-0.3cm,yshift=-1.1cm,fill=white] {CST};
		    \end{scope}
	    }
    },
    stage/.style = {
	    draw,
	    rounded corners,
	    inner sep = 1.5em,
    },
    trans/.style = {
	    ->,
    },
    mod/.style = {
	    fill=white,
	    font=\footnotesize,
    },
    transl/.style = {
	    align=center,
	    font=\footnotesize,
	    midway,
	    %below,
	    %yshift=-.9cm,
    },
    vprbk/.style = {
	    draw,
	    rounded corners,
    },
]
	\begin{scope}[
		local bounding box=mist-syntax,
		execute at end scope={
			\begin{scope}
				\node[
					draw,
					dotted,
					fit=(mist-syntax),
				    rounded corners,
				] (syntax) {};
				\node[mod,above=of syntax,xshift=-1.5cm,yshift=-1.3cm] {\texttt{mist-syntax}};
			\end{scope}
		}
	]
		\node[stage] (source) {Source};
		\begin{scope}[wrapper={1ex}{Wrapper}]
		    \node[stage, right=of source] (ast) {AST};
	    \end{scope}
	\end{scope}
	\begin{scope}[
		local bounding box=mist-core,
		execute at end scope={
			\begin{scope}
				\node[
					draw,
					dotted,
					fit=(mist-core),
				    rounded corners,
				    inner sep = 1ex,
				] (mist-core) {};
				\node[mod,above=of mist-core,xshift=-1.5cm,yshift=-1.1cm] {\texttt{mist-core}};
			\end{scope}
		}
	]
	    \node[stage, right=of ast] (hir) {HIR};
	    \node[stage, right=of hir] (mir) {MIR};
	\end{scope}
	\node[stage, right=of mir, xshift=-1cm] (wasm) {WASM};

	\begin{scope}[
		local bounding box=viperserver,
		execute at end scope={
			\begin{scope}
				\node[
					draw,
					dotted,
					fit=(viperserver),
				    rounded corners,
				    inner sep = 2ex,
				] (viperserver) {};
				\node[mod,above=of viperserver,yshift=-1.1cm] {\texttt{mist-codegen-viper}};
			\end{scope}
		}
	]
		\node[stage, below=of mir] (vpr) {\texttt{silver}};
	\end{scope}

	\begin{scope}[
		local bounding box=viperserver,
		execute at end scope={
			\begin{scope}
				\node[
					draw,
					dotted,
					fit=(viperserver),
				    rounded corners,
				    inner sep = 2ex,
				] (viperserver) {};
				\node[mod,above=of viperserver,xshift=-0cm,yshift=-1.1cm] {\texttt{viperserver}};
			\end{scope}
		}
	]
		\node[vprbk, below=of hir] (carbon) {\texttt{carbon}};
		\node[vprbk, below=of carbon, yshift=.6cm] (silicon) {\texttt{silicon}};
	\end{scope}

	\begin{scope}[
		local bounding box=viperserver,
		execute at end scope={
			\begin{scope}
				\node[
					draw,
					dotted,
					fit=(viperserver),
				    rounded corners,
				    inner sep = 2ex,
				] (viperserver) {};
				\node[mod,above=of viperserver,xshift=-.9cm,yshift=-1.1cm] {\texttt{mist-lsp}};
			\end{scope}
		}
	]
		\node[stage, below=of ast] (lsp) {Diagnostics};
	\end{scope}
	
	\draw[trans] (source) -- (ast) node[transl] {Parsing};
    \draw[trans] (title) -- (hir) node[transl] {Type checking};
    \draw[trans] (hir) -- (mir) node[transl] {Lowering};
    
    \draw[trans] (mir) -- (wasm) node[transl] {};
    \draw[trans] (mir) -- (vpr) node[transl] {};
	
	\draw[trans] (ast) -- (lsp) node[transl] {};
	\draw[trans] (hir) -- (lsp) node[transl] {};
	
	\draw[trans] (vpr) -- (carbon) node[transl] {};
	\draw[trans] (vpr) -- (silicon) node[transl] {};
	\draw[trans] (carbon) -- (lsp) node[transl] {};
	\draw[trans] (silicon) -- (lsp) node[transl] {};
\end{tikzpicture}
\end{document}
```

> [!caption]
> The compilation pipeline visualized.

:::


During compilation of a Mist program, it goes through many stages, each with their own representation and unique properties. Each stage is computed based on information gathered in the previous stages.


### Syntax trees {#syntax-trees}
All Mist programs start with the source code, the language described in \cref{the-mist-programming-language}, which then gets parsed using a hand-written recursive descent parser. For reasons further explained in \cref{language-server-integration}, it is important that the parser retains all information in the source code when constructing the next stage of compilation.

The first few stages of compilation, the ones revolving around the syntax, uses a _red-green tree_ approach to representation of the syntax-tree, which was first introduced in the Roslyn C# compiler (see @ericlippertPersistenceFacadesRoslyn2012). It splits the syntax tree into two stages: one called the _green tree_ (also referred to as the _concrete-syntax tree_, see \cref{concrete-syntax-tree-cst}) and one called the _red tree_ (also referred to as the _astract-syntax tree_, see \cref{abstract-syntax-tree-ast}).


#### Concrete-syntax tree (CST) {#concrete-syntax-tree-cst}
The concrete-syntax tree is a full-fidelity representation over source code, meaning no information about the original code is lost. Conceptually, the goal of a CST is to provide a semi-structured view into the source code, by storing a hierarchy of syntax-nodes and tokens, which maintains a complete mapping back to the original source code.

The job of the parser is to construct such a CST, by consuming tokens are adding nodes to the tree. Internally the CST maintains the _kind_ of syntax-nodes and tokens inserted by the parser, but does nothing to prevent the parser from emitting malformed language constructs. This is by design, as we want the CST to be able to contain partial or invalid syntax. Doing so, allows the compiler to optimistically continue its compilation process, even when the source program contains syntax errors and/or is actively being written.

Relating the CST to the grammar (see \cref{appendix-the-mist-grammar}) node kinds represent productions and terminals for syntax-nodes and tokens respectively. For all syntactically valid programs, the corresponding CST will have a structure conforming the to the hierarchy described in the grammar. But for invalid programs, children of nodes might have a mixture of allowed and disallowed syntax kinds.

This means that all queries performed on the CST must account of the partial nature of the tree, and thus the caller must consider both cases where the expected node exists, and where it does not.



::: {.figure #figure-cst-example}

> [!subfigure|width=0.4]
> ```mist
> fn main() -> { let x = 5 }
> ```

```{=tex}
\\[1.5em]
```

```tikz
\usetikzlibrary{trees}

\definecolor{Teal50}{rgb} {0.9411765,0.99215686,0.98039216}
\definecolor{Teal100}{rgb} {0.8,0.9843137,0.94509804}
\definecolor{Teal200}{rgb} {0.6,0.9647059,0.89411765}
\definecolor{Teal300}{rgb} {0.36862746,0.91764706,0.83137256}
\definecolor{Teal400}{rgb} {0.1764706,0.83137256,0.7490196}
\definecolor{Teal500}{rgb} {0.078431375,0.72156864,0.6509804}
\definecolor{Teal600}{rgb} {0.050980393,0.5803922,0.53333336}
\definecolor{Teal700}{rgb} {0.05882353,0.4627451,0.43137255}
\definecolor{Teal800}{rgb} {0.06666667,0.36862746,0.34901962}
\definecolor{Teal900}{rgb} {0.07450981,0.30588236,0.2901961}
\definecolor{Teal950}{rgb} {0.015686275,0.18431373,0.18039216}

\definecolor{Slate50}{rgb} {0.972549,0.98039216,0.9882353}
\definecolor{Slate100}{rgb} {0.94509804,0.9607843,0.9764706}
\definecolor{Slate200}{rgb} {0.8862745,0.9098039,0.9411765}
\definecolor{Slate300}{rgb} {0.79607844,0.8352941,0.88235295}
\definecolor{Slate400}{rgb} {0.5803922,0.6392157,0.72156864}
\definecolor{Slate500}{rgb} {0.39215687,0.45490196,0.54509807}
\definecolor{Slate600}{rgb} {0.2784314,0.33333334,0.4117647}
\definecolor{Slate700}{rgb} {0.2,0.25490198,0.33333334}
\definecolor{Slate800}{rgb} {0.11764706,0.16078432,0.23137255}
\definecolor{Slate900}{rgb} {0.05882353,0.09019608,0.16470589}
\definecolor{Slate950}{rgb} {0.007843138,0.023529412,0.09019608}

\newcommand{\synode}[2]{{\color{Teal700}\texttt{#1}}\\\texttt{#2}}
\newcommand{\sytoken}[3]{{\color{Slate700}\texttt{#1}}\\{\color{Teal600}\texttt{#3}}}

\begin{document}
\begin{tikzpicture}[
	scale=0.5,
	font=\footnotesize,
	align=center,
	edge from parent fork down,
	color=Slate400,
	token/.style={font=\footnotesize},
	snode/.style={font=\footnotesize},
	level distance=2.75cm,
	level 1/.style={sibling distance=25mm},
	level 2/.style={sibling distance=25mm},
	level 3/.style={sibling distance=25mm},
]
\node[snode] {\synode{FN}{0..26}}
		child {node[token,xshift=1.5em] {\sytoken{FN\_TOKEN}{0..1}{"fn"}}}
		child {node[token,xshift=1em] {\sytoken{WS}{0..1}{" "}}}
		child {node[snode] {\synode{NAME}{0..4}}
			child {node[token] {\sytoken{IDENT}{0..4}{"main"}}}
		}
		child {node[snode,xshift=1.5em] {\synode{PARAM\_LIST}{0..2}}
			child {node[token] {\sytoken{L\_PAREN}{0..1}{"("}}}
			child {node[token] {\sytoken{R\_PAREN}{0..1}{")"}}}
		}
		child {node[token,xshift=1.5em] {\sytoken{WS}{0..1}{" "}}}
		child {node[token,xshift=1.5em] {\sytoken{THIN\_ARROW}{0..1}{"->"}}}
		child {node[token,xshift=1.5em] {\sytoken{WS}{0..1}{" "}}}
		child {node[snode,xshift=1.2em] {\synode{BLOCK\_EXPR}{0..12}}
			child {node[token] {\sytoken{L\_CURLY}{0..1}{"\{"}}}
			child {node[token] {\sytoken{WS}{0..1}{" "}}}
			child {node[snode] {\synode{LET\_STMT}{0..9}}
				child {node[token] {\sytoken{LET\_KW}{0..3}{"let"}}}
				child {node[token] {\sytoken{WS}{0..1}{" "}}}
				child {node[snode] {\synode{NAME}{0..1}}
					child {node[token] {\sytoken{IDENT}{0..1}{"x"}}}
				}
				child {node[token] {\sytoken{WS}{0..1}{" "}}}
				child {node[token] {\sytoken{EQ}{0..1}{"="}}}
				child {node[token] {\sytoken{WS}{0..1}{" "}}}
				child {node[snode] {\synode{LITERAL}{0..1}}
					child {node[token] {\sytoken{INT\_NUMBER}{0..1}{"5"}}}
				}
			}
			child {node[token] {\sytoken{WS}{0..1}{" "}}}
			child {node[token] {\sytoken{R\_CURLY}{0..1}{"\}"}}}
		}
;
\end{tikzpicture}
\end{document}
```

> [!caption]
> A simple Mist program, with its CST displayed below where upper case words refer to the syntax kind, the numbers separated by `..` indicate the span in the source code, and syntax-nodes have their children indented while terminals have the string they span in double-quotes.

```
  × parse error: specify the type here
   ╭────
 1 │ fn main() -> { let x = 5 }
   ·              ┬
   ·              ╰── here
   ╰────


  × parse error: expected ';'
   ╭────
 1 │ fn main() -> { let x = 5 }
   ·                        ┬
   ·                        ╰── help: add ';' here
   ╰────
```


:::




#### Abstract-syntax tree (AST) {#abstract-syntax-tree-ast}
The _abstract-syntax tree_ layer provides a fully type safe way to traverse syntax. Contrary to how AST’s are often implemented, that is using in-memory data types with concrete fields for children, the AST in Mist is only a projection over the CST. This allows high reuse of the underlying data, and minimizes overall memory usage.

Each node in the AST contains a pointer into the CST, and provides type safe accessors for children and even parents. These accessors are generated directly from the grammar (see \cref{appendix-the-mist-grammar}) resulting in a about 3500 lines for highly repetitive code. Additionally, AST nodes also maintain the current span in the source file, allowing referencing back to to the original source code for providing diagnostics and semantic highlighting.

```rust
enum Item {
  Fn(Fn),
  Struct(Struct),
  TypeInvariant(TypeInvariant),
}
```

```rust
struct Fn { syntax: SyntaxNode }
impl Fn {
  fn fn_token(&self) -> Option<SyntaxToken> { .. }
  fn name(&self) -> Option<Name> { .. }
  fn param_list(&self) -> Option<ParamList> { .. }
  fn thin_arrow_token(&self) -> Option<SyntaxToken> { .. }
  fn ret(&self) -> Option<Type> { .. }
  fn conditions(&self) -> AstChildren<Condition> { .. }
  fn decreases(&self) -> Option<Decreases> { .. }
  fn body(&self) -> Option<BlockExpr> { .. }
  fn semicolon_token(&self) -> Option<SyntaxToken> { .. }
}
```


### High-level IR (HIR) {#high-level-ir-hir}


### Mid-level IR (MIR) {#mid-level-ir-mir}
The MIR representation is a control-flow graph (CFG) with branchless basic blocks and branching terminators. Every construct in the HIR is lowered into MIR.



```{=tex}
\begin{definition}\label{definition-place} \@ifnextchar\par{\@gobble}{}
```

Let $\places$ be the (possibly infinite) set of all places inductively defined such that
$$
\begin{aligned}
\slots &\subseteq \places, \\
\{ \rho.f_i \mid \rho \in \places \land \texttt{t} = \texttt{typeOf}(\rho) \land f_i \in \fields(\texttt{t}) \} &\subseteq \places. \\
\end{aligned}
$$
Then any place $\rho$ starts with a slot $s$ followed by a sequence of well-typed fields $f_1,\dots,f_n$, denoted by $\rho = s.f_1.\cdots .f_n$.
```{=tex}
\end{definition}
```




```{=tex}
\begin{definition}\label{definition-place-fields} \@ifnextchar\par{\@gobble}{}
```

Let $\rho$ be a place of with fields $f_1,\dots,f_n$, then we say that $\fields(\rho)$ is the set of places of the fields of $\rho$. Formally:
$$
\fields(\rho) = \{ \rho.f_1, \dots, \rho.f_n \}
$$
For places with types without fields, such as `int` and `bool`, we define $\fields(\rho) = \emptyset$.
```{=tex}
\end{definition}
```




```{=tex}
\begin{definition}\label{definition-place-prefix} \@ifnextchar\par{\@gobble}{}
```

Let $\rho$ be a place s.t. $\rho = s.f_1.\cdots.f_n$, then we define the prefix set of $\rho$ as
$$
\prefix(\rho) = \{ s,\;\; s.f_1,\;\;\cdots,\;\; s.f_1.\cdots.f_n \}.
$$

```{=tex}
\end{definition}
```





## Incremental recompilation model {#incremental-recompilation-model}
The compiler is built in a way to allow incremental recompilation.

Salsa is an incremental computation engine. Read more [here](https://salsa-rs.netlify.app/).



## Language server integration {#language-server-integration}
To provide a good editor experience,


# Automatic folding of isorecursive structures {#automatic-folding-of-isorecursive-structures}
In Mist, types such as `struct`s and `enum`s are named, allowing them to be referenced inside other types, in function arguments, and local variables (see \cref{types-in-mist}). In addition to being a collection of fields, they can also carry logical properties with `invariant`s. From a programmer's perspective, these fields and properties can be accessed at any point in the program, and the invariants can for the most part be assumed to hold, without the need for any additional annotation.^[Note: Due to this, we say that the Mist source language has _transparent_ types, but it's not important.]

This property, however, introduces an implicit guarantee about where and when the invariants of a type holds. This is tricky when we, for example, modify the internals of a struct in a sequence of operations, and part way through the mutation the invariants only partially hold.



::: {.figure #figure-breaking-invariant}

> [!subfigure|width=0.9,align=b]
> ```{.mist .numberLines .ignoreErrors}
> struct S { x: X, y: Y }     struct X { u: int, v: int }
> struct Y { a: A, b: int }   struct A { f: int, g: int, h: int }
>
> invariant X { self.u == -self.v }
>
> fn inc(s: &mut S) -> int {
>     assert s.x.v + s.x.u == 0;
>     
>     s.x.u += 1;
>     s.x.v -= 1;
>
>     s.y.a.g
> }
> ```

> [!caption]
> This program introduces three pieces of information: Definitions for structs `S`, `X`, `Y`, and `A` \lineref{1-2}, an invariant on `X` \lineref{4}, and a function `inc` \lineref{6-13}.


:::


> [!example]
> Consider the program in \cref{figure-breaking-invariant}. The `struct X` has an `invariant` \lineref{4} establishing a relationship between `.u` and `.v`.
>
> With this, the programmer can assume that when given a reference to type `S` \lineref{6} then it's field `.x` of type `X` satisfies the invariant of `X`. Thus the assertion \lineref{7} must hold.
>
> The invariant, however, is temporarily broken when `s.x.u` is incremented \lineref{9} only to be restored again when `s.x.v` is decremented \lineref{10}.

In Mist, we want _temporary invalidation_ of invariants to be allowed, and we want the compiler to figure out^[Authors note: I don't use "infer" here on purpose, since we will use it in a more formal sense later] when invariants must hold and when it's okay for them not to.

The goal is thus for the compiler to infer what we call _folding points_, which are locations in a function were we either can assume the properties of an invariant (an _unfold_), or where we must assert that an invariant holds (a _fold_).

Each folding is associated with a place (as a reminder, a place is a slot and sequence of fields, see \cref{definition-place}). We say that when we unfold a place, that place becomes unfolded and so is its type, and conversely the same is true for fold. Additionally, only folded places can be unfolded and only unfolded places can be folded.

Conceptually, this means that all places and types have two states, folded and unfolded, and such types are called _isorecursive types_.

> [!remark]
> For the most part, languages has _equirecursive types_ as opposed to _isorecursive types_. With equirecursive types there's no difference between having a named type, and having access to its fields (they are equivalent), but with isorecursive types these two states are distinct. Going between the two states we refer to as _fold_ and _unfold_, but common terminology is also _roll_ and _unroll_.

In addition to controlling invariants, folding also limits field access such that a place must be unfolded for its fields to be accessible, and this exactly what the compiler uses to determine folding places.

> [!example]
> Consider again the program in \cref{figure-breaking-invariant}. In the first assertion (repeated below) the nested field `.x.v` needs to be accessible on `s`.
> ```{.mist .ignoreErrors}
> assert s.x.v + s.x.u == 0;
> ```
> This means that not only does `s` need to be unfolded, but also `s.x`. Incidentally, `.x.u` is also accessed, which requires the same unfolded places.
> 
> Moreover, the argument `s: &mut S` requires that upon returning, `s` and anything it might contain, will be folded. This makes sure that invariants of `s`, and any nested invariants, hold when the function exits.

To reason about these foldings more formally, we need a data structure to lay a foundation precisely describing folding requirements.



## Folding tree structure {#folding-tree-structure}
A folding tree is a data structure, denoted by $\T$, that maintains the folding state of places. It works by representing data types as a tree, where nodes are fields of potentially nested `struct`s. The leaves of the tree are all the places that are _folded_, while the internal nodes are _unfolded_ places, both uniquely described by paths from the root.



::: {.figure #figure-a-visualization-of-the-maximally-unfolded-tree-given-by-s}

> [!subfigure|width=0.30]
> ```{.folding-tree root="$\\T_s$"}
> { x { u X v X } y { a { f X g X h X } b X } }
> ```

> [!caption]
> A visualization of a maximally unfolded folding tree of `struct S` defined in \cref{figure-breaking-invariant}.


:::




```{=tex}
\begin{definition}\label{definition-folding-tree} \@ifnextchar\par{\@gobble}{}
```

Let $\T$ be a folding tree represented as a set of places, where every prefix of a place in $\T$ is also in $\T$. Nodes in the tree can either be _folded_ or _unfolded_, where folded nodes are leaves and unfolded nodes are internal nodes. All places have a set of associated fields, and unfolded nodes will have all of their fields as children.

Formally, we say that the any folding tree is a _prefix-closed set of places_, that is
$$
\forall \rho \in \T : \prefix(\rho) \subseteq \T.
$$

Additionally, let $\Ts$ represent the set of all trees.

```{=tex}
\end{definition}
```




```{=tex}
\begin{example}\label{example-folding-tree} \@ifnextchar\par{\@gobble}{}
```
To see how such a folding tree might look, consider the `struct S` from the running example \cref{figure-breaking-invariant}. The tree in \cref{figure-a-visualization-of-the-maximally-unfolded-tree-given-by-s} displays the maximally unfolded version of `S`.

The set representing this tree will be:
$$\T_s = \{ s,\; s.x,\; s.x.u,\; s.x.v,\; s.y,\; s.y.a,\; s.y.a.f,\; s.y.a.g,\; s.y.a.h,\; s.y.b \}.$$
```{=tex}
\end{example}
```


When working with folding trees, it is useful to be able to refer to the set of folded places. For this we introduce a function $\leaves$ which computes this.



```{=tex}
\begin{definition}\label{definition-leaves} \@ifnextchar\par{\@gobble}{}
```
Let $\T$ be a folding tree, then $\leaves(\T)$ is the set of all leaves (and thus folded places) in $\T$ computed by
$$
\leaves(\T) = \{ \rho \mid \rho \in \T \land \fields(\rho) \not\subseteq \T \}.
$$

```{=tex}
\end{definition}
```


> [!example]
> Consider again `struct S` from \cref{figure-breaking-invariant} and $\T_s$ from \cref{example-folding-tree}, then we can compute the folded places in $\T_s$ as:
> $$\leaves(\T_s) = \{ s.x.u,\; s.x.v\;, s.y.a.f,\; s.y.a.g,\; s.y.a.h,\; s.y.b \}.$$

## Operations on folding trees

Fundamentally a folding tree has two operations: $\unfold$ and $\fold$, which both take a place and a folding tree, and either unfolds or folds that place in the tree. The requirements for folding, is that the given place is unfolded and that all of it's fields are folded. Conversely, the requirements for unfolding is that the given place is folded.





```{=tex}
\begin{definition}\label{definition-folding} \@ifnextchar\par{\@gobble}{}
```

Let $\mathcal{F} : \places \to \Ts \to \Ts$ be a _folding_ or _unfolding_ of a place in a folding tree. Let $\rho$ be a place and $\T$ a folding tree, then we define $\fold$ and $\unfold$
$$
\begin{aligned}
\fold(\rho, \T) &= \T \setminus \fields(\rho) \\[2pt]
\text{where }& \fields(\rho) \subseteq \leaves(\T) \\[1em]
\unfold(\rho, \T) &= \T \cup \fields(\rho) \\[2pt]
\text{where }& \rho \in \leaves(\T) \\
\end{aligned}
$$
Additionally, we allow $\fold$ and $\unfold$ to be _partially applied_, which is to say that $\fold\;\rho = \lambda \T. \fold(\rho, \T)$ is a function of type $\T \to \T$ which folds $\rho$ in the provided tree, and likewise for $\unfold$.
```{=tex}
\end{definition}
```




::: {.figure #figure-folding-tree-folding-sequence}

> [!subfigure|width=0.13]
> ```{.folding-tree root="$\\T_1$"}
> { x X y X }
> ```

> [!subfigure|width=0.1,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize \unfold\;$.y$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.13]
> ```{.folding-tree root="$\\T_2$"}
> { x X y { a X b X } }
> ```

> [!subfigure|width=0.1,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize \unfold\;$.y.a$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.13]
> ```{.folding-tree root="$\\T_3$"}
> { x X y { a { f X g X h X } b X } }
> ```

> [!subfigure|width=0.1,align=t]
> $$
> \overset{\text{\footnotesize \fold\;$.y.a$}}{\longrightarrow}
> $$

> [!subfigure|width=0.13]
> ```{.folding-tree root="$\\T_4$"}
> { x X y { a X b X } }
> ```

> [!subfigure|width=0.1,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize \fold\;$.y$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.13]
> ```{.folding-tree root="$\\T_5$"}
> { x X y X }
> ```

> [!caption]
> A sequence of unfoldings and foldings. Each successive tree operates on the previous one, where the operation performed is indicated by the arrow between them.


:::


> [!example]
> If we again consider `struct S` from \cref{figure-breaking-invariant}, then \cref{figure-folding-tree-folding-sequence} shows the $\unfold$ and $\fold$ operations transform the tree.
>
> We see that unfolding $.y$ makes all of the fields of $.y$ available and folded, while the last fold of $.y$  removes the fields to have $.y$ folded. A similar thing happens for the unfolding and folding of $.y.a$.

As $\fold$ and $\unfold$ are the building blocks for further analysis, it is helpful to formalise some of the properties they have, the first of which might seem trivial, but is perhaps the most important one.



```{=tex}
\begin{lemma}\label{lemma-leaves-from-folding} \@ifnextchar\par{\@gobble}{}
```

Let $\rho$ be a place and $\T$ be a folding tree.

If $\fields(\rho) \subseteq \leaves(\T)$, then the leaves in the tree given by folding $\rho$ will be
	$$\leaves(\fold(\rho, \T)) = (\leaves(\T) \setminus \fields(\rho)) \cup \{\rho\}.$$
Similarly, if $\rho \in \leaves(\T)$, then unfolding $\rho$ will give
	$$\leaves(\unfold(\rho, \T)) = (\leaves(\T) \setminus \{\rho\}) \cup \fields(\rho).$$
```{=tex}
\end{lemma}
```


A detailed proof for this lemma is found in \cref{appendix-proof-of-leaves-from-folding}.

The next property, is that the two functions, $\fold$ and $\unfold$, allow us to undo the actions of the other.

> [!lemma]
> The operations $\fold$ and $\unfold$ are inverse functions.^[Todo: Do we need to say that $\rho$ has to be the same, since they are binary, or is that implied?]

> [!proof]
> Let $\rho$ be a place and $\T$ be a folding tree such that $\rho : (f_1, \dots, f_n) \in \leaves(\T)$. By definition of $\unfold$, $\leaves(\unfold(\rho, \T))$ will contain $\{f_1, \dots, f_n\}$, thus have the conditions met for folding $\rho$. Sequentially this means,
> $$
> \begin{aligned}
> \fold(\rho, \unfold(\rho, \T))
> 	&= \fold(\rho, \T \cup \{ \rho.f_1, \dots, \rho.f_2 \}) \\
> 	&= (\T \cup \{ \rho.f_1, \dots, \rho.f_2 \}) \setminus \{ \rho.f_1, \dots, \rho.f_2 \} \\
> 	&= \T \\
> \end{aligned}
> $$

Another neat property, is that $\fold$ and $\unfold$ commutes under the inverse, allowing us to undo chains of foldings by reversing the chain and inverting^[Todo: Which is more correct here, "inverting" or "inversing"?] every folding.



```{=tex}
\begin{lemma}\label{lemma-fold-and-unfold-are-anticommutative} \@ifnextchar\par{\@gobble}{}
```

Let $\T_1,\T_2$ be a folding trees and let $\mathcal{F}_1, \mathcal{F}_2$ be foldings such that $\mathcal{F}_1(\mathcal{F}_2(\T_1)) = \T_2$ then we say that the composition of foldings is anticommutative [see @bourbakiElementsMathematicsChapters2009 pp. 482], that is
$$
\mathcal{F}_1 \circ \mathcal{F}_2 = (\mathcal{F}_2^{-1} \circ \mathcal{F}_1^{-1})^{-1}.
$$

```{=tex}
\end{lemma}
```


> [!proof]
> Assuming function composition "$\circ$" is associative, then
> $$
> \begin{aligned}
> (\mathcal{F}_1 \circ \mathcal{F}_2) \circ (\mathcal{F}_2^{-1} \circ \mathcal{F}_1^{-1})
> &= \mathcal{F}_1 \circ (\mathcal{F}_2 \circ \mathcal{F}_2^{-1}) \circ \mathcal{F}_1^{-1} \\
> &= \mathcal{F}_1 \circ \mathcal{F}_1^{-1} = \text{\normalfont{id}}.
> \end{aligned}
> $$

A common operation on folding trees is transforming an existing tree into a new one with a desired place folded. To do so, a sequence of foldings and unfoldings must be performed to arrive at the desired tree. We call this operation _requires_ and use the notation $\T \requires \rho$ to say that we want the tree $\T$ but with the minimal number of foldings and unfoldings performed to have $\rho$ be folded.



```{=tex}
\begin{definition}\label{definition-requires} \@ifnextchar\par{\@gobble}{}
```
Let $\requires : \Ts \to \places \to \Ts$ be the recursive function that takes a folding tree $\T$ and a place $\rho$ and produces a new tree, where $\rho$ is folded, defined like so
$$
\T \requires \rho = \begin{cases}
	\T & \text{if } \rho \in \leaves(\T) \land \rho \in \T, \\
	\fold(\rho, \T \requires \rho.f_1 \requires \dots \requires \rho.f_n) & \text{if } \rho \notin \leaves(\T) \land \rho \in \T \land \fields(\rho) \subseteq \T, \\
	\unfold(\rho', \T \requires \rho') & \text{if } \rho \notin \leaves(\T) \land \rho \notin \T \land \rho = \rho'.f_i, \\
\end{cases}
$$
```{=tex}
\end{definition}
```




::: {.figure #figure-folding-tree-requires-sequence}

> [!subfigure|width=0.1]
> ```{.folding-tree root="$\\T_1$"}
> { x X y X }
> ```

> [!subfigure|width=0.07,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize $\requires .x.u$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.125]
> ```{.folding-tree root="$\\T_2$"}
> { x { u X v X } y X }
> ```

> [!subfigure|width=0.07,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize $\requires .y.a.g$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.19]
> ```{.folding-tree root="$\\T_3$"}
> { x { u X v X } y { a { f X g X h X } b X } }
> ```

> [!subfigure|width=0.06,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize $\requires .x$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.13]
> ```{.folding-tree root="$\\T_4$"}
> { x X y { a { f X g X h X } b X } }
> ```

> [!subfigure|width=0.09,align=t]
> $$
> \begin{gathered}
> \text{\footnotesize $\requires .y$} \\[-5pt]
> \longrightarrow
> \end{gathered}
> $$

> [!subfigure|width=0.1]
> ```{.folding-tree root="$\\T_5$"}
> { x X y X }
> ```

> [!caption]
> A sequence of folding trees showing how $\requires$ can perform the necessary foldings for making fields accessible. Each successive tree operates on the previous one, where the operation performed is indicated by arrow between them.


:::


> [!example]
> To get an idea of the operation, see \cref{figure-folding-tree-requires-sequence}, which again works on `struct S` from \cref{figure-breaking-invariant}.
>
> The first transition shows how a single unfolding of $.x$ is performed, to make $.x.u$ available, while the second transition shows the two unfoldings necessary to make $.y.a.g$ available.
>
> The last two transitions show the folding up to make $.x$ and $.y$ folded.

For $\requires$ to be useful, we need to show some properties that it satisfies, but first, we need to establish way to talk about the remaining leaves after requiring a place.



```{=tex}
\begin{definition}\label{definition-cut} \@ifnextchar\par{\@gobble}{}
```
Let $\rho$ be a place and $\T$ a folding tree, then we define $\cut : \places \to \Ts \to \text{\normalfont PowerSet}(\places)$ as the function that computes the set of remaining leaves in $\T$ after removing all that are either a prefix or a suffix of $\rho$. Formally defined as
$$
\cut(\rho,\T) = \{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho) \not\subseteq \prefix(\rho') \land \prefix(\rho') \not\subseteq \prefix(\rho) \}
$$
```{=tex}
\end{definition}
```


> [!example]
> Consider again \cref{figure-folding-tree-requires-sequence} and lets look at cutting places in the trees:
> $$
> \begin{aligned}
> \cut(s.x,     \T_2) &= \{s.y\}, \\
> \cut(s.y.a,   \T_3) &= \{s.x.u,\;s.x.v,\;s.y.b\}, \\
> \cut(s.y.a.f, \T_3) &= \{s.x.u,\;s.x.v,\;s.y.a.g,\;s.y.a.h\;,\;s.y.b\}, \\
> \cut(s.y.a.f, \T_5) &= \{s.x\}. \\
> \end{aligned}
> $$

Intuitively, the $\cut$ can be thought of as removing all leaves leading up to $\rho$, and all leaves which are children of $\rho$. This also means that if we cut a field, it wont remove as more than cutting its parent will. In the extreme case, cutting the root of the tree always _removes all leaves_, while cutting anywhere else, does not necessarily do so.



```{=tex}
\begin{lemma}\label{lemma-folding-tree-weaken-cut} \@ifnextchar\par{\@gobble}{}
```

Let $\rho$ be a place and let $\rho.f_i \in \fields(\rho)$. Then for any folding tree $\T$, the leaves in remaining after cutting $\rho$ will be no more than when cutting $\rho.f_i$.
$$
\begin{aligned}
\cut(\rho, \T) \subseteq \cut(\rho.f_i, \T)
\end{aligned}
$$

```{=tex}
\end{lemma}
```


> [!proof]
> Let $\rho'$ be an element of $\leaves(T)$, then assume $\prefix(\rho) \not\subseteq \prefix(\rho')$, and, $\prefix(\rho') \not\subseteq \prefix(\rho)$, and recall that $\prefix(\rho) \subset \prefix(\rho.f_i)$ since $\prefix(\rho.f_i) = \prefix(\rho) \cup \{\rho.f_i\}$.
> 
> Since $\prefix(\rho')$ is not contained in $\prefix(\rho)$, and that it is a subset of $\prefix(\rho.f_i)$, we can say that $\prefix(\rho') \not\subseteq \prefix(\rho.f_i)$.
> 
> By a similar argument, we know that $\prefix(\rho.f_i)$ is larger than $\prefix(\rho)$, and since $\prefix(\rho) \not\subseteq \prefix(\rho')$, then $\prefix(\rho.f_i) \not\subseteq \prefix(\rho')$.
> 
> Since $\rho'$ was taken from $\leaves(\T)$ and the properties assumed were the conditions necessary to be in $\cut(\rho, \T)$, and we showed the conditions for being in $\cut(\rho.f_i, \T)$, we can conclude that the subset inclusion holds.

One point of interest regarding $\cut$, is that the set of places it produces _does not contain the provided $\rho$_, and thus does not form the leaves of a valid folding tree. It, however, does remove everything that would be in violation of having $\rho$ be a leaf, and thus is quite useful in specifying the perhaps the most important the property of $\requires$.



```{=tex}
\begin{lemma}\label{lemma-requires-properties} \@ifnextchar\par{\@gobble}{}
```

The tree produced by $\T \requires \rho$ will have $\rho$ folded, and every leaf of $\T$ will still be a leaf if it is not an ancestor nor descendent of $\rho$, that is
$$
\cut(\rho, \T) \cup \{ \rho \} \subseteq \leaves(\T \requires \rho).
$$
```{=tex}
\end{lemma}
```


The proof for this is deferred to \cref{appendix-proof-of-leaves-from-folding}.

With \cref{lemma-requires-properties} we can ensure that requiring some place in one subtree does not alter the folded places of any unrelated subtree.

> [!example]
> Consider `struct S` from \cref{figure-breaking-invariant} once again. If we require some (potentially nested) field of `s.y`, then it will never fold nor unfold the fields of `s.x`.
> 
> This ensures that folding stays isolated to the parts of a structure for which it is relevant.

## Ordering of folding trees

The goal of folding trees is to formalise the state of foldings at specific program points, and thus it is crucial that we not only have ways to mutate them, but also have a ways of relating one folding tree to another.

The first step for doing so, is defining a _partial order_ for folding trees.



```{=tex}
\begin{definition}\label{definition-folding-tree-order} \@ifnextchar\par{\@gobble}{}
```

Let $\T_1$ and $\T_2$ be folding trees, then we say that $\T_1$ is _smaller than_ $\T_2$ iff every leaf of $\T_1$ is contained in $\T_2$, that is
$$
\T_1 \smaller \T_2 = \leaves(\T_1) \subseteq \T_2,
$$
or equivalently, since $\leaves(\T_1) \subseteq \T_1$ and $\T_1$ is prefix closed
$$
\T_1 \smaller \T_2 = \T_1 \subseteq \T_2.
$$

```{=tex}
\end{definition}
```




```{=tex}
\begin{lemma}\label{lemma-folding-tree-partial-order} \@ifnextchar\par{\@gobble}{}
```

The operator $\smaller$ defines a partial order for $\Ts$.
```{=tex}
\end{lemma}
```


> [!proof]
> To show this, we refer the fact that $\smaller$ is defined in terms of $\subseteq$ which forms a partial order.

With an ordering established, we define how to combine two trees, either by creating a new tree which fits within both, or one which contains both trees. This is especially useful for talking about the most unfolded tree, which does not unfold more than either of two other trees.



```{=tex}
\begin{definition}\label{definition-folding-tree-join-and-meet} \@ifnextchar\par{\@gobble}{}
```

The _join_ of two folding trees $\T_1$ and $\T_2$ is defined by
$$
\T_1 \join \T_2 = \T_1 \cup \T_2.
$$
Similarly, the _meet_ of two folding trees $\T_1$ and $\T_2$ is defined by
$$
\T_1 \meet \T_2 = \T_1 \cap \T_2.
$$

```{=tex}
\end{definition}
```




```{=tex}
\begin{lemma}\label{lemma-folding-tree-join-and-meet-are-least-upper-bound-and-greatest-upper-bound} \@ifnextchar\par{\@gobble}{}
```

For folding trees, $\join$ and $\meet$ compute the _least upper bound_ and _greatest lower bound_ respectively, with respect to $\smaller$.
```{=tex}
\end{lemma}
```


> [!proof]
> The operators are defined in terms of $\cup$ and $\cap$, which compute the least upper bound and greatest lower bound w.r.t. $\subseteq$, and thus so will $\join$ and $\meet$.



::: {.figure #figure-folding-meet-join}

> [!subfigure]
> ```{.folding-tree root="$\\T_1 \\meet \\T_2$"}
> { x X y { a X b X } }
> ```

> [!subfigure]
> ```{.folding-tree root="$\\T_1$"}
> { x { u X v X } y { a X b X } }
> ```

> [!subfigure]
> ```{.folding-tree root="$\\T_2$"}
> { x X y { a { f X g X h X } b X } }
> ```

> [!subfigure]
> ```{.folding-tree root="$\\T_1 \\join \\T_2$"}
> { x { u X v X } y { a { f X g X h X } b X } }
> ```

> [!caption]
> A visualization of the lattice operations described in \cref{definition-folding-tree-join-and-meet}, showing the result of $\T_1 \meet \T_2$ and  $\T_1 \join \T_2$.


:::


> [!example]

The operators allows us to construct new trees, but when doing so, it is crucial that the resulting trees are still folding trees.



```{=tex}
\begin{lemma}\label{lemma-folding-tree-join-and-meet-are-closed} \@ifnextchar\par{\@gobble}{}
```

$\Ts$ is closed under both $\join$ and $\meet$, which is to say that the set produced by either, satisfies the properties of being a folding tree.
```{=tex}
\end{lemma}
```


> [!proof]
> The condition for being a folding tree, is that the set must be closed under prefix in accordance to \cref{definition-folding-tree}. To show this, let $\T_1$ and $\T_2$ be arbitrary folding trees, then we can assume that
> $$
> \forall \rho \in \T_1 : \prefix(\rho) \subseteq \T_1, \;\text{ and, }\; \forall \rho \in \T_2 : \prefix(\rho) \subseteq \T_2.
> $$
> What we need to show is that
>  $$
> \forall \rho \in \T_1 \join \T_2 : \prefix(\rho) \subseteq \T_1 \join \T_2, \;\text{ and, }\; \forall \rho \in \T_1 \meet \T_2 : \prefix(\rho) \subseteq \T_1 \meet \T_2.
> $$
> For the first let $\rho_1$ be an element of $\T_1 \join \T_2$, then we know that $\rho_1$ is an element of $\T_1$ or $\T_2$. With out loss of generality assume $\rho_1 \in \T_1$, and we have $\prefix(\rho_1) \subseteq \T_1$ by the initial assumption. Then with the fact that $\T_1 \smaller \T_1 \join \T_2$, we can say that $\prefix(\rho_1) \subseteq \T_1 \join \T_2$ by transitivity.
> 
> Next, let $\rho_2$ be an element of $\T_1 \meet \T_2$, which means that $\rho_2$ must be an element of both $\T_1$ and $\T_2$, thus giving us $\prefix(\rho_2) \subseteq \T_1$ and $\prefix(\rho_2) \subseteq \T_2$. Combining these two, we get that $\prefix(\rho_2) \subseteq \T_1 \cap \T_2$, which by \cref{definition-folding-tree-join-and-meet} shows $\prefix(\rho_2) \subseteq \T_1 \meet \T_2$.



```{=tex}
\begin{definition}\label{definition-folding-tree-top-and-bottom} \@ifnextchar\par{\@gobble}{}
```

Let $\bot \in \Ts$ be the _least element_ with respect to $\smaller$, which is the tree with the root folded, or the set $\emptyset$.

Additionally, let $\top \in \Ts$ be the _greatest element_ with respect to $\smaller$, which is the maximally unfolded tree, which is given by $\places$.

```{=tex}
\end{definition}
```


> [!remark]
> Remember from \cref{definition-place} that $\places$ is potentially infinite, which would make $\top$ an infinite set, meaning that it is impractical to represent fully in practice. Luckily, it satisfies two algebraic properties which makes it simple to use in most cases:
> $$
> \begin{array}{ccc}
> \;\;\;\;&\top \join \T = \top, \text{ and, } \top \meet \T = \T & \forall \T \in \Ts.
> \end{array}
> $$

Having defined the ordering, upper and lower bounds, $\bot$ and $\top$, we have the necessary requirements to put it all together.



```{=tex}
\begin{lemma}\label{lemma-folding-tree-lattice} \@ifnextchar\par{\@gobble}{}
```

The set of folding trees $\Ts$ equipped with $\smaller$ forms a lattice.



```{=tex}
\end{lemma}
```


> [!proof]
> The requirements are showed in \cref{lemma-folding-tree-partial-order}, \cref{lemma-folding-tree-join-and-meet-are-least-upper-bound-and-greatest-upper-bound}, and, \cref{lemma-folding-tree-join-and-meet-are-closed}.



This leads us to the final bit of notation for foldings trees, which is computing the minimal foldings and unfoldings required to transform one tree into another. We let $\tinto$ be the function that computes the composition of foldings and unfoldings satisfying the following equation:
$$
\begin{aligned}
\tinto[\T_1, \T_2] = \;& \mathcal{F}_n \circ \dots \circ \mathcal{F}_1 \\
\text{such that } [&\mathcal{F}_n \circ \dots \circ \mathcal{F}_1](\T_1) = \T_2
\end{aligned}
$$



```{=tex}
\begin{definition}\label{definition-folding-tree-transition} \@ifnextchar\par{\@gobble}{}
```

Let $\T_1, \T_2$ be folding trees, then $\tinto(\T_1, \T_2)$ computes the foldings and unfoldings necessary to transform $\T_1$ into $\T_2$, that is
$$
\begin{aligned}
\tinto[\T_1, \T_2] = \;& \mathcal{F}_n \circ \dots \circ \mathcal{F}_1 \\
\text{such that } [&\mathcal{F}_n \circ \dots \circ \mathcal{F}_1](\T_1) = \T_2.
\end{aligned}
$$

```{=tex}
\end{definition}
```


> [!example]
> Using the trees from \cref{figure-folding-meet-join}, we can compute the foldings by way of $\tinto$:
> $$
> \begin{gathered}
> \begin{aligned}
>     \tinto[\T_1 \meet \T_2, \T_1] &= \unfold\;.x &
>     \tinto[\T_1 \meet \T_2, \T_2] &= \unfold\;.y.a \\
>     \tinto[\T_1 \join \T_2, \T_1] &= \fold\;.x &
>     \tinto[\T_1 \join \T_2, \T_2] &= \fold\;.y.a \\
> \end{aligned} \\
> \begin{aligned}
>     \tinto[\T_1 \join \T_2, \T_1 \meet \T_2] &= \fold\;.x \circ \fold\;.y.a \\
>     \tinto[\T_1, \T_2] &= \fold\;.x \circ \unfold\;.y.a \\
> \end{aligned}
> \end{gathered}
> $$

> [!lemma]
> The function $\tinto$ is _anticommutative_:
> $$\tinto[\T_1, \T_2] = \tinto[\T_2, \T_1]^{-1}$$

> [!proof]
> Let $\T_1$ and $\T_2$ be arbitrary folding trees, and recall that $\mathcal{F}_1 \circ \mathcal{F}_2 = (\mathcal{F}_2^{-1} \circ \mathcal{F}_1^{-1})^{-1}$ from \cref{lemma-fold-and-unfold-are-anticommutative}, then:
> $$
> \begin{aligned}
>   \T_1 &= \tinto[\T_1, \T_2]^{-1}(\tinto[\T_1, \T_2](\T_1)) \\
>        &= \tinto[\T_1, \T_2]^{-1}(\T_2) \\
>        &= [\mathcal{F}_n \circ \dots \circ \mathcal{F}_1]^{-1}(\T_2) \\
>        &= [\mathcal{F}_1^{-1} \circ \dots \circ \mathcal{F}_n^{-1}](\T_2) \\
>        &= \tinto[\T_2, \T_1](\T_2) \\
> \end{aligned}
> $$
> From this we get that $\T_1 = \tinto[\T_2, \T_1](\tinto[\T_1, \T_2](\T_1))$, showing that $\tinto[\T_2, \T_1]$ is the inverse of $\tinto[\T_1, \T_2]$.

With folding trees defined, we can continue to look at how they relate to programs written in Mist.

## Folding analysis {#folding-analysis}

### Folding-level IR (FIR) {#folding-level-ir-fir}
In \cref{compilation-stages} we introduced the multi-stage structure of the Mist compiler, and specifically the MIR representation which is a CFG. In terms of computing foldings and unfoldings, this is the stage we are concerned with, but instead of considering the full set of instructions and terminators, we consider a smaller variant that focuses much more concretely on _how and what_ places are used and omit the details for performing actual computation or verification. The grammar for this smaller language is shown in \cref{figure-fir-grammar}.



::: {.figure #figure-fir-grammar}

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


:::




### Semantics {#semantics}


### Abstract semantics {#abstract-semantics}


::: {.figure #figure-cfg-with-annotated-analysis}

> [!subfigure|width=0.35]
> ```tikz
> \usetikzlibrary{positioning}
>
> \begin{document}
> \begin{tikzpicture}[
>     auto,
>     box/.style = {rounded corners,color=white,fill=teal,
>         align=center,inner sep=4mm},
>     pp/.style = {align=center},
>     tr/.style = {align=center, font=\footnotesize, color=teal},
> ]
>     \begin{scope}[node distance=20mm and 5mm]
>         \node[box] (b1)                  {$\alpha_1$};
>         \node[box] (b2) [below=of b1]    {$\alpha_2$};
>         \node[box] (b3) [below=of b2]    {$\alpha_3$};
>         \node[box] (b4) [right=of b3]    {$\alpha_4$};
>     \end{scope}
>     \begin{scope}[node distance=-.5mm]
>         \node[pp] (pp1) [above=of b1]    {$\phi_1$};
>         \node[pp] (pp1') [below=of b1,xshift=3mm]   {$\phi_1'$};
>         \node[pp] (pp2) [above=of b2,xshift=3mm]    {$\phi_2$};
>         \node[pp] (pp2') [below=of b2,xshift=3mm]   {$\phi_2'$};
>         \node[pp] (pp3) [above=of b3,xshift=3mm]    {$\phi_3$};
>         \node[pp] (pp3') [below=of b3,xshift=3mm]   {$\phi_3'$};
>         \node[pp] (pp4) [above=of b4,xshift=3mm]    {$\phi_4$};
>         \node[pp] (pp4') [below=of b4]   {$\phi_4'$};
>     \end{scope}
>
>     \begin{scope}[rounded corners,color=teal,-latex]
>         \draw (b1) -- node[tr] {$\delta(\phi_1', \phi_2)$} (b2);
>         \draw (b2) -- node[tr,yshift=-1.5mm] {$\delta(\phi_2', \phi_3)$} (b3);
>         \draw (b2.south) |- ([yshift=13mm]b4.north) -- node[tr,yshift=2mm] {$\delta(\phi_2', \phi_4)$} (b4.north);
>         \draw (b3.south) -- ++(0,-3mm) -| ([xshift=-3mm]b2.west) node[tr,left] {$\delta(\phi_3', \phi_2)$} |- ([yshift=4mm]b2.north) -- (b2.north);
>     \end{scope}
> \end{tikzpicture}
> \end{document}
> ```

> [!subfigure|width=0.55]
> $$
> \begin{aligned}
>     \A(\phi_1) &\smaller \bsem{\alpha_1}(\A(\phi_2)) \\
>     \A(\phi_2) &\smaller \bsem{\alpha_2}(\A(\phi_3)) \\
>     \A(\phi_2) &\smaller \bsem{\alpha_2}(\A(\phi_4)) \\
>     \A(\phi_3) &\smaller \bsem{\alpha_3}(\A(\phi_2)) \\
> \end{aligned}
> $$
> $$
> \begin{gathered}
> \bsem{{\color{Teal900} \delta(\phi_i, \phi_j)}}(\A(\phi_j)) = \tinto[\A(\phi_j), \A(\phi_i)](\A(\phi_j)) \\
> \bsem{\alpha_i}(\A(\phi_i')) = \A(\phi_i)
> \end{gathered}
> $$

> [!caption]
> An abstract CFG with annotated program locations and constraints.


:::



# Conclusion {#conclusion}
```{=tex}
\appendix
```


# The Mist Grammar {#appendix-the-mist-grammar}
```{.ungram .noNames}
SourceFile =
  Item*

Item =
  Const
| Fn
| Struct
| TypeInvariant

Name = 'ident' | 'self'

NameRef = 'ident' | 'self'

Const =
  Attrs
  'const' Name ':' Type
  ('=' Expr)?
  ';'

Fn =
  Attrs
 'fn' Name ParamList
 ('->' Type)?
 Condition*
 Decreases?
 (BlockExpr | ';')

Attr =
  'ghost'
| 'pure'

Attrs = Attr*

ParamList =
 '(' Param* ')'

Param = Attrs Name (':' Type)? ','?

Condition =
  Requires
| Ensures

Requires =
  ('requires' | 'req') CommaExpr*
Ensures =
  ('ensures'  | 'ens') CommaExpr*

CommaExpr =
  Expr ','?

Decreases =
  ('decreases' | 'dec') ('_' | Expr)

Struct =
  Attrs
  'struct' Name GenericArgList? '{' StructField* '}'

StructField = Attrs Name ':' Type ','?

TypeInvariant =
  'invariant' NameRef GenericArgList? BlockExpr

BlockExpr =
 '{'
   Stmt*
   Expr?
 '}'

Type =
  NamedType
| Primitive
| Optional
| ListType
| GhostType
| RefType

NamedType =
  Name GenericArgList?

GenericArgList =
  '<' GenericArg* '>'

GenericArg =
  Type ','?

Primitive =
  'int'
| 'bool'

Optional =
  '?' Type

ListType =
  '[' Type ']'

GhostType =
  'ghost' Type

RefType =
  '&' 'mut'? Type

Stmt =
  LetStmt
| Item
| ExprStmt
| AssertStmt
| AssumeStmt

LetStmt =
 'let' Name (':' Type)?
 ('=' Expr)?
 ';'

ExprStmt =
  Expr ';'

AssertStmt =
 'assert' CommaExpr* ';'

AssumeStmt =
 'assume' CommaExpr* ';'

ReturnExpr =
 'return' Expr ';'

Invariant =
 'invariant' CommaExpr*

Expr =
  Literal
| IfExpr
| ReturnExpr
| WhileExpr
| ForExpr
| PrefixExpr
| BinExpr
| BlockExpr
| RangeExpr
| CallExpr
| ListExpr
| IndexExpr
| NotNullExpr
| FieldExpr
| StructExpr
| ParenExpr
| RefExpr
| IdentExpr
| NullExpr
| ResultExpr
| QuantifierExpr

IfExpr =
  'if' Expr BlockExpr
  ('else' IfExprElse)?

IfExprElse =
  IfExpr | BlockExpr

WhileExpr =
  'while' Expr
  Invariant*
  Decreases?
  BlockExpr

ForExpr =
  'for' NameInExpr
  Invariant*
  BlockExpr

PrefixExpr =
  ('-' | '!') Expr

BinExpr =
  Expr
  (
    '||' | '&&'
  | '==' | '!=' | '<=' | '>=' | '<' | '>'
  | '+' | '*' | '-' | '/' | '%' | '<<' | '>>' | '^' | '|' | '&'
  | '=' | '+=' | '/=' | '*=' | '%=' | '>>=' | '<<=' | '-=' | '|=' | '&=' | '^='
  )
  Expr

RangeExpr =
  Expr?
  '..'
  Expr?

CallExpr =
  Expr ArgList

ArgList =
  '(' Arg* ')'

Arg =
  Expr ','?

ListExpr =
  '[' CommaExpr* ']'

IndexExpr =
  base:Expr '[' index:Expr ']'

NotNullExpr =
  Expr '!'

FieldExpr =
  Expr '.' field:NameRef

StructExpr =
  NameRef '{' fields:StructExprField* '}'

StructExprField =
  NameRef ':' Expr ','?

ParenExpr =
  '(' Expr ')'

RefExpr =
  '&' 'mut'? Expr

IdentExpr =
  NameRef

NullExpr = 'null'

ResultExpr = 'result'

QuantifierExpr = Quantifier QuantifierOver Expr

QuantifierOver = ParamList | NameInExpr

NameInExpr = Name 'in' Expr

Quantifier =
  'forall' | 'exists'

Literal =
    'int_number'
  | 'true' | 'false'
```



# Proof of leaves from folding {#appendix-proof-of-leaves-from-folding}
## Proof of \cref{lemma-leaves-from-folding}



```{=tex}
\begin{proof}\label{proof-leaves-from-folding} \@ifnextchar\par{\@gobble}{}
```

We prove the two statements separately.
$$
\begin{aligned}
&\leaves(\fold(\rho, \T)) \\
  = \;&\text{[By \cref{definition-folding}]} \\
	  &\leaves(\T \setminus \fields(\rho)) \\
  = \;&\text{[By \cref{definition-leaves}]} \\
	  &\{ \rho' \mid \rho' \in (\T \setminus \fields(\rho)) \land \fields(\rho') \not\subseteq (\T \setminus \fields(\rho)) \} \\
  = \;&\text{[Distribute "$\in$" over "$\setminus$"]} \\
	  &\{ \rho' \mid \rho' \in \T \land \rho' \notin \fields(\rho) \land \fields(\rho') \not\subseteq (\T \setminus \fields(\rho)) \} \\
  = \;&\text{[Move $\rho' \notin \fields(\rho)$ out to removal of $\fields(\rho)$]} \\
      &\{ \rho' \mid \rho' \in \T \land \fields(\rho') \not\subseteq (\T \setminus \fields(\rho)) \} \setminus \fields(\rho) \\
  = \;&\text{[For $\rho' = \rho$, $\fields(\rho') \not\subseteq (\T \setminus \fields(\rho))$ evaluates to true, so move $\rho$ out]} \\
      &(\{ \rho' \mid \rho' \in \T \land \fields(\rho') \not\subseteq \T \} \cup \{\rho\}) \setminus \fields(\rho) \\
  = \;&\text{[By \cref{definition-folding}]} \\
      &(\leaves(\T) \cup \{\rho\}) \setminus \fields(\rho) \\
\end{aligned}
$$
$$
\begin{aligned}
&\leaves(\unfold(\rho, \T)) \\
= \;&\text{[By \cref{definition-folding}]} \\
	&\leaves(\T \cup \fields(\rho)) \\
= \;&\text{[By \cref{definition-leaves}]} \\
	&\{ \rho' \mid \rho' \in (\T \cup \fields(\rho)) \land \fields(\rho') \not\subseteq (\T \cup \fields(\rho)) \} \\
= \;&\text{[Split set in two, by inclusion in either $\T$ or $\fields(\rho)$]} \\
	&\{ \rho' \mid \rho' \in \T \land \fields(\rho') \not\subseteq (\T \cup \fields(\rho)) \} \\
	&\cup \{ \rho' \mid \rho' \in \fields(\rho) \land \fields(\rho') \not\subseteq (\T \cup \fields(\rho)) \} \\
= \;&\text{[Since $\fields(\rho) \subseteq \leaves(\T)$, and $\rho \neq \rho'$, the last set becomes $\fields(\rho)$]} \\
	&\{ \rho' \mid \rho' \in \T \land \fields(\rho') \not\subseteq (\T \cup \fields(\rho)) \} \\
	&\cup \fields(\rho) \\
= \;&\text{[For $\rho' = \rho$, $\fields(\rho') \not\subseteq (\T \cup \fields(\rho))$ will be false,} \\&\; \text{and will be the only case where union with $\fields(\rho)$ makes a difference]} \\
	&(\{ \rho' \mid \rho' \in \T \land \fields(\rho') \not\subseteq \T) \} \setminus \{\rho\}) \cup \fields(\rho) \\
= \;&\text{[By \cref{definition-leaves}]} \\
	&(\leaves(\T) \setminus \{\rho\}) \cup \fields(\rho) \\
\end{aligned}
$$

```{=tex}
\end{proof}
```


## Proof of \cref{lemma-requires-properties}



```{=tex}
\begin{proof}\label{proof-requires-properties} \@ifnextchar\par{\@gobble}{}
```

We split the proof goal to show these two properties:
$$
\begin{aligned}
\rho &\in \leaves(\T) & (1) \\
\{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho) \not\subseteq \prefix(\rho') &\land \prefix(\rho') \not\subseteq \prefix(\rho) \} \subseteq \leaves(\T) & (2) \\
\end{aligned}
$$
We do the proof by case distinction on $\T \requires \rho$, and induction on $\rho$:

**Case 1**: Assume $\rho \in \leaves(\T)$, and, $\rho \in \T$, then $\leaves(\T \requires \rho) = \leaves(\T)$. Goal $(1)$ immediately follows, so what we need to show is
$$
\{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho) \not\subseteq \prefix(\rho') \land \prefix(\rho') \not\subseteq \prefix(\rho) \} \subseteq \leaves(\T).
$$
Since $\rho'$ is limited to leaves in $\T$, we know that the set will never contain elements outside of $\leaves(\T)$, concluding goal $(2)$.

**Case 2**: Assume $\rho \notin \leaves(\T)$, $\rho \in \T$, and, $\fields(\rho) \subseteq \T$, and let $\T_i = \T \requires \rho.f_1 \requires \dots \requires \rho.f_i$, then we need to show
$$
\{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho) \not\subseteq \prefix(\rho') \land \prefix(\rho') \not\subseteq \prefix(\rho) \} \cup \{ \rho \} \subseteq \leaves(\fold(\rho, \T_n)).
$$
Now compute $\T_1 = \T \requires \rho.f_1$, which by induction gives us
$$
\begin{aligned}
\rho.f_1 &\in \leaves(\T_1) & (3) \\
\{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho.f_1) \not\subseteq \prefix(\rho') &\land \prefix(\rho') \not\subseteq \prefix(\rho.f_1) \} \subseteq \leaves(\T_1) & (4) \\
\end{aligned}
$$
By \cref{lemma-folding-tree-weaken-cut} we can weaken $(4)$ to get
$$
\{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho) \not\subseteq \prefix(\rho') \land \prefix(\rho') \not\subseteq \prefix(\rho) \} \subseteq \leaves(\T_1)
$$
Now we perform induction on the fields, where the previous case was the base case, and thus we assume
$$
\begin{aligned}
\{\rho.f_1, \dots, \rho.f_i \} &\subseteq \leaves(\T_i) & (5) \\
\{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho) \not\subseteq \prefix(\rho') &\land \prefix(\rho') \not\subseteq \prefix(\rho) \} \subseteq \leaves(\T_i) & (6) \\
\end{aligned}
$$
Then we compute $\T_{i+1} = \T_i \requires \rho.f_{i+1}$, which by induction says that
$$
\begin{aligned}
\rho.f_{i+1} &\in \leaves(\T_{i+1}) & (7) \\
\{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho.f_{i+1}) \not\subseteq \prefix(\rho') &\land \prefix(\rho') \not\subseteq \prefix(\rho.f_{i+1}) \} \subseteq \leaves(\T_{i+1}) & (8) \\
\end{aligned}
$$
By $(7,8)$ we can derive that all prior fields of $\rho$ will still be in $\leaves(\T_{i+1})$, since $\prefix(\rho.f_u)$ will never be contained in $\prefix(\rho.f_v)$, and vice versa, for all $u,v$, and thus $\{\rho.f_1,\dots,\rho.f_i,\rho.f_{i+1}\} \subseteq \leaves(\T_{i+1})$. And again we can apply \cref{lemma-folding-tree-weaken-cut} on $(8)$ to weaken the property.

By completing the induction on the fields, we end up with
$$
\begin{aligned}
\fields(\rho) &\subseteq \leaves(\T_n) & (9) \\
\{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho) \not\subseteq \prefix(\rho') &\land \prefix(\rho') \not\subseteq \prefix(\rho) \} \subseteq \leaves(\T_n) & (10) \\
\end{aligned}
$$
Using \cref{lemma-leaves-from-folding}, with $(9)$ giving us the necessary conditions to perform $\fold\;\rho$ on $\T'$, we show $(1)$. What remains to be shown $(2)$, which after applying \cref{lemma-leaves-from-folding} is
$$
\{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho) \not\subseteq \prefix(\rho') \land \prefix(\rho') \not\subseteq \prefix(\rho) \} \subseteq (\leaves(\T_n) \setminus \fields(\rho)) \cup \{\rho\}
$$
From $(10)$ we know that the left-hand side is contained in $\leaves(\T_n)$, and since fields of $\rho$ will not satisfy $\prefix(\rho) \not\subseteq \prefix(\rho')$, no entries from $\fields(\rho)$ will be in the left-hand side, thus showing $(2)$.

**Case 3:** Assume $\rho \notin \leaves(\T)$, $\rho \notin \T$, and, $\rho = \rho^*.f_i$ (that is, $\rho^*$ is the parent of $\rho$), and let $\T' = \T \requires \rho^*$, then we need to show
$$
\{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho) \not\subseteq \prefix(\rho') \land \prefix(\rho') \not\subseteq \prefix(\rho) \} \cup \{ \rho \} \subseteq \leaves(\unfold(\rho^*, \T')).
$$
By induction on $\rho^*$, we know that
$$
\begin{aligned}
\rho^* &\in \leaves(\T') & (11) \\
\{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho^*) \not\subseteq \prefix(\rho') &\land \prefix(\rho') \not\subseteq \prefix(\rho^*) \} \subseteq \leaves(\T') & (12) \\
\end{aligned}
$$
By $(11)$ we have the necessary condition met to perform $\unfold\;\rho^*$, and can by apply \cref{lemma-leaves-from-folding} to alter the goal to
$$
\begin{aligned}
\{ \rho' \mid \rho' \in \leaves(\T) \land \prefix(\rho) \not\subseteq \prefix(\rho') \land \prefix(\rho') \not\subseteq \prefix(\rho) \} \cup \{\rho\} \\ \subseteq (\leaves(\T') \setminus \{\rho^*\})\cup\fields(\rho^*).
\end{aligned}
$$
Since $\rho \in \fields(\rho^*)$ we get $(1)$. Next, we look two cases of $\rho'$: If $\rho' = \rho^*$, then it will be excluded since $\prefix(\rho^*) \subseteq \prefix(\rho)$, satisfying the $\subseteq$ of the right-hand side. Otherwise, $\rho' \neq \rho^*$, then we can use $(12)$ to show that it is included in $\leaves(\T')$. Thus showing $(2)$.


```{=tex}
\end{proof}
```
