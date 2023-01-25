# toki-monsi-rust

A Rust implementation of the [`toki-monsi`](https://github.com/formicant/toki-monsi) Toki Pona palindrome generator.


## Usage

```
toki-monsi-rust [OPTIONS] <max-word-count>
```

| Argument           |                               |
|--------------------|-------------------------------|
| `<max-word-count>` | Maximum palindrome word count |

| Options                   |                |                    |
|---------------------------|----------------|--------------------|
| `-o`, `--output <output>` | Output path    | `stdout` if absent |
| `-s`, `--sort <sort>`     | Result sorting | unsorted if absent |
- | Sort options        |
  |---------------------|
  | `-s A`, `-s alphabetical` |
  | `-s L`, `-s length`       |
  | `-s W`, `-s word-count`   |

| flags             |                            |
|-------------------|----------------------------|
| `-h`, `--help`    | Prints help information    |
| `-V`, `--version` | Prints version information |


## Performance

No sorting, no grammar parsing:

|words ⩽|     count | time, s |
|------:|----------:|--------:|
|     1 |         5 |   0.006 |
|     2 |        32 |   0.006 |
|     3 |       171 |   0.006 |
|     4 |       840 |   0.007 |
|     5 |      4042 |   0.007 |
|     6 |     19544 |   0.009 |
|     7 |     93782 |   0.017 |
|     8 |    449797 |   0.061 |
|     9 |   2154033 |   0.263 |
|    10 |  10310145 |   1.261 |
|    11 |  49309893 |   6.075 |
|    12 | 235782933 |  58.860 |
