extern crate pest;

use pest::{
    iterators::Pairs,
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser, error::Error
};

#[derive(Parser)]
#[grammar = "calc/calc.pest"]
pub struct Calculator;

use Assoc::*;
use Rule::*;

pub fn parse(input: &str) -> Result<Pairs<Rule>,Error<Rule>> {
    Calculator::parse(Rule::calculation, input)
}

pub fn eval(expression: Pairs<Rule>) -> f64 {
    let operators = PrecClimber::new(vec![
        Operator::new(add, Left) | Operator::new(subtract, Left),
        Operator::new(multiply, Left) | Operator::new(divide, Left),
        Operator::new(modulus, Left),
        Operator::new(power, Right),
    ]);

    operators.climb(
        expression,
        |pair| match pair.as_rule() {
            Rule::num => pair.as_str().trim().parse::<f64>().unwrap(),
            Rule::expr => eval(pair.into_inner()),
            Rule::ident => {
                let id = pair.as_str().trim();
                println!("{}", id);
                let value = match id {
                    "pi" => std::f64::consts::PI,
                    "e" => std::f64::consts::E,
                    _ => 0.0,
                };
                value
            }
            _ => f64::NAN,
        },
        |lhs, op, rhs| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.powf(rhs),
            Rule::modulus => (lhs % rhs) as f64,

            _ => f64::NAN,
        },
    )
}