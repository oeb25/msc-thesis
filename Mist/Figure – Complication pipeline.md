---
tags: figure
---

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