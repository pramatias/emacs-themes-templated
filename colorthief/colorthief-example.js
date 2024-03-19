#!/usr/bin/env node

// Import the ColorThief module
const ColorThief = require('colorthief');

// Function to convert RGB values to hexadecimal format
function rgbToHex(rgbArray) {
    return '#' + rgbArray.map(component => {
        const hex = component.toString(16);
        return hex.length === 1 ? '0' + hex : hex;
    }).join('');
}

// Get the path to the image file from the command line arguments
const imgPath = process.argv[2]; // The first argument after 'node' and the script name

if (!imgPath) {
    console.error('Please provide the path to the image file.');
    process.exit(1); // Exit the process with an error code
}

// Function to generate JSON object
async function generateColorJSON(imgPath) {
    try {
        // Get the dominant color of the image
        const dominantColor = await ColorThief.getColor(imgPath);

        // Get a palette of colors from the image
        const palette = await ColorThief.getPalette(imgPath, 10);

        // Convert RGB values to hexadecimal format
        const dominantColorHex = rgbToHex(dominantColor);
        const paletteHex = palette.map(rgb => rgbToHex(rgb));

        // Create JSON object
        const colorJSON = {
            dominant_color: dominantColorHex,
            palette: paletteHex
        };

        return colorJSON;
    } catch (err) {
        console.error('Error:', err);
        return null;
    }
}

// Call the function and log the JSON object
generateColorJSON(imgPath)
    .then(colorJSON => {
        if (colorJSON) {
            console.log(JSON.stringify(colorJSON, null, 2));
        } else {
            console.log('Failed to generate color JSON.');
        }
    })
    .catch(err => {
        console.error('Error:', err);
    });
