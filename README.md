# toki-monsi-rust

A Rust implementation of the [`toki-monsi`](https://github.com/formicant/toki-monsi) Toki Pona palindrome generator.


## Performance

Single-threaded, no grammar parsing:

|words ⩽|    count | time, s |
|------:|---------:|--------:|
|     1 |        5 |   0.001 |
|     2 |       32 |   0.001 |
|     3 |      171 |   0.002 |
|     4 |      840 |   0.004 |
|     5 |     4042 |   0.009 |
|     6 |    19544 |   0.034 |
|     7 |    93782 |   0.153 |
|     8 |   449797 |   0.660 |
|     9 |  2154033 |   2.823 |
|    10 | 10310145 |  14.527 |
