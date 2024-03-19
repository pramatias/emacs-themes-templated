# ColorThief

ColorThief is a JavaScript library that allows you to extract color palettes and
dominant colors from images. It can be used both in the browser and in Node.js
environments.

For more information about ColorThief and its capabilities, visit the
[ColorThief homepage](https://lokeshdhakar.com/projects/color-thief/). You can
upload an image to visualize its colors in addition to generating it's JSON
values from the script.

## Usage

ColorThief can be used in Node.js to extract color information from images. After installing the package, you can use it from the command line as follows:

```bash
colorthief /path/to/image.jpg > colors.json

```

## Installation

To use ColorThief you can install it via npm:

```bash
npm i --save colorthief
npm install pngjs
npm install cwise-compiler
npm cache clean --force
npm install
npm update
```

Now colorthief is installed. You can symlink the executble 

```bash
 ln -s path/to/repo/colorthief/colorthief-example.js colorthief
```

#It outputs json like this: 10 values

``` json
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

```
