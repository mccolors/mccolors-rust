# mccolors-rust

`mccolors-rust` is a Rust library for handling Minecraft-style color codes in your terminal text. Easily add colorful and expressive text to your Rust applications for a visually appealing user experience.

## Installation

You can include `mccolors-rust` in your Rust project by adding it to your `Cargo.toml`:

```toml
[dependencies]
mccolors-rust = "0.1.1"
```

## Usage

```rust
use mccolors_rust::{mcwrite, mcreplace, mcdelete};

// Writing text with Minecraft color codes
mcwrite("&4Hello &rWorld!");

// Replacing Minecraft color codes with their respective colors
let formatted_text = mcreplace("&6Formatted &ctext &athat &3shines!");

// Removing Minecraft color codes from text
let clean_text = mcdelete("&eClean &7this &ftext.");
```

## Functions

### `mcwrite(text, reset_all=True)`
Allows writing text with Minecraft color codes in the terminal.

### `mcreplace(text, reset_all=True)`
Replaces Minecraft color codes in the text with their respective colors.

### `mcremove(text)`
Removes Minecraft color codes from the text, leaving clean text.

## Parameters

- `text`: The input text containing Minecraft color codes.

## Contributions and Issues

Contributions and suggestions are welcome! If you encounter any issues or want to contribute, please open an issue or pull request on [GitHub](https://github.com/wrrulos/mccolors-rust).

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
