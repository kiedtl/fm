mod ops;

use ops::Operators;

use std::env;
use std::process;
use std::collections::HashMap;

// token types
const INT: &str = "integer";
const OPT: &str = "operator";
const SUB: &str = "subexpression";

pub fn calculate(args: &[String]) -> String {
    process(&parse(&lex(args)))
}

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
    let mut ctr = 0;
    for _ in 0..tokens.len() {
        let mut token: &str = &tokens[ctr];
        // check if token is numeric
        if token.parse::<f64>().is_ok() {
            ast.push([("type".to_owned(), 
                       INT.to_owned()), 
                     ("token".to_string(), 
                      token.to_owned())]
                     .iter()
                     .cloned()
                     .collect()
                     );
        } else if token == SUB {
            let expr_start = ctr;
            let mut expr_depth = 1;
            while token != ")" && expr_depth == 0 {
                ctr = ctr + 1;
                token = &tokens[ctr];
                if token == "(" {
                    expr_depth += 1;
                } 
                if token == ")" {
                    expr_depth -= 1;
                }
            }
            // send the expression slice, without 
            // parenthesis, to calculate() then get the return value
            // and insert it into the ast.
            let answer = calculate(&tokens[(expr_start+1 as usize)..(ctr-1 as usize)]);
            ast.push([("type".to_owned(),
                      INT.to_owned()),
                     ("token".to_string(),
                      answer.to_owned())]
                     .iter()
                     .cloned()
                     .collect()
                     );
        } else {
            let operator: Operators = token.into();
            if operator == Operators::Unimplemented {
                eprintln!("ERROR: token {} not recognized as OPT or INT! aborting.", &token);
                process::exit(1);
            }

            ast.push([("type".to_owned(), 
                       OPT.to_owned()), 
                     ("token".to_owned(), 
                      token.to_owned())]
                     .iter()
                     .cloned()
                     .collect()
                     );
        }

        ctr += 1;
    }
    debug(format!("DEBUG: AST: {:?}", ast));
    ast
}

fn process(ast: &[HashMap<String, String>]) -> String {
    let mut val:     f64    = 0.0;
    let mut lastopt: Option<Operators> = None;
    // let mut lastint: f64    = 0;
    for map in ast {
        if map["type"] == INT.to_string() {
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
                    Operators::Add => val += map["token"].parse::<f64>().unwrap(),
                    Operators::Subtract => val -=  map["token"].parse::<f64>().unwrap(),
                    Operators::Multiply => val *=  map["token"].parse::<f64>().unwrap(),
                    Operators::Divide => val /= map["token"].parse::<f64>().unwrap(),
                    Operators::Modulo => val %= map["token"].parse::<f64>().unwrap(),
                    Operators::Exponent => val = val.powf(map["token"].parse::<f64>().unwrap()),
                    Operators::Factorial => val = factorial(val),
                    Operators::Logarithm => val = val.log(map["token"].parse::<f64>().unwrap()),
                    Operators::NRoot => val = root(val, map["token"].parse::<f64>().unwrap()),
                    _ => eprintln!("ERROR: operator {} not implemented yet.", map["token"]),
                }

                // reset values
                lastopt = None;
                // lastint = 0;
            }
        }

        if map["type"] == OPT.to_string() {
            lastopt = Some((&map["token"]).into());
        }

        debug(format!("DEBUG: val:{} opt:{:?}", val, lastopt));
    }

    val.to_string()
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

fn debug(text: String) {
    if env::var("FD_DEBUG").is_ok() {
        println!("{}", text);
    }
}

fn warn(text: String) {
    if env::var("FD_WARN").is_ok() {
        eprintln!("{}", text);
    }
}
