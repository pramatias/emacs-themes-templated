# Color Permute

Color Permute is a command-line utility for processing color permutations.
It requires two colors as an input, and a modus file. All six colors can be included 
by using the -a flag

## Usage 

### Permute all colors in a modus file

``` sh
color-permute -f modus-theme.el -a

```

### Permute red and blue colors

``` sh
color-permute -f modus-theme.el -r red -b blue

```
### Options

``` sh
-f, --file: Specifies the input file.
-r, --red: Specifies the red color.
-g, --green: Specifies the green color.
-y, --yellow: Specifies the yellow color.
-b, --blue: Specifies the blue color.
-m, --magenta: Specifies the magenta color.
-c, --cyan: Specifies the cyan color.
-a, --all: Use all colors.
```
## Installation

To use Color Permute, you need to have Rust installed. You can install it using Rust's package manager, Cargo:

``` sh
cargo build --release
cp target/release/color-permute ~/.local/bin/
```

##License
This project is licensed under the MIT License
