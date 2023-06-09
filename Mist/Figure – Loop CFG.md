---
tags: figure
---

```{.mist .ignoreErrors}
                             ┌───────────────── :B0 ──────────────────┐
                             │ #1_sum := $0                           │
                             │ #2_i := $0                             │
                             │ !goto :B1                              │
                             └─┬──────────────────────────────────────┘
                               ▼
┌───────── :B5 ──────────┐   ┌───────────────── :B1 ──────────────────┐
│ #10 := (== #1_sum $45) │   │ #3 := (< #2_i $10)                     │
│ !assert #10 -> :B6     │ ◀─┤ !switch #3 { 1 -> :B2, otherwise :B5 } │◀┐
└─┬──────────────────────┘   └─┬──────────────────────────────────────┘ │
  ▼                            ▼                                        │
┌───────── :B6 ──────────┐   ┌───────────────── :B2 ──────────────────┐ │
│ !return                │   │ #4 := (- $10 #2_i)                     │ │
└────────────────────────┘   │ !goto :B3                              │ │
                             └─┬──────────────────────────────────────┘ │
                               ▼                                        │
                             ┌───────────────── :B3 ──────────────────┐ │
                             │ #5 := (+ #1_sum #2_i)                  │ │
                             │ #1_sum := #5                           │ │
                             │ #6 := (+ #2_i $1)                      │ │
                             │ #2_i := #6                             │ │
                             │ #7 := (- $10 #2_i)                     │ │
                             │ #8 := (< #7 #4)                        │ │
                             │ !assert #8 -> :B4                      │ │
                             └─┬──────────────────────────────────────┘ │
                               ▼                                        │
                             ┌───────────────── :B4 ──────────────────┐ │
                             │ #9 := (<= $0 #7)                       │ │
                             │ !assert #9 -> :B1                      ├─┘
                             └────────────────────────────────────────┘
```

> [!caption]
> The MIR CFG of the function `@f` from [[Example – Desugaring for loop]].
