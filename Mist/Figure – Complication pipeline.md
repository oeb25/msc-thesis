---
tags: figure
---

```tikz
\usetikzlibrary{positioning}
\usetikzlibrary{fit,calc,backgrounds}
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

\def\plw{23em}
\def\sep{1em}
\def\ysep{1em}
\def\stwo{(\plw - \sep*1)/2}
\def\sthree{(\plw - \sep*2)/3}
\def\sfour{(\plw - \sep*3)/4}
\def\sfive{(\plw - \sep*4)/5}

\begin{tikzpicture}[
    node distance=\ysep and \sep,
    mod/.style 2 args = {
		local bounding box=#1,
		execute at end scope = {
			\begin{scope}
				\node[draw, fit=(#1), rounded corners, dotted] (#1) {};
				\node[left=of #1, left-mod-title] {\footnotesize \texttt{#2}};
			\end{scope}
	    }
    },
    left-mod-title/.style = {anchor=north, rotate=90, yshift=.1em, minimum height=1.2em},
    right-mod/.style 2 args = {
		local bounding box=#1,
		execute at end scope = {
			\begin{scope}
				\node[draw, fit=(#1), rounded corners, dotted] (#1) {};
				\node[right=of #1, right-mod-title] {\footnotesize \texttt{#2}};
			\end{scope}
	    }
    },
    right-mod-title/.style = {anchor=north, rotate=-90, yshift=.1em, minimum height=1.2em},
    bottom-mod/.style 2 args = {
		local bounding box=#1,
		execute at end scope = {
			\begin{scope}
				\node[draw, fit=(#1), rounded corners, dotted] (#1) {};
				\node[below=of #1, bottom-mod-title] {\footnotesize \texttt{#2}};
			\end{scope}
	    }
    },
    bottom-mod-title/.style = {anchor=north, yshift=1em, minimum height=1.2em},
    stage/.style = {
	    draw,
	    rounded corners,
	    inner sep = 1em,
	    minimum width = \plw,
    },
    two-stage/.style = {
	    draw,
	    rounded corners,
	    inner sep = 1em,
	    minimum width = \stwo,
    },
    three-stage/.style = {
	    draw,
	    rounded corners,
	    inner sep = 1em,
	    minimum width = \sthree,
    },
    four-stage/.style = {
	    draw,
	    rounded corners,
	    inner sep = 1em,
	    minimum width = \sfour,
    },
    five-stage/.style = {
	    draw,
	    rounded corners,
	    inner sep = 1em,
	    minimum width = \sfive,
    },
    trans/.style = {
	    ->,
    },
    vps-trans/.style = {
	    ->, rounded corners
    },
    dep/.style = {
	    ->, dashed, rounded corners
    },
    transl/.style = {
	    align=center,
	    font=\footnotesize,
	    midway,
	    %below,
	    %yshift=-.9cm,
    },
    ast/.style = { dashed, inner sep=0.5em },
    hir/.style = { },
    item/.style = { dashed, inner sep=0.5em },
    mir/.style = {  },
    mir-loop/.style = { in=10,out=-10,loop,looseness=2 },
    vps/.style = { dashed, inner sep=0.5em },
    lsp/.style = {
	    draw,
	    rounded corners,
	    inner sep = 1em,
	},
]
	\node[stage] (source-file) {Source file};

	\begin{scope}[
		mod={syntax}{syntax}
	]
		\node[stage, below=of source-file, yshift=-\ysep] (cst) {CST};
		\node[stage, ast, below=of cst] (ast) {AST};
	\end{scope}

	\begin{scope}[
		mod={core}{core}
	]
		\node[four-stage, below=of ast.south west, xshift=\sfour/2, yshift=-\ysep] (def-1) {Def};
		\node[four-stage, right=of def-1] (def-2) {Def};
		\node[four-stage, right=of def-2] (def-3) {Def};
		\node[four-stage, right=of def-3] (def-4) {Def};

		\node[four-stage, hir, below=of def-2] (hir-2) {HIR};
		\node[four-stage, hir, left=of hir-2] (hir-1) {HIR};
		\node[four-stage, hir, right=of hir-2] (hir-3) {HIR};
		\node[four-stage, hir, right=of hir-3] (hir-4) {HIR};

		\node[five-stage, item, below=of hir-1.south west, xshift=\sfive/2] (item-1) {Item};
		\node[five-stage, item, right=of item-1] (item-2) {Item};
		\node[five-stage, item, right=of item-2] (item-3) {Item};
		\node[five-stage, item, right=of item-3] (item-4) {Item};
		\node[five-stage, item, right=of item-4] (item-5) {Item};

		\node[four-stage, mir, below=of item-1.south west, xshift=\sfour/2] (mir-1) {MIR};
		\node[four-stage, mir, right=of mir-1] (mir-2) {MIR};
		\node[four-stage, mir, right=of mir-2] (mir-3) {MIR};
		\node[four-stage, mir, right=of mir-3] (mir-4) {MIR};
	\end{scope}


	\begin{scope}[
		mod={cg-vpr}{cg-vpr}
	]
		\node[three-stage, below=of mir-1.south west, minimum width=\sthree*2+\sep, yshift=-\ysep, xshift=(\sthree*2+\sep)/2] (vpr-1) {VPR};
		\node[three-stage, right=of vpr-1] (vpr-2) {VPR};
	\end{scope}



	\begin{scope}[
		bottom-mod={vpsvr}{viperserver}
	]
		\node[two-stage, vps, below=of vpr-1.south west, xshift=\stwo/2, yshift=-\ysep] (vps-1) {\texttt{carbon}};
		\node[two-stage, vps, right=of vps-1] (vps-2) {\texttt{silicon}};
	\end{scope}



	\draw[trans] (source-file) -> (cst);
	\draw[trans] (cst) -> (ast);

	\draw[trans] ([yshift=2*\ysep]def-1.north) -> (def-1);
	\draw[trans] ([yshift=2*\ysep]def-2.north) -> (def-2);
	\draw[trans] ([yshift=2*\ysep]def-3.north) -> (def-3);
	\draw[trans] ([yshift=2*\ysep]def-4.north) -> (def-4);

	\draw[trans] (def-1) -> (hir-1);
	\draw[trans] (def-2) -> (hir-2);
	\draw[trans] (def-3) -> (hir-3);
	\draw[trans] (def-4) -> (hir-4);

	\draw[dep] ([xshift= \sep]def-2.south west) |- ([xshift= \sep,yshift=-\ysep/2]def-2.south west) -| ([xshift=-\sep]hir-1.north east);
	\draw[dep] ([xshift=-\sep]def-2.south east) |- ([xshift=-\sep,yshift=-\ysep/2]def-2.south east) -| ([xshift= \sep]hir-3.north west);
	\draw[dep] ([xshift= \sep]def-4.south west) |- ([xshift= \sep,yshift=-\ysep/2]def-4.south west) -| ([xshift=-\sep]hir-3.north east);

	\draw[trans] (hir-1) |- ([yshift=-\ysep/2] hir-1.south) -| (item-1);
	\draw[trans] (hir-2) |- ([yshift=-\ysep/2] hir-2.south) -| (item-2);
	\draw[trans] (hir-2) |- ([yshift=-\ysep/2] hir-2.south) -| (item-3);
	\draw[trans] (hir-3) |- ([yshift=-\ysep/2] hir-3.south) -| (item-4);
	\draw[trans] (hir-4) |- ([yshift=-\ysep/2] hir-4.south) -| (item-5);

	\draw[trans] (item-1) |- ([yshift=-\ysep/2] item-1.south) -| (mir-1);
	\draw[trans] (item-2) |- ([yshift=-\ysep/2] item-2.south) -| (mir-2);
	\draw[trans] (item-3) |- ([yshift=-\ysep/2] item-3.south) -| (mir-3);
	\draw[trans] (item-4) |- ([yshift=-\ysep/2] item-4.south) -| (mir-3);
	\draw[trans] (item-5) |- ([yshift=-\ysep/2] item-5.south) -| (mir-4);

	\path[->] (mir-1) edge [mir-loop] node {} (mir-1);
	\path[->] (mir-2) edge [mir-loop] node {} (mir-2);
	\path[->] (mir-3) edge [mir-loop] node {} (mir-3);
	\path[->] (mir-4) edge [mir-loop] node {} (mir-4);

	\draw[trans] (mir-1) |- ([yshift=-\ysep] mir-1.south) -| ([xshift=-\sthree]vpr-1);
	\draw[trans] (mir-2) |- ([yshift=-\ysep] mir-2.south) -| (vpr-1);
	\draw[trans] (mir-3) |- ([yshift=-\ysep] mir-3.south) -| ([xshift= \sthree]vpr-1);
	\draw[trans] (mir-4) |- ([yshift=-\ysep] mir-4.south) -| (vpr-2);
	
	%\draw[vps-trans] ([yshift= 2*\ysep] vps-1.north) -- (vps-1.north);
	%\draw[vps-trans] ([xshift=\stwo*1/6]vpr-1.south) |- ([xshift=\stwo*1/6,yshift=-\ysep*3/4] vpr-1.south) -| ([xshift=-\stwo*1/6]vps-2.north);
	%\draw[vps-trans] ([xshift=-\stwo*1/6]vpr-2.south) |- ([xshift=-\stwo*1/6,yshift=-\ysep*5/4] vpr-2.south) -| ([xshift= \stwo*1/6]vps-1.north);
	%\draw[vps-trans] ([xshift= \stwo*1/6]vpr-2.south) -- ([xshift= \stwo*1/6,yshift=-2*\ysep] vpr-2.south);

	\draw[->] (vpr-1.south) -> ([yshift=-\ysep*2+.3em]vpr-1.south);
	\draw[->] (vpr-2.south) -> ([yshift=-\ysep*2+.3em]vpr-2.south);

	\begin{scope}[
		bottom-mod={lsp}{lsp}
	]
		\path let \p1=($(source-file.north)-(vps-1.south)$) in node(ls)[right=of source-file.north east,anchor=north west, inner sep=0, minimum height=\y1-\pgflinewidth,lsp,xshift=\sep] {\rotatebox{-90}{Language Server}};
	 \end{scope}

	\node[draw, rounded corners, right=of ls.east, anchor=west,xshift=\sep, inner sep=0.5em] (editor) {\rotatebox{-90}{Editor}};

	\draw[<-] (source-file.east) -> ([xshift=2*\sep]source-file.east);
	\draw[->] (syntax.east) -> ([xshift=2*\sep-.35em]syntax.east);
	\draw[->] (core.east) -> ([xshift=2*\sep-.45em]core.east);
	\draw[->] (vpsvr.east) -> ([xshift=2*\sep-.35em]vpsvr.east);

	\draw[<-] ([xshift=-2*\sep,yshift= \ysep/3]editor.west) -> ([yshift= \ysep/3]editor.west);
	\draw[->] ([xshift=-2*\sep,yshift=-\ysep/3]editor.west) -> ([yshift=-\ysep/3]editor.west);
\end{tikzpicture}
\end{document}
```

> [!caption]
> The compilation pipeline visualized.
