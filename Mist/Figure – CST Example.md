---
tags: figure
---

> [!subfigure|width=0.4]
> ```{.mist .ignoreErrors}
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
> A simple Mist program, with its CST displayed below where uppercase words refer to the syntax kind, the numbers separated by `..` indicate the span in the source code, and syntax-nodes have their children indented while terminals have the string they span in double-quotes.

%%```
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
%%
