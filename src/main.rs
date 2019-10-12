use std;
use std::env;
use std::process;
use std::collections::HashMap;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut nargs: Vec<String> = Vec::new();
    for i in 1..args.len() {
        nargs.push((&*args[i]).to_owned());
    }
    let answ: String = process(parse(lex(nargs)));
    println!("{}", answ);
}
