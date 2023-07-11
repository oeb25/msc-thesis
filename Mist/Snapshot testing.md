---
tags: subsection
---

_Snapshot testing_, sometimes also referred to as _expect testing_, [[@minskyTestingExpectations2015]] is a way to validate the behavior of select subsystems by writing an execution of that system and saving the result of that execution, known as a _snapshot_. When we rerun the tests, their behavior is checked against the snapshot. It is up to the developer to ensure that the initially produced result is as expected. However, the test will catch any subsequent observable changes to the system, intentional or otherwise.

During development, we often refactor or add new features and, in the process, unwillingly influence some adjacent part of the compiler. Snapshot testing makes such unintentional changes much easier to account for and lowers the burden of adding new tests since we do not have to compute and insert the results manually.

Additionally, when we want to change the behavior of a subsystem, old test cases relying on past behavior will fail. In these scenarios, manually updating tests becomes a hindrance to the development pace rather than an aid. However, With snapshot testing, existing snapshots will catch this divergence as expected, allowing us to update all snapshots to the new behavior easily.

In Mist, we use Insta [[@ronacherMitsuhikoInsta2023]] for snapshot testing, which with `cargo insta`, gives us an interactive CLI to test and review snapshot tests.

Another side effect of not manually authoring test results is that it allows for generating large test outputs with plenty of visual niceties, which aid in understanding and inevitable debugging.

![[Figure – Snapshot example]]

[[Figure – Snapshot example]] shows how we use snapshot testing in the Mist compiler. The function `@check_trace` $\lineref{3, 17}$ takes a Mist program, with special annotation in the form of `$0..$0`, and highlights all locations in the generated Viper program, which maps back to the annotated source location. This is an interesting example which highlights the power of snapshot testing. Firstly, using plain Mist code with direct inline annotations makes it easy to describe arbitrary real scenarios precisely. Secondly, displaying the actual generated output, with markers for where the spans end up, allows for validating results visually without having to reproduce the generated code and fiddle with numeric spans.

The `expect!(@###"..."###)` macro invocations $\lineref{9, 23}$ are the indications to `insta` of where snapshots should go as newly generated results from `@check_test` gets accepted. This approach to snapshot testing compiler internals and outputs is inspired by rust-analyzer [[@RustAnalyzerArchitecture2023]].
