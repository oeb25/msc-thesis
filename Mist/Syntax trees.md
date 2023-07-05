---
tags: subsection
---

All Mist programs start with the source code, the language described in [[The Mist Programming Language]], which then gets parsed using a hand-written recursive descent parser [[@kladovResilientLLParsing]]. For reasons further explained in [[Language server integration]], it is essential that the parser retains all information in the source code when constructing the next stage of compilation.

The first few stages of compilation, the ones revolving around the syntax, use a _red-green tree_ approach to the representation of the syntax tree, which was first introduced in the Roslyn C# compiler [[@ericlippertPersistenceFacadesRoslyn2012]]. It splits the syntax tree into two stages: one called the _green tree_ (also referred to as the _concrete-syntax tree_, see [[Concrete-syntax tree (CST)]]) and one called the _red tree_ (also referred to as the _astract-syntax tree_, see [[Abstract-syntax tree (AST)]]).
