use pest::iterators::Pair;

use crate::calc::calc::Rule;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parts: Vec<FunctionPart>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompareOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionPart {
    Identifier(String),
    CompareExpression(String, CompareOperator, String),
}

impl FunctionPart {
    pub fn from(expression: Pair<Rule>) -> Vec<FunctionPart> {
        if let Rule::function_parameter = expression.as_rule() {
            return expression
                .into_inner()
                .map(FunctionPart::from_single)
                .filter(|p| p.is_some())
                .map(|p| p.unwrap())
                .collect::<Vec<FunctionPart>>();
        }
        vec![]
    }

    fn from_single(expression: Pair<Rule>) -> Option<FunctionPart> {
        if let Rule::function_parameter_item = expression.as_rule() {
            let inner = expression.into_inner().collect::<Vec<Pair<Rule>>>();
            if inner.len() == 1 {
                return Some(FunctionPart::Identifier(inner[0].as_str().to_string()));
            } else if inner.len() == 2 {
                let mut compare_expression = inner[1].clone().into_inner();

                let compare_operator = match compare_expression.next().unwrap().as_rule() {
                    Rule::compare_eq => CompareOperator::Equal,
                    _ => return None,
                };

                return Some(FunctionPart::CompareExpression(
                    inner[0].as_str().to_string(),
                    compare_operator,
                    compare_expression.next().unwrap().as_str().to_string(),
                ));
            }

            for pair in inner {
                match pair.as_rule() {
                    Rule::function_parameter_ident => {
                        return Some(FunctionPart::Identifier(pair.as_str().to_string()));
                    }
                    Rule::function_parameter_compare => {
                        let mut compare_expression = pair.into_inner();
                        let left = compare_expression.next().unwrap().as_str().to_string();
                        let operator = match compare_expression.next().unwrap().as_rule() {
                            Rule::compare_eq => CompareOperator::Equal,
                            // Rule::function_parameter_compare_not_equal => CompareOperator::NotEqual,
                            // Rule::function_parameter_compare_greater_than => {
                            //     CompareOperator::GreaterThan
                            // }
                            // Rule::function_parameter_compare_greater_than_or_equal => {
                            //     CompareOperator::GreaterThanOrEqual
                            // }
                            // Rule::function_parameter_compare_less_than => CompareOperator::LessThan,
                            // Rule::function_parameter_compare_less_than_or_equal => {
                            //     CompareOperator::LessThanOrEqual
                            // }
                            _ => return None,
                        };
                        let right = compare_expression.next().unwrap().as_str().to_string();
                        return Some(FunctionPart::CompareExpression(left, operator, right));
                    }
                    _ => return None,
                }
            }
        }
        None
    }
}

impl Function {
    pub fn from(expression: Pair<Rule>) -> Option<Function> {
        if let Rule::function = expression.as_rule() {
            let mut fun_name: Option<String> = None;
            let mut parts: Option<Vec<FunctionPart>> = None;

            for pair in expression.into_inner() {
                match pair.as_rule() {
                    Rule::function_name => {
                        fun_name = Some(pair.as_str().to_string());
                    }
                    Rule::function_parameter => {
                        parts = Some(FunctionPart::from(pair));
                    }
                    _ => continue,
                }
            }
            if let Some(name) = fun_name {
                return Some(Function {
                    name,
                    parts: parts.unwrap_or(vec![]),
                });
            }
            None
        } else {
            None
        }
    }
}
