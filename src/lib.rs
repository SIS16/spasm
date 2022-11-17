use std::fs;
use ansi_term::Colour;
use std::{collections::HashSet, path::PathBuf};

mod parse;
mod token;

#[derive(Debug)]
#[allow(dead_code)]
pub struct AssemblerArguments {
    pub file_name: String,
    pub output_path: String,
    pub debug: bool,
    pub defines: HashSet<String>,
}

pub fn assemble_file(args: AssemblerArguments) {
    let path = PathBuf::from(args.file_name);

    // Check if input file exists
    if !&path.exists() {
        println!("Path {path:?} does not exist!");
        std::process::exit(1);
    }

    // Read entire file
    let content = fs::read(&path).expect("Could not read file");

    // Parse the file as a utf-8 encoded string
    let content = String::from_utf8(content).expect("Could not parse file as utf-8");

    // Map the file contents into a Vec of lines
    let lines: Vec<_> = content.lines().map(|string| string.to_owned()).collect();

    // Lex the file into a token vector
    let mut tokens = token::tokenize_lines(&path, &lines);

    // Build the program from the token vector
    let program = parse::build_program(&path, &lines, &mut tokens);

    println!("{program:#?}")

    // TODO - Resolve all labels

    // TODO - Compile into binary output file
}

pub fn report_error(
    error: &str,
    path: &PathBuf,
    lines: &Vec<String>,
    line_number: u32,
    col_start: u32,
    col_end: u32,
) -> ! {
    // Print error message
    eprintln!(
        "{} {}",
        Colour::Red.bold().paint("[ERROR]"),
        Colour::Red.paint(error)
    );

    // Print the file path with the line and col number
    eprintln!(
        "{}",
        Colour::Fixed(246).paint(format!(
            "{}:{}:{}",
            // Conanicalization is platform specific
            if cfg!(target_os = "windows") {
                let path = fs::canonicalize(path).unwrap();
                path.to_str()
                    .unwrap()
                    .trim_start_matches("\\\\?\\")
                    .to_owned()
            } else {
                let path = fs::canonicalize(path).unwrap();
                path.to_str().unwrap().to_owned()
            },
            line_number + 1,
            col_start + 1
        ))
    );

    // Print the lines around and including the one with the error
    let start = if line_number < 2 { 0 } else { line_number - 2 };

    for n in start..line_number + 1 {
        eprintln!(
            "{}: {}",
            Colour::Blue.paint(format!("{:>3}", n + 1)),
            lines.get(n as usize).unwrap()
        );
    }

    // Print the space before the highlight
    for _ in 0..col_start + 5 {
        eprint!(" ");
    }

    // Print the underline highlight
    for _ in col_start..col_end {
        eprint!("{}", Colour::Red.paint("^"));
    }

    eprintln!("");

    // Print the space before "here"
    for _ in 0..col_start + 5 {
        eprint!(" ");
    }

    eprintln!("{}", Colour::Red.paint("here"));

    // Exit with non-zero code to signal an error occurred
    std::process::exit(1);
}
