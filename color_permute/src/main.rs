use clap::{App, Arg};
use std::path::{Path, PathBuf};
use std::io::{self, BufRead, BufReader, Write};
use std::fs;
use std::io::{ Error, ErrorKind, Read};
use std::process::{Command, Stdio};
use std::fs::OpenOptions;
use std::io::{SeekFrom, Seek};
use std::fs::File;

mod color_permutations;

use color_permutations::process_color_permutations;

fn main() {

    let matches = App::new("Color Permute")
        .arg(
            Arg::with_name("file")
                .short('f')
                .long("file")
                .required(true)
                .takes_value(true)
                .help("Input file"),
        )
        .arg(
            Arg::with_name("red")
                .short('r')
                .takes_value(true)
                .conflicts_with("all")
                .help("Red color"),
        )
        .arg(
            Arg::with_name("green")
                .short('g')
                .takes_value(true)
                .conflicts_with("all")
                .help("Green color"),
        )
        .arg(
            Arg::with_name("yellow")
                .short('y')
                .takes_value(true)
                .conflicts_with("all")
                .help("Yellow color"),
        )
        .arg(
            Arg::with_name("blue")
                .short('b')
                .takes_value(true)
                .conflicts_with("all")
                .help("Blue color"),
        )
        .arg(
            Arg::with_name("magenta")
                .short('m')
                .takes_value(true)
                .conflicts_with("all")
                .help("Magenta color"),
        )
        .arg(
            Arg::with_name("cyan")
                .short('c')
                .takes_value(true)
                .conflicts_with("all")
                .help("Cyan color"),
        )
        .arg(
            Arg::with_name("all")
                .short('a')
                .long("all")
                .help("Use all colors"),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();

    let colors = if matches.is_present("all") {
        vec![
            "red".to_string(),
            "green".to_string(),
            "yellow".to_string(),
            "blue".to_string(),
            "magenta".to_string(),
            "cyan".to_string(),
        ]
    } else {
        let mut colors = vec![];
        if let Some(color) = matches.value_of("red") {
            colors.push(get_color(Some(color)));
        }
        if let Some(color) = matches.value_of("green") {
            colors.push(get_color(Some(color)));
        }
        if let Some(color) = matches.value_of("yellow") {
            colors.push(get_color(Some(color)));
        }
        if let Some(color) = matches.value_of("blue") {
            colors.push(get_color(Some(color)));
        }
        if let Some(color) = matches.value_of("magenta") {
            colors.push(get_color(Some(color)));
        }
        if let Some(color) = matches.value_of("cyan") {
            colors.push(get_color(Some(color)));
        }
        colors
    };

    // Print the collected colors
    println!("Collected colors:");
    for color in &colors {
        println!("{}", color);
    }

    // Check if at least two colors are provided
    if colors.len() < 2 {
        eprintln!("Error: At least two colors must be provided");
        return;
    }
    let colors_refs: Vec<&str> = colors.iter().map(|s| s.as_str()).collect();
    let unique_perms = process_color_permutations(&colors_refs);

    for (index, perm) in unique_perms.iter().enumerate() {
        let new_file_path = copy_file_with_name(&file, index,  perm.clone());
        let _ = swap_colors_in_file(&new_file_path, perm);
        let _ = remove_last_line(&new_file_path);
    }
}

fn remove_last_line(file_path: &PathBuf) -> io::Result<()> {
    let output = Command::new("head")
        .args(&["-n", "-1"])
        .arg(&file_path)
        .output()?;

    if output.status.success() {
        // Write the output (except the last line) back to the file
        std::fs::write(&file_path, &output.stdout)?;
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to execute head command"))
    }
}


fn swap_colors_in_file(file_path: &PathBuf, colors: &[&str]) -> io::Result<()> {
    let color_pairs = generate_color_pairs(colors);

    // Print the generated color pairs
    println!("Generated color pairs:");
    for (color, next_color) in &color_pairs {
        println!("Color: {}, Next Color: {}", color, next_color);
    }

    let initial_lines = read_file_content(&file_path, 215)?;

    // Iterate over each color pair and replace colors in the content
    for (i, (color, next_color)) in color_pairs.iter().take(color_pairs.len() - 1).enumerate() {
        replace_in_file(&file_path, color, "Intermediate").expect("Failed to replace colors in file");
        replace_in_file(&file_path, next_color, color).expect("Failed to replace colors in file");
        replace_in_file(&file_path, "Intermediate", next_color).expect("Failed to replace colors in file");
        //let _ = remove_last_line(&file_path);
        // Print intermediate step
        println!("Intermediate step {} completed", i + 1);
    }

    replace_in_file(&file_path, "^ere","").expect("Failed to replace colors in file");
    write_file_content(&file_path, &initial_lines, 215)
}

fn read_file_content(file_path: &PathBuf, line_number: usize) -> io::Result<String> {
    // Open the file for reading
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    // Seek to the start of the file
    reader.seek(SeekFrom::Start(0))?;

    // Read lines until the specified line number is reached
    let mut content = String::new();
    for _ in 0..line_number {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            // Reached EOF before reaching the specified line number
            return Ok(content);
        }
        content.push_str(&line);
    }

    Ok(content)
}

