#[macro_use]
extern crate lalrpop_util;

use wasm_bindgen::prelude::*;

mod ast;
lalrpop_mod!(#[allow(clippy::all)] pub expression); // synthesized by LALRPOP

#[wasm_bindgen]
pub fn evaluate(expression: &str) -> i32 {
    expression::ExprParser::new()
        .parse(expression)
        .unwrap()
        .eval()
}

#[cfg(test)]
mod tests {
    use crate::evaluate;
    use rstest::rstest;

    #[rstest(
    exp,
    expected,
    case("1 + 2", 3),
    case("2 * 2", 4),
    case("10 / 4", 2),
    case("3 - 9", -6),
    case("-5 * -7", 35),
    case("100 * 0", 0),
    case("8 * 2 + 3", 19),
    case("6 * (2 + 3)", 30),
    case("-100 - (-1000)", 900),
    case("150 / (20 - 5) * 3", 30),
    case("150 / ((20 - 5) * 5)", 2),
    case("10 * 15 / ((20 - 5) * 5)", 2),
    case("(100 + 50) / ((20 - 5) * 5)", 2),
    case("100 + 150 / ((20 - 5) * 5)", 102),
    )]
    fn when_expression_evaluated_then_correct_value_returned(exp: &str, expected: i32) {
        assert_eq!(evaluate(exp), expected);
    }
}
