---
tags: paragraph
---

In this thesis, we explore the combination of emphasis on usability and tooling with the power and formal foundations of formal verification projects such as Viper. The remainder of the thesis is organized as follows:

- We cover the necessary background information for this thesis in [[Background]].
- We present in [[The Mist Programming Language]] a new programming language, Mist, which supports formal verification constructs with a high emphasis on usability, abstraction, and expressivity.
- We present in [[The Mist Compiler]] an implementation of the compilation infrastructure for Mist based on incremental compilation and verification, with accompanying editor integration. %%with an accompanying implementation of the Language Server Protocol.%%
- In [[Automatic folding of isorecursive structures]], we formalize an algorithm for computing fold- and unfold-points for isorecursive structures and present an analysis for automatically augmenting programs Mist to make all resource access well-defined.
- We present and study a few exemplary cases in [[Case studies]], exercising and highlighting the capabilities of the Mist language.
- We conclude with related and future work in [[Conclusion]].

 %%become well-defined in terms of accessibility.%% %%and give a formally verified implementation of the algorithm in Mist.%%