---
tags: section
---

### Analysis of [[Case study – Ascending sum]]

#todo

```{.times unit="ms" .total .percentage}
Backend,Compiling, Starting Viper, Verification
Carbon,2.4120916,812.9068794,2586.5077792
Silicon,2.6053918,778.213308,3813.904792
```

```{.times unit="s"}
Program,Carbon,Silicon
Generated,5.3,2.3
Hand written,1.8,1.8
```

```{.times unit="s"}
command,mean,stddev,median,user,system,min,max
mist-cli viper --backend silicon examples/sorted.mist,4.5694925483399995,0.05097872529418713,4.5579094189400005,0.006067399999999999,0.00977726,4.48684625244,4.65249766944
mist-cli viper --backend carbon examples/sorted.mist,3.25388258164,0.04611581038007451,3.25593104394,0.0051633,0.0078007599999999995,3.18217758544,3.30975841944
```

- Carbon:
    - Hand optimized: 1.8s
    - Generated: 5.3s
- Silicon:
    - Hand optimized: 1.8s
    - Generated: 2.3s
