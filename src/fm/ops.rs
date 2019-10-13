#[derive(Debug, PartialEq)]
pub enum Operators {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    Factorial,
    NRoot,
    Logarithm,
    Unimplemented,
}

impl Operators {
    pub fn from_token<S>(token: S) -> Self 
    where S: AsRef<str> 
    {
        match token.as_ref() {
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            "%" => Self::Modulo,
            "^" => Self::Exponent,
            "!" => Self::Factorial,
            "nrt" => Self::NRoot,
            "log" => Self::Logarithm,
            _ => Self::Unimplemented,
        }
    }

    
}

impl<S> From<S> for Operators where S: AsRef<str> {
    fn from(token: S) -> Self {
        Self::from_token(token)
    }
}