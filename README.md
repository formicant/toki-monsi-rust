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
|     1 |         5 |   0.008 |
|     2 |        32 |   0.008 |
|     3 |       171 |   0.009 |
|     4 |       840 |   0.010 |
|     5 |      4042 |   0.012 |
|     6 |     19544 |   0.016 |
|     7 |     93782 |   0.035 |
|     8 |    449797 |   0.122 |
|     9 |   2154033 |   0.587 |
|    10 |  10310145 |   2.739 |
|    11 |  49309893 |  13.874 |
|    12 | 235782933 |  78.777 |
