use clap::{Arg, App};
use std::fs;
use serde_json::Value;
use itertools::Itertools;
use std::path::{Path, PathBuf};
use std::process::{Command};
use std::io::{self, Error, ErrorKind};
use std::process::Stdio;
use std::io::Write;


fn main() {
    let matches = App::new("Theme Generator")
            .arg(Arg::with_name("json")
            .short('j')
            .long("json")
            .required(true)
            .takes_value(true)
            .help("Path to the JSON file"))
        .arg(Arg::with_name("file")
            .short('f')
            .long("file")
            .required(true)
            .takes_value(true)
            .help("Path to the input file"))
        .get_matches();


    let json_file = matches.value_of("json").unwrap();
    let input_file = matches.value_of("file").unwrap();

    // Read JSON file and extract dominant color and palette
    let json_content = fs::read_to_string(json_file).expect("Failed to read JSON file");
    let json: Value = serde_json::from_str(&json_content).expect("Failed to parse JSON");
    let dominant_color = json["dominant_color"].as_str().expect("Missing dominant color");
    let palette = json["palette"].as_array().expect("Missing palette").iter().map(|c| c.as_str().unwrap()).collect::<Vec<_>>();

    // Generate permutations of palette colors and write to new files
    generate_permutations(&palette, input_file, dominant_color);
}

fn generate_permutations(palette: &[&str], input_file: &str, dominant_color: &str) {
    // Generate permutations of the first three palette colors
    let permutations = palette.iter().take(3).map(|&x| x).permutations(3); // Dereference the elements

    for (i, permutation) in permutations.enumerate() {
        // Print the permutation
        println!("Permutation {}: {:?}", i, permutation);

        // Create modified content
        let modified_content = generate_modified_content(&permutation, dominant_color, palette);

        // Write modified contents to a new file
        let output_file_name = copy_file_with_name(input_file, i);

        // Process the file
        if let Err(err) = process_file(&output_file_name, "srgn-colors", &modified_content) {
            eprintln!("Error: {}", err);
        } else {
            println!("Content inserted successfully!");
        }

        // Extract filename without the "-theme.el" suffix
        if let Some(theme_file_name) = extract_theme_file_name(&output_file_name) {
            if let Err(err) = process_file(&output_file_name, "srgn-name", &theme_file_name) {
                eprintln!("Error: {}", err);
            } else {
                println!("Content inserted successfully!");
            }
        }
    }
}

fn generate_modified_content(permutation: &[&str], dominant_color: &str, palette: &[&str]) -> String {
    let mut modified_content = String::new();

    if let Some(dominant_color_string) = build_color_string(0, dominant_color) {
        modified_content.push_str(&dominant_color_string);
    } else {
        // Handle the case where build_color_string returns None for dominant color
        println!("Invalid dominant color");
        // You may want to return early or handle this case differently based on your requirements
    }

    let mut j = 1; // Start numbering from 1 for the first three colors
    for (_k, &color) in permutation.iter().enumerate() {
        if let Some(color_string) = build_color_string(j, color) {
            modified_content.push_str(&color_string);
        } else {
            // Handle the case where build_color_string returns None for palette color
            println!("Invalid palette color");
            // You may want to return early or handle this case differently based on your requirements
        }
        j += 1;
    }

    // Process the rest of the palette individually
    for color in &palette[3..] {
        if let Some(color_string) = build_color_string(j, color) {
            modified_content.push_str(&color_string);
        } else {
            // Handle the case where build_color_string returns None for palette color
            println!("Invalid palette color");
            // You may want to return early or handle this case differently based on your requirements
        }
        j += 1;
    }

    modified_content
}

fn process_file(file_path: &PathBuf, template: &str, content: &str) -> io::Result<()> {
    // Execute the 'cat' command and capture its output
    let cat_output = Command::new("cat")
        .arg(file_path)
        .output()?;

    // Check if the 'cat' command was successful
    if cat_output.status.success() {
        // Build the 'srgn' command
        let mut srgn_command = Command::new("srgn");
        srgn_command.args(&["--literal-string", template, content]);

        // Set up the 'srgn' command's stdin to receive the output of 'cat'
        let mut srgn_process = srgn_command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        // Write the output of 'cat' to 'srgn's stdin
        if let Some(ref mut stdin) = srgn_process.stdin {
            stdin.write_all(&cat_output.stdout)?;
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to open srgn's stdin"));
        }

        // Drop the stdin handle to signal EOF
        drop(srgn_process.stdin.take());

        // Read the output of 'srgn'
        let srgn_output = srgn_process.wait_with_output()?;

        // Check if 'srgn' completed successfully
        if srgn_output.status.success() {
            // Write the output of 'srgn' to a temporary file
            let temp_file_path = "/tmp/temp_file"; // Temporary file path
            fs::write(temp_file_path, &srgn_output.stdout)?;

            // Replace the original file with the temporary file
            fs::rename(temp_file_path, file_path)?;

            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "srgn command failed"))
        }
    } else {
        Err(Error::new(ErrorKind::Other, "cat command failed"))
    }
}

fn extract_theme_file_name(output_file_name: &PathBuf) -> Option<String> {
    output_file_name
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|name| name.trim_end_matches("-theme").to_string())
}

//modus vivendi template
fn build_color_string(index: usize, color: &str) -> Option<String> {
    if index > 7 {
        return None; // Ignore input if index is greater than 10
    }

    let base_color = match index {
        0 => "bg-main",
        1 => "bg-dim",
        2 => "bg-active",
        3 => "bg-inactive",
        4 => "border",
        5 => "fg-main",
        6 => "fg-dim",
        7 => "fg-alt",
        _ => panic!("Invalid base color index"),
    };

    Some(format!("({}          \"{}\")\n", base_color, color))
}

//doom themes template
// fn build_color_string(index: usize, color: &str) -> Option<String> {
//     if index > 10 {
//         return None; // Ignore input if index is greater than 10
//     }

//     let base_color = match index {
//         0 => "(bg".to_string(),
//         1 => "bg-alt".to_string(),
//         2 => "base0".to_string(),
//         3 => "base1".to_string(),
//         4 => "base2".to_string(),
//         5 => "base3".to_string(),
//         6 => "base4".to_string(),
//         7 => "base5".to_string(),
//         8 => "base6".to_string(),
//         9 => "base7".to_string(),
//         10 => "base8".to_string(),
//         _ => panic!("Invalid base color index"),
//     };

//     Some(format!("({}      '(\"{}\"))\n", base_color, color))
// }

fn copy_file_with_name(original_file: &str, count: usize) -> PathBuf {
    // Split the file name into prefix and suffix
    let parts: Vec<&str> = original_file.split("-theme.el").collect();
    if parts.len() != 2 {
        panic!("Invalid file name format");
    }
    let (prefix, suffix) = (parts[0], "-theme.el");

    // Construct new file name with count inserted before the suffix
    let new_file_name = format!("{}-{}{}", prefix, count, suffix);

    // Execute the command line `cp` utility to copy the file
    let output = Command::new("cp")
        .arg(original_file)
        .arg(&new_file_name)
        .output()
        .expect("Failed to execute command");

    // Check if there were any errors during copying
    if !output.status.success() {
        panic!("Failed to copy file: {:?}", output.stderr);
    }

    // Return the path to the newly created file
    Path::new(&new_file_name).to_path_buf()
}
