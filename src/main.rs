mod fm;

use fm::ops::Operators;

use std;
use std::env;
use std::process;
use std::collections::HashMap;

const VERS: &str = "0.0.3";

// token types
const _INT: &str = "integer";
const _OPT: &str = "operator";

fn lex(tokens: &[String]) -> Vec<String> {
    // iterate through Vector
    // and check if there are 
    // spaces in any of the tokens.
    // if so, save that token, split it,
    // and replace the original item with it.
    let mut newtokens: Vec<String> = Vec::new();
    for token in tokens {
        if token.contains(" ") {
            let split_token = token.split(" ");
            let newsplit_tokens: Vec<&str> = split_token.collect::<Vec<&str>>().to_owned();
            for newsplit_token in newsplit_tokens {
                newtokens.push(newsplit_token.to_owned());
            }
        } else {
            newtokens.push(token.to_owned());
        }
    }

    newtokens
}

fn parse(tokens: &[String]) -> Vec<HashMap<String, String>> {
    // abstract syntax table
    let mut ast: Vec<HashMap<String, String>> = Vec::new();
    for token in tokens {
        let token: &str = token.as_ref();
        // check if token is numeric
        if token.parse::<f64>().is_ok() {
            ast.push([("type".to_owned(), 
                       _INT.to_owned()), 
                     ("token".to_string(), 
                      token.to_owned())]
                     .iter()
                     .cloned()
                     .collect()
                     );
        } else {
            let operator: Operators = token.into();
            if operator == Operators::Unimplemented {
                println!("ERROR: token {} not recognized as OPT or INT! aborting.", &token);
                process::exit(1);
            }

            ast.push([("type".to_owned(), 
                       _OPT.to_owned()), 
                     ("token".to_owned(), 
                      token.to_owned())]
                     .iter()
                     .cloned()
                     .collect()
                     );
        }
    }
    debug(format!("DEBUG: AST: {:?}", ast));
    ast
}

fn factorial(x: f64) -> f64 {
    let mut val: f64 = 0.0;
    for y in 0..(x as u8) {
        val *= y as f64;
    }

    val
}

fn root(num: f64, mut base: f64) -> f64 {
    // convert base to decimal
    // e.g. if base is 5, then convert 5 to fraction, then to decimal
    // e.g. 5 = 1/5 = 0.2
    base = 1.0 / base;

    // push num to the power of base
    num.powf(base)
}

fn process(ast: &[HashMap<String, String>]) -> String {
    let mut val:     f64    = 0.0;
    let mut lastopt: Option<Operators> = None;
    // let mut lastint: f64    = 0;
    for map in ast {
        if map["type"] == _INT.to_string() {
            // check if we are already in an expression
            if lastopt.is_none() {
                if val == 0.0 {
                    val = map["token"].parse::<f64>().unwrap();
                } else {
                    warn(format!("WARN!: token {} has no operator!", map["token"]));
                }
            } else {
                // if so, calculate and add to val, then reset other values.
                match lastopt.unwrap() {
                    Operators::Add => val = val + map["token"].parse::<f64>().unwrap(),
                    Operators::Subtract => val = val - map["token"].parse::<f64>().unwrap(),
                    Operators::Multiply => val = val * map["token"].parse::<f64>().unwrap(),
                    Operators::Divide => val = val / map["token"].parse::<f64>().unwrap(),
                    Operators::Modulo => val = val % map["token"].parse::<f64>().unwrap(),
                    Operators::Exponent => val = val.powf(map["token"].parse::<f64>().unwrap()),
                    Operators::Factorial => val = factorial(val),
                    Operators::Logarithm => val = val.log(map["token"].parse::<f64>().unwrap()),
                    Operators::NRoot => val = root(val, map["token"].parse::<f64>().unwrap()),
                    Operators::Unimplemented => eprintln!("WARN: operator {} not implemented yet.", map["token"]),
                }

                // reset values
                lastopt = None;
                // lastint = 0;
            }
        }
        if map["type"] == _OPT.to_string() {
            lastopt = Some((&map["token"]).into());
        }
        debug(format!("DEBUG: val:{} opt:{:?}", val, lastopt));
    }

    val.to_string()
}

fn debug(text: String) {
    if !env::var("FD_DEBUG").is_err() {
        println!("{}", text);
    }
}

fn warn(text: String) {
    if !env::var("FD_WARN").is_err() {
        println!("{}", text);
    }
}

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

    --version|-v|version
        Display version and exit.
", VERS);
}

fn version() {
    println!("fm {}", VERS);
}

fn main() {
    let args: Vec<String> = env::args().collect();
   
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


    let answer = process(&parse(&lex(&args)));
    println!("{}", answer);
}
