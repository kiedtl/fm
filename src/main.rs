use std;
use std::env;
use std::process;
use std::collections::HashMap;

const VERS: &str = "0.0.3";

// token types
const _INT: &str = "integer";
const _OPT: &str = "operator";

// operators
const _OPERATORS: &'static [&'static str] = &["+",      // addition 
                                              "-",      // subtraction
                                              "*",      // multiplication
                                              "/",      // division
                                              "%",      // modulus
                                              "^",      // power
                                              "!",      // factorial
                                              "nrt",    // root
                                              "log"     // logarithm
                                             ];


fn lex(tokens: Vec<String>) -> Vec<String> {
    // iterate through Vector
    // and check if there are 
    // spaces in any of the tokens.
    // if so, save that token, split it,
    // and replace the original item with it.
    let mut newtokens: Vec<String> = Vec::new();
    for token in &tokens {
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
    return newtokens;
}

fn parse(tokens: Vec<String>) -> Vec<HashMap<String, String>> {
    // abstract syntax table
    let mut ast: Vec<HashMap<String, String>> = Vec::new();
    for token in tokens {
        // check if token is numeric
        if token.parse::<f64>().is_ok() {
            ast.push([("type".to_owned(), 
                       _INT.to_owned()), 
                     ("token".to_string(), 
                      token)]
                     .iter()
                     .cloned()
                     .collect()
                     );
        } else if _OPERATORS.contains(&&*token) {
            ast.push([("type".to_owned(), 
                       _OPT.to_owned()), 
                     ("token".to_owned(), 
                      token)]
                     .iter()
                     .cloned()
                     .collect()
                     );
        } else {
            println!("ERROR: token {} not recognized as OPT or INT! aborting.", token);
            process::exit(1);
        }
    }
    debug(format!("DEBUG: AST: {:?}", ast));
    return ast;
}

fn factorial(x: f64) -> f64 {
    let mut val: f64 = 0.0;
    for y in 1..(x as u8) {
        val = val * y as f64;
    }
    return val;
}

fn process(ast: Vec<HashMap<String, String>>) -> String {
    let mut val:     f64    = 0.0;
    let mut lastopt: String = "".to_owned();
    // let mut lastint: f64    = 0;
    for map in &ast {
        if map["type"] == _INT.to_string() {
            // check if we are already in an expression
            if lastopt == "".to_owned() {
                if val == 0.0 {
                    val = map["token"].parse::<f64>().unwrap();
                } else {
                    warn(format!("WARN!: token {} has no operator!", map["token"]));
                }
            } else {
                // if so, calculate and add to val, then reset other values.
                if lastopt == "+" { val = val + map["token"].parse::<f64>().unwrap(); }
                else if lastopt == "-" { val = val - map["token"].parse::<f64>().unwrap(); }
                else if lastopt == "*" { val = val * map["token"].parse::<f64>().unwrap(); }
                else if lastopt == "/" { val = val / map["token"].parse::<f64>().unwrap(); }
                else if lastopt == "%" { val = val % map["token"].parse::<f64>().unwrap(); }
                else if lastopt == "^" { val = val.powf(map["token"].parse::<f64>().unwrap()); }
                else if lastopt == "!" { val = val + factorial(map["token"].parse::<f64>().unwrap()); }
                else { println!("WARN: operator {} not implemented yet.", map["token"]); }

                // reset values
                lastopt = "".to_owned();
                // lastint = 0;
            }
        }
        if map["type"] == _OPT.to_string() {
            lastopt = map["token"].to_string();
        }
        debug(format!("DEBUG: val:{} opt:{}", val, lastopt));
    }
    return format!("{}", val)
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
    
    let mut nargs: Vec<String> = Vec::new();
    for i in 1..args.len() {
        nargs.push((&*args[i]).to_owned());
    }
   
    // Display help if there aren't any args.
    if nargs.len() < 1 { 
        help();
        std::process::exit(1);
    }

    // print help and exit if first arg is ?
    if nargs[1] == "--help" ||
        nargs[1] == "-h"    ||
        nargs[1] == "-help" ||
        nargs[1] == "help"  ||
        nargs[1] == "?" {
            help();
            std::process::exit(1);
    }

    // print version if first arg is -v
    if nargs[1] == "-v" ||
        nargs[1] == "--version" ||
        nargs[1] == "version" {
            version();
            std::process::exit(1);
    }


    let answ: String = process(parse(lex(nargs)));
    println!("{}", answ);
}
