#+TITLE: Theme Generator

* Introduction

"Theme Generator," is designed to automate the process
of generating color themes for Emacs. It relies on the `srgn` command-line
program and takes as input an Emacs theme file (`name-theme.el`) and a JSON file
containing color information.

The user has to open the theme he desires to create color permutations from
and replace the name of the theme with srgn-name and the color information
with srgn-color.

;; Replace:

``` emacs-lisp
(deftheme modus-vivendi-tinted
```
;; With:

``` emacs-lisp
(deftheme srgn-name
```

;; Replace for modus themes:

``` emacs-lisp

      (bg-main          "#0d0e1c")
      (bg-dim           "#1d2235")
      (fg-main          "#ffffff")
      (fg-dim           "#989898")
      (fg-alt           "#c6daff")
      (bg-active        "#4a4f69")
      (bg-inactive      "#2b3045")
      (border           "#61647a")


```
;; With:

``` emacs-lisp
srgn-color
```

;; Replace for doom themes:

``` emacs-lisp
  ((bg      '("#33261d"))
(bg-alt      '("#291f19"))
(base0      '("#815a36"))
(base1      '("#c19958"))
(base2      '("#573f2a"))
(base3      '("#94918c")) 
(base4      '("#a93c19"))
(base5      '("#575855"))
(base6      '("#606556"))
(base7      '("#999d7f"))
(base8      '("#94918c"))
```

;; With:

``` emacs-lisp
srgn-color
```

## Note
Inside the theme, all occurences of the original name, have to be replaced
with srgn-name. The name is derived from the original filename it gets as 
a seed for the permutations with the -f flag. It generates new names for the 
new files. It generates 6 new files, with new names like that:


```bash
theme-generator -f emacs-theme.el -j colors.json 
emacs0-theme.el
emacs1-theme.el
emacs2-theme.el
emacs3-theme.el
emacs4-theme.el
emacs5-theme.el
emacs6-theme.el
```

## Note
The json file has to have at least 10 colors, or it will crash.
The program has to be recompiled to work for doom themes, right now it has 
the option to generate only modus themes, the function to generate doom themes
has to be uncommented, and the function to generate modus themes has to be commented
to work for doom themes. 

* Dependencies

This project has the following dependencies:

- srgn 
[srgn](https://github.com/alexpovel/srgn): Code Surgeon 

* Usage

To use the Theme Generator, follow these steps:

1. Install the `srgn` command-line program.
2. Compile and build the Rust project.

``` bash
cargo build --release
```

3. Run the executable with the necessary arguments.

``` bash
theme-generator/target/release/theme-generator -f name-theme.el -j colorthief_output.json
```

The command-line interface supports the following options:

- `-j, --json <PATH>`: Path to the JSON file containing color information.
- `-f, --file <PATH>`: Path to the input Emacs theme file (`name-theme.el`).

* Input

The Theme Generator requires two input files:

1. `name-theme.el`: An Emacs theme file ending in `-theme.el`.
2. `colors.json`: A JSON file containing color information in the following format:

#+BEGIN_EXAMPLE
{
  "dominant_color": "#9f3733",
  "palette": [
    "#622d22",
    "#945046",
    "#a79f9d",
    "#524a54",
    "#dcb4ac",
    "#dca088",
    "#e18f12",
    "#675e5c",
    "#9d8895"
  ]
}
#+END_EXAMPLE

* Output

The Theme Generator performs the following tasks:

1. Replaces occurrences of `srgn-name` in the `name-theme.el` file with the
   filename.
2. Replaces occurrences of `srgn-colors` in the `name-theme.el` file with the
   colors found in the `colors.json` file.
3. Generates several permutations (6 for now) of colors for the theme, doom
   theme, or modus theme. It creates several new files.

* License

This project is licensed under the [MIT
License](https://opensource.org/licenses/MIT).

