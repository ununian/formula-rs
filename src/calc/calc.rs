extern crate pest;

use std::collections::HashMap;

use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    prec_climber::{Assoc, Operator, PrecClimber},
    Parser,
};

#[derive(Parser)]
#[grammar = "calc/calc.pest"]
pub struct Calculator;

use Assoc::*;
use Rule::*;

use crate::ExpValue;

pub fn parse(input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    Calculator::parse(Rule::calculation, input)
}

#[derive(Debug, Clone)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn get_value(self, table: &HashMap<String, ExpValue>) -> Option<ExpValue> {
        table.get(&self.name).cloned()
    }
}

fn get_root(expression: Pairs<Rule>) -> Option<Pair<Rule>> {
    expression.clone().next()
}

pub fn get_dependencies(expression: Pairs<Rule>) -> Vec<String> {
    let mut dependencies = Vec::new();
    let root = get_root(expression);

    match root {
        Some(r) => {
            for pair in r.into_inner() {
                if let Rule::ident | Rule::function_parameter_ident = pair.as_rule() {
                    let dependency = pair.as_str().to_string();
                    if !dependencies.contains(&dependency) {
                        dependencies.push(pair.as_str().to_string());
                    }
                }
            }
            dependencies
        }
        None => [].to_vec(),
    }
}

pub fn eval(expression: Pairs<Rule>, table: &HashMap<String, ExpValue>) -> ExpValue {
    let operators = PrecClimber::new(vec![
        Operator::new(add, Left) | Operator::new(subtract, Left),
        Operator::new(multiply, Left) | Operator::new(divide, Left),
        Operator::new(modulus, Left),
        Operator::new(power, Right),
    ]);

    operators.climb(
        expression,
        |pair| match pair.as_rule() {
            Rule::num => ExpValue::Number(pair.as_str().trim().parse::<f64>().unwrap()),
            Rule::expr => eval(pair.into_inner(), &table),
            Rule::ident => {
                let name = pair.as_str().trim();
                let id = Identifier {
                    name: name.to_string(),
                };

                id.get_value(&table).unwrap_or(ExpValue::Error)
            }
            _ => ExpValue::Number(f64::NAN),
        },
        |lhs, op, rhs| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::subtract => lhs - rhs,
            Rule::multiply => lhs * rhs,
            Rule::divide => lhs / rhs,
            Rule::power => lhs.powf(rhs),
            Rule::modulus => (lhs % rhs),

            _ => ExpValue::Error,
        },
    )
}
