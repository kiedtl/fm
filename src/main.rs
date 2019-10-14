mod fm;

use std::env;

const VERS: &str = "0.0.3";

fn help() {
    println!("
fm {}
MIT (c) Kied Llaentenn

fm is a terminal-based mathematical expression evaluator.

Usage:
    [ARGS=0] fm [EXPRESSION]

Arguments:
    FD_DEBUG    Enable debug statements. (Note: FD_DEBUG only has to be set, the value does not matter.)
    FD_WARN     Enable warning statements. (Note: FD_WARN just has to be set, the value doesn't  matter.)

Operators:
    +           Addition.
    -           Subtraction.
    *           Multiplication.
    /           Division.
    %           Modulus.
    ^           Raise to power.
    !           Factorial
    nrt         Root (e.g., \"3 nrt 27\" means 'calculate the cube root of 27')
    log         Logarithm

Examples:
    $ FD_DEBUG=1 fm \"21 + 21 + 21\"
        (Show debug statements while calculating 21*3)
    $ fm 234 ^ 89
        (Calculate 234 to the power of 89)

Other:
    --help|-help|-h|help|?
        Display this help message.

    --version|-version|-v|version
        Display version and exit.
", VERS);
}

fn version() {
    println!("fm {}", VERS);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
   
    // Display help if there aren't any args.
    if args.len() < 1 { 
        help();
        std::process::exit(1);
    }

    // print help and exit if first arg is ?
    // print version if first arg is -v
    match args[0].as_ref() {
        "--help" | "-h" | "-help" | "help" | "?" => {
            help();
            std::process::exit(1);
        }
        "--version" | "-v" | "-version" | "version" => {
            version();
            std::process::exit(1);
        }
        _ => {},
    }

    println!("{}", fm::calculate(&args));
}
