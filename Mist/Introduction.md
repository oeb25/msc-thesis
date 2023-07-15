---
tags: chapter
---

With ubiquitous computing entering all parts of society and dependence on software systems ever-growing, the importance of their reliability and correctness goes hand in hand.^[#todo: why is it important?] Gaining confidence in the safety of a program can be difficult. It is challenging and laborious for developers to establish trust in our written software, but nevertheless a crucial task. Foundationally, formal proofs can give systems mathematical guarantees, but authoring such proof is not trivial and therefore needs tools to assist. Program verification is such a tool that uses automated logical reasoning to formally check the correctness of programs. However, even with such tools, the burden of verification still needs to improve for mass adoption. Thus, with an increase in the usability of verification tools, direct influence is expected on which programs are subject to verification, both from a feasibility and economic standpoint. The problem is two-fold: lower the barrier of entry to formal verification through tooling and strengthen the formal foundation of program verification to broaden its usage.

Verification infrastructure sits as the basis for program verification. Viper [[@mullerViperVerificationInfrastructure2016]] is a collection of projects that allow verifying procedural and functional programs using quantified permission-based reasoning. Encoding programs for verification in such systems can give exceptional guarantees about various properties of those programs. For such tools to be efficient, they employ certain tricks to eliminate unbounded search spaces, such as introducing _predicates_ as boundaries of recursive definitions. [[@parkinsonSeparationLogicAbstraction2005]] [[@mullerViperVerificationInfrastructure2016#Section~2.3]] Usage of predicates requires explicit instantiation and application, known as _fold_ and _unfold_, to avoid potentially infinite unrollings. Dealing with such foldings, however, can be time-consuming and error-prone. To mitigate this downside, we formalize a method for inferring these foldings using static analysis techniques, making it automatic and thus more ergonomic for verification.

In the spirit of ergonomics, language design and tooling are core for enabling productive development environments. The Rust programming language has set a new standard for the combination of developer and runtime efficiency, in no small part thanks to its high emphasis on tooling and safety. Rust-analyzer is a model example of what is possible when building language and editor integration. The Rust project has empowered developers in the otherwise daunting image systems programming have had, and been a catalyst for a new wave of low-level programming languages, like Zig [[@kelleyHomeZigProgramming]] and Vale [[@ovadiaValeFirstPrototype2023]]. We explore what it would entail to replicate this success in the field of program verification.

%%
til intro

Måske en enkelt sætning om hvorfor det en crucial task at vi har sikre programmer (et eksempel på hvordan det ellers kan gå galt i værste fald - flystyrt, attack you name it) - hjælper med at understrege hvorfor det altså er nødvendigt

“For such tools to be efficient…”: Måske sige de employer certain tricks der kræver human interaction?

%%