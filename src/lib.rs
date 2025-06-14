mod utils;

extern crate core;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Self {
        Self(s.parse::<i32>().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum Op{
    Add,
    Sub,
    Mul,
    Div
}

impl Op{
    pub fn new(s: &str) -> Op{
        match s { 
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!("Unknown op: {}", s),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr{
    pub lhs: Number,
    pub rhs: Number,
    pub op: Op,
}
impl Expr {
    pub fn new(s: &str) -> Self {
        let lhs = Number::new(s);
        let rhs = Number::new(s);
        let op = Op::new(s);
        
        Self { lhs, rhs, op }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn parse_one_plus_two(){
        assert_eq!(
            Expr::new("1+2"),
            Expr {
                lhs: Number(1),
                rhs: Number(2),
                op: Op::Add,
            }
        )
    }
    
    // test 2
    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Op::Add);
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Op::Sub);
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), Op::Mul);
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), Op::Div);
    } 
    
    // first thing tested
    #[test]
    fn parse_number(){
        assert_eq!(Number::new("123"), Number(123))
    }
}
