---
tags: subsection
---

We have started work on emitting WebAssembly (Wasm), a portable low-level byte-code format, from the MIR representation, enabling Mist programs to run on a wide range of platforms. Wasm would be an exciting match for verified programs, as the WebAssembly specification [[@wasmSpec]] is built on well-defined behavior and formal semantics, further supported by research in that area. [[@wattMechanisingVerifyingWebAssembly2018]] [[@raoIrisWasmRobustModular2023]] The ultimate ideal of such efforts would be verified holistic compilation, with Wasm as the simple and relatively high-level intermediary target, from where verified optimizations, execution, and machine code compilation, could take place.
