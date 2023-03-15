# Prideflag

## Usage

### Compiling

Before running the program, you might need to compile it.\
Assuming you have `cargo` installed, simply run:

```
cargo build --release
```

The binary `prideflag` should now be on the path: `<project_path>/target/release/`.

You might want to have the said path on your `$PATH` to follow the next steps.

### Basic

```
prideflag
```

This prints the default flag, i.e. the rainbow.

![Output of the program, run with no args](../images/no_args.png)

### With specified flag

You can specify the full name of a flag to print it.

```
prideflag --flag lesbian
```

![Output of the program, run with `--flag lesbian`](../images/lesbian_full.png)

...or you can use the short version.<sup>1</sup>

```
prideflag --flag l
```

![Output of the program, run with `--flag l`](../images/lesbian_short.png)

...also, it's case-insensitive!

```
prideflag --flag LeSbIaN
```

![Output of the program, run with `--flag LeSbIaN` (lesbian is written with weird casing)](../images/lesbian_spongecase.png)

<sup>1</sup> In scripting, it is recommended to use the full name to avoid ambiguity.

### Other options

-   `--help` or `-h`
-   `--version` or `-V`

## Assets

### Flags

For a list of the available flags, read [this](./flags.md).

### Colors

All the HTML colors + some that are unique to the LGBTI flags.

## Development

For development, read [this](./dev.md).