fn write_file_content(file_path: &PathBuf, initial_lines: &str, line_number: u64) -> io::Result<()> {
    // Read the remaining content of the file after the specified line number
    let remaining_lines = read_remaining_lines(file_path, line_number)?;

    // Count the number of lines in initial_lines
    let initial_lines_count = initial_lines.lines().count();
    println!("Initial lines count: {}", initial_lines_count);

    // Count the number of lines in remaining_lines
    let remaining_content_count = remaining_lines.len();
    println!("Remaining content count: {}", remaining_content_count);

    // Create a new file for writing
    let mut file = OpenOptions::new().write(true).create(true).open(file_path)?;

    // Write the initial lines at the beginning of the file
    file.seek(SeekFrom::Start(0))?;
    file.write_all(initial_lines.as_bytes())?;

    // Write the remaining content after the initial lines
    for line in remaining_lines {
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?; // Add newline after each line
    }

    println!("Colors swapped in file: {:?}", file_path);

    Ok(())
}
fn read_remaining_lines(file_path: &PathBuf, line_number: u64) -> io::Result<Vec<String>> {
    // Open the file for reading
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read the lines until the specified line number
    let mut remaining_lines = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if index as u64 >= line_number {
            remaining_lines.push(line);
        }
    }

    Ok(remaining_lines)
}

fn replace_in_file(file_path: &PathBuf, color: &str, next_color: &str) -> io::Result<()> {
    // Execute the 'cat' command and capture its output
    let cat_output = Command::new("cat")
        .arg(file_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()?;

    // Check if the 'cat' command was successful
    if cat_output.status.success() {
        // Get the stdout of 'cat'
        let cat_stdout = cat_output.stdout;

        // Build the 'sed' command
        let mut sed_process = Command::new("sed")
            .arg(&format!("s/{}/{}/g", color, next_color))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;

        // Write the output of 'cat' to the stdin of 'sed'
        if let Some(stdin) = sed_process.stdin.as_mut() {
            stdin.write_all(&cat_stdout)?;
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to open stdin for sed"));
        }

        // Wait for the 'sed' command to complete
        let sed_output = sed_process.wait_with_output()?;

        // Check if 'sed' completed successfully
        if sed_output.status.success() {
            // Write the output of 'sed' to a temporary file
            let temp_file_path = "/tmp/temp_file"; // Temporary file path
            fs::write(temp_file_path, &sed_output.stdout)?;

            // Replace the original file with the temporary file
            fs::rename(temp_file_path, file_path)?;

            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "sed command failed"))
        }
    } else {
        Err(Error::new(ErrorKind::Other, "cat command failed"))
    }
}

fn generate_color_pairs<'a>(colors: &'a [&'a str]) -> Vec<(&'a str, &'a str)> {
    let mut pairs = Vec::new();
    let len = colors.len();
    for i in 0..len {
        let next_index = (i + 1) % len;
        let color_pair = (colors[i], colors[next_index]);
        pairs.push(color_pair);
        //println!("Color pair: {:?}", color_pair);
    }
    pairs
}

fn copy_file_with_name(original_file: &str, count: usize, colors: Vec<&str>) -> PathBuf {
    // Print the colors passed as arguments
    println!("Colors: {:?}", colors);

    // Split the file name into prefix and suffix
    let parts: Vec<&str> = original_file.split("-theme.el").collect();
    if parts.len() != 2 {
        panic!("Invalid file name format");
    }
    let (prefix, suffix) = (parts[0], "-theme.el");

    // Get the initials of the colors
    let colors_initials: String = colors.iter().map(|color| color.chars().next().unwrap()).collect();

    // Construct new file name with count inserted before the suffix and color initials appended
    let new_file_name = format!("{}-{}{}{}", prefix, count, colors_initials, suffix);

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

    // Construct new file name with count inserted before the suffix and color initials appended
    let new_theme_name = format!("{}-{}{}", prefix, count, colors_initials);

    // Replace original file name with new file name
    replace_in_file(&PathBuf::from(new_file_name.clone()), &prefix, &new_theme_name)
        .expect("Failed to replace original file name");

    // Return the path to the newly created file
    Path::new(&new_file_name).to_path_buf()
}

fn get_color(color: Option<&str>) -> String {
    if let Some(color) = color {
        color.to_string()
    } else {
        panic!("Color value not provided");
    }
}
