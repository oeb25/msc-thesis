---
tags: paragraph
title: Dafny
---

is a verification-aware programming language, [[@leinoDafnyAutomaticProgram2010]] which in many ways is similar to Viper and has ergonomic ambitions aligned with those of Mist, exemplified by their holistic editor integration [[@dafnyManual#Section~14]] and empowering language features. Such language features include type inference and ghost code [[@dafnyManual#Sections~12.3-4]], and _subset-types_ [[@dafnyManual#Section 5.6.3]], which are analogous to type `invariant`s in Mist. Dafny targets memory-managed languages and tracks resource access through `reads` and `modifies` attributes instead of using immutable (`&`) and mutable (`&mut`) references. Consequently, Dafny does not have quantified permissions nor a notion of passing or transferring ownership, and thus does not need folds and unfolds.

%%
- [[@dafnyManual#Section~6.4.4.]]
    - 6.4.4. Function Transparency
- By not having quantified permission, objects passed around in Dafny never looses permission%%
