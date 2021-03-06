use std::fmt;
use self::SExpr::*;

#[derive(Debug, Clone)]
pub enum SExpr {
    Str(String),
    Num(f64),
    Bool(bool),
    Ident(String),
    List(Vec<SExpr>),
    Nil
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // "string"
            Str(ref s) => write!(f, "\"{}\"", s),

            // num
            Num(n) => write!(f, "{}", n),

            // #t | #f
            Bool(b) => if b {
                write!(f, "true")
            } else {
                write!(f, "false")
            }

            // ident
            Ident(ref s) => write!(f, "{}", s),

            // (a b c ...)
            List(ref exps) => {
                write!(f, "(")?;
                let len = exps.len();
                if len > 0 {
                    for i in 0..len - 1 {
                        write!(f, "{} ", &exps[i])?;
                    }
                    write!(f, "{}", &exps[len - 1])?;
                }
                write!(f, ")")
            },

            // ()
            Nil => write!(f, "()")
        }
    }
}
