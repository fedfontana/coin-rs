# Coin-rs

Toss coins and other random utilities for the terminal

## Installation

```console
$ cargo install coin-rs
```

## Usage

If you want to toss a coin run:
```
toss
```

Which is equivalent to
```
toss 1 coin -p 0.5
```

If you want to toss a dice run
```
toss dice
```

Which is equivalent to
```
toss 1 dice -m 1 -M 6
```

If you want to let the program choose between two or more options run
```
toss 10 choose option1 option2 ...
```

For more information about the options provided, please run `toss --help` and `toss <coin | dice> --help`
