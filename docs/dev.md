# Development

## Contributing

Want to contribute? Thank you very much~\
Here is what you need to know.

-   Engine: [`main.rs`](../src/main.rs)
-   Flags: [`flags.rs`](../src/flags.rs)
-   Colors: [`colors.rs`](../src/colors.rs)

> After changing a file, don't forget to run `cargo check` to verify if it's all good 😄

### Adding a flag

It's easy! You simply need to `insert` it in the hashmap of flags, as following:

```rs
...

pub static ref FLAGS: HashMap<&'static str, Vec<color::RGBColor>> = {
    let mut h = HashMap::new();
    ... // stuff before

    // Here is where you put your flag!
    h.insert(
        "flag_name",
        vec![
            color::COLOR_1,
            color::COLOR_2,
            ...
        ],
    );
    h // be sure to put your line BEFORE this one!!
};

...
```

### Adding a color

You might need other colors that are currently not present.

**File structure**

The file has two main blobs: the flag-specific colors (which start with `PRIDE_`, `GAY_`, ...) and the HTML colors that are not prefixed.

> The color you want to add is probably flag-specific, so it should go in the first block.

**Flag-specific colors blob ordering**

You might notice that it is sorted from red to violet, with white-grey-black colors before -- it would be nice if you follow that pattern!

**Writing the line of code**

You need to have the RGB value of your color, e.g. `(205, 92, 92)`.

Now, you can add it, like that:

```rs
pub const <FLAG NAME>_<COLOR NAME>: RGBColor = <the RGB value>;
```

Great! You can now use it in your flag by doing `color::<your color>`, with `<your color>` being the `<FLAG NAME>_<COLOR NAME>` mentioned above.

### Modifying the engine

Currently, the [engine](#current-engine) is kind of limited and only allows to print plain lines of colors ; this means that flags like demisexual or intersex cannot be printed... 😞

However, feel free to remake the engine to make it support those!

If you do so, make sure to have one function for one task, like the current engine does.

## Current engine

### `make_flag`

Given an array of bands (a vector filled with 3-tuples of `u8`), generates the bands using [`make_band`](#make_band) and sticks them together with a newline character.

```rs
fn make_flag(flag: Vec<color::RGBColor>) -> String
```

Note: `color::RGBColor` is an alias of `(u8, u8, u8)` for convenience.

### `make_band`

Retreives the current terminal size and creates a fitting band of spaces, with a background color generated with [`make_esc`](#make_esc).

```rs
fn make_band(r: u8, g: u8, b: u8) -> String
```

### `make_esc`

Generates the escape sequence based on the color RGB values.

```rs
fn make_esc(r: u8, g: u8, b: u8) -> String
```

### `get_flag`

Given a flag name or alias, returns the array of color tuples corresponding to it (wrapped in `Some`).

If the flag name or alias does not exist, returns `None`.

```rs
fn get_flag(name: &str) -> Option<Vec<color::RGBColor>>
```
