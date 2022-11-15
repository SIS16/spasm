use std::{
    collections::{HashSet, VecDeque},
    env,
};

use spasm::{assemble_file, AssemblerArguments};

fn main() {
    let mut args: VecDeque<_> = env::args().collect();

    // Remove binary name from argv
    args.pop_front();

    // Parse command line arguments
    let args = parse_args(args);

    // println!("{args:?}");

    assemble_file(args);
}

/**
 * Parses assembler arguments from command line argv
 */
fn parse_args(mut args: VecDeque<String>) -> AssemblerArguments {
    let mut file_name: Option<String> = None;
    let mut output_path: Option<String> = None;
    let mut debug: bool = false;
    let mut defines: HashSet<String> = HashSet::new();

    if args.is_empty() {
        print_help_statement();
        std::process::exit(1);
    }

    while !args.is_empty() {
        // We know since the argv is not empty that we can unwrap
        let arg = args.pop_front().unwrap();

        match arg.as_str() {
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            "-o" | "--output" => {
                if args.is_empty() {
                    eprintln!("Expected file name after {arg} argument!");
                    print_help_statement();
                    std::process::exit(1);
                } else if output_path.is_some() {
                    eprintln!("Unexpected duplicate argument {arg}!");
                    print_help_statement();
                    std::process::exit(1);
                }

                output_path = Some(args.pop_front().unwrap());
            }
            "-d" | "--debug" => {
                debug = true;
            }
            "-D" | "--define" => {
                if args.is_empty() {
                    eprintln!("Expected file name after {arg} argument!");
                    print_help_statement();
                    std::process::exit(1);
                }

                defines.insert(args.pop_front().unwrap());
            }
            "-v" | "--version" => {
                println!("SPASM v{}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            _ => {
                if arg.starts_with("-") {
                    eprintln!("Unexpected option argument '{arg}'!");
                    print_help_statement();
                    std::process::exit(1);
                } else if !args.is_empty() {
                    eprintln!("Unexpected arguments after file name: {:?}", args);
                    print_help_statement();
                    std::process::exit(1);
                }

                file_name = Some(arg);
            }
        }
    }

    let file_name = match file_name {
        Some(out) => out,
        None => {
            eprintln!("Expected file name after options!");
            print_help_statement();
            std::process::exit(1);
        }
    };

    if !file_name.ends_with(".asm") {
        eprintln!("File name '{file_name}' must end with '.asm'!");
        print_help_statement();
        std::process::exit(1);
    }

    let output_path = match output_path {
        Some(out) => out,
        None => file_name.replace(".asm", ".bin"),
    };

    if output_path == file_name {
        eprintln!("Output path '{output_path}' will overwrite input path '{file_name}'!");
        std::process::exit(1);
    }

    AssemblerArguments {
        file_name,
        output_path,
        debug,
        defines,
    }
}

/**
 * Print SPASM usage
 */
fn print_usage() {
    println!("      SPASM - sis16 Assembler");
    println!("");
    println!("Usage:");
    println!("  spasm --version");
    println!("  spasm --help");
    println!("  spasm [-o out_file] [options...] file_name");
    println!();
    println!("Options:");
    println!("  -h, --help                    Prints this help dialogue");
    println!("  -o, --output <output_path>    Specifies the output file path");
    println!("  -d, --debug                   Emits debug information");
    println!("  -D, --define <variable_name>  Define a compile time variable");
    println!("  -v, --version                 Print the current version");
    println!();
    println!("Examples:");
    println!("  spasm --output main.o --debug main.asm");
}

fn print_help_statement() {
    println!("Use 'spasm --help' to see usage!")
}
