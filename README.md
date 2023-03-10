# Prideflag

Prints a pride flag in the console.

Side project for fun.

## Compile

```
cargo build --release
```

## Run

```
./target/release/prideflag
```

By default, it prints the rainbow flag, but you can specify another one:

```
./target/release/prideflag --flag lesbian
```

## Examples

**Without args**

![Output of the program, run with no args](./images/no_args.png)

**Arg `lesbian`**

![Output of the program, run with `--flag lesbian`](./images/lesbian_full.png)

**Arg `g` (alias for `gay`)**

![Output of the program, run with `--flag gay`](./images/gay_alias.png)
