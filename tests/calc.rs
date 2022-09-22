#[cfg(test)]
mod parse_calc_tests {
    use formula::*;
    use std::fs;

    #[test]
    fn parse_formulas() {
        let content = fs::read_to_string("tests/data/data.txt").unwrap();

        for formula in content.lines() {
            let result = formula::parse(&formula);
            match result {
                Err(err) => {
                    assert!(false, "{} Failed: {}", formula, err);
                }
                _ => (),
            }
        }
    }

    #[test]
    fn parse_get_function() {
        let formula = "COUNT(relationship;)";
        let result = formula::parse(&formula).unwrap();

        // println!("{:?}", result);
        let function =
            Function::from(result.clone().next().unwrap().into_inner().next().unwrap()).unwrap();
        assert_eq!(function.name, "COUNT");
        assert_eq!(
            function.parts,
            vec![formula::FunctionPart::Identifier(
                "relationship".to_string()
            )]
        );
    }

    #[test]
    fn parse_get_function_with_compare() {
        let formula = "COUNT(relationship;issueTypeId=1848788)";
        let result = formula::parse(&formula).unwrap();

        let function =
            Function::from(result.clone().next().unwrap().into_inner().next().unwrap()).unwrap();
        assert_eq!(function.name, "COUNT");
        assert_eq!(
            function.parts,
            vec![
                FunctionPart::Identifier("relationship".to_string()),
                FunctionPart::CompareExpression(
                    "issueTypeId".to_string(),
                    CompareOperator::Equal,
                    "1848788".to_string()
                )
            ]
        );
    }

    #[test]
    fn parse_get_function_with_compare_2() {
        let formula = "SUM(subtask.estimatePoint;status=4)";
        let result = formula::parse(&formula).unwrap();

        let function =
            Function::from(result.clone().next().unwrap().into_inner().next().unwrap()).unwrap();
        assert_eq!(function.name, "SUM");
        assert_eq!(
            function.parts,
            vec![
                FunctionPart::Identifier("subtask.estimatePoint".to_string()),
                FunctionPart::CompareExpression(
                    "status".to_string(),
                    CompareOperator::Equal,
                    "4".to_string()
                )
            ]
        );
    }

    

    #[test]
    fn parse_get_dependencies_1() {
        let formula = "GET_NOW-GET_UPDATE_TIME";
        let result = formula::parse(&formula).unwrap();

        let dependencies = formula::get_dependencies(result);
        assert_eq!(dependencies, vec!["GET_NOW", "GET_UPDATE_TIME"]);
    }
}

#[cfg(test)]
mod number_calc_tests {
    use formula::{self, ExpValue};
    use std::collections::HashMap;

    fn create_num_table() -> HashMap<String, ExpValue> {
        let mut table = HashMap::new();

        table.insert("a".to_string(), ExpValue::Number(6.0));
        table.insert("b".to_string(), ExpValue::Number(3.0));
        table.insert("c".to_string(), ExpValue::Number(5.0));
        table.insert("d".to_string(), ExpValue::Number(8.0));

        table
    }

    #[test]
    fn calc_add() {
        let exp = formula::parse("a + b").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(9.0));
    }

    #[test]
    fn calc_add_multi() {
        let result = formula::eval(
            formula::parse("a + b + c + d").unwrap(),
            &create_num_table(),
        );

        let result2 = formula::eval(formula::parse("d+c+a+b").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(22.0));
        assert_eq!(result2, ExpValue::Number(22.0));
    }

    #[test]
    fn calc_add_assoc() {
        let result = formula::eval(formula::parse("a + (b + c)").unwrap(), &create_num_table());
        let result2 = formula::eval(formula::parse("(a + b) + c").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(14.0));
        assert_eq!(result2, ExpValue::Number(14.0));
    }

    #[test]
    fn calc_sub() {
        let exp = formula::parse("a - b").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(3.0));
    }

    #[test]
    fn calc_sub_multi() {
        let result = formula::eval(
            formula::parse("a - b - c - d").unwrap(),
            &create_num_table(),
        );
        assert_eq!(result, ExpValue::Number(-10.0));
    }

    #[test]
    fn calc_sub_assoc() {
        let result = formula::eval(formula::parse("a - (b - c)").unwrap(), &create_num_table());
        let result2 = formula::eval(formula::parse("(a - b) - c").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(8.0));
        assert_eq!(result2, ExpValue::Number(-2.0));
    }

    #[test]
    fn calc_mul() {
        let exp = formula::parse("a * b").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(18.0));
    }

    #[test]
    fn calc_mul_multi() {
        let result = formula::eval(
            formula::parse("a * b * c * d").unwrap(),
            &create_num_table(),
        );
        assert_eq!(result, ExpValue::Number(720.0));
    }

    #[test]
    fn calc_mul_assoc() {
        let result = formula::eval(formula::parse("a * (b * c)").unwrap(), &create_num_table());
        let result2 = formula::eval(formula::parse("(a * b) * c").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(90.0));
        assert_eq!(result2, ExpValue::Number(90.0));
    }

    #[test]
    fn calc_div() {
        let exp = formula::parse("a / b").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(2.0));
    }

    #[test]
    fn calc_div_multi() {
        let result = formula::eval(
            formula::parse("a / b / c / d").unwrap(),
            &create_num_table(),
        );
        assert_eq!(result, ExpValue::Number(0.05));
    }

    #[test]
    fn calc_div_assoc() {
        let result = formula::eval(formula::parse("d / (a / b)").unwrap(), &create_num_table());
        let result2 = formula::eval(formula::parse("(a / b) / c").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(4.0));
        assert_eq!(result2, ExpValue::Number(0.4));
    }

    #[test]
    fn calc_pow() {
        let exp = formula::parse("a ^ b").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(216.0));
    }

    #[test]
    fn calc_pow_multi() {
        let result = formula::eval(formula::parse("4 ^ 3 ^ 2").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(262144.0));
    }

    #[test]
    fn calc_pow_assoc() {
        let result = formula::eval(formula::parse("4 ^ 3 ^ 2").unwrap(), &create_num_table());
        let result2 = formula::eval(formula::parse("(4 ^ 3) ^ 2").unwrap(), &create_num_table());
        assert_eq!(result, ExpValue::Number(262144.0));
        assert_eq!(result2, ExpValue::Number(4096.0));
    }

    #[test]
    fn calc_rem() {
        let exp = formula::parse("a % b").unwrap();
        let result = formula::eval(exp, &create_num_table());
        assert_eq!(result, ExpValue::Number(0.0));
    }
}
