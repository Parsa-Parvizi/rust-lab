use rust_lab::ex3_rpn::*;

#[test]
fn simple_ops() {
    assert_eq!(eval_rpn("3 4 +"), Ok(7.0));
    assert_eq!(eval_rpn("10 4 -"), Ok(6.0)); // ترتیب عملوندها مهم است
    assert_eq!(eval_rpn("6 4 /"), Ok(1.5));
    assert_eq!(eval_rpn("2.5 2 *"), Ok(5.0));
}

#[test]
fn complex_expression() {
    assert_eq!(eval_rpn("5 1 2 + 4 * + 3 -"), Ok(14.0));
    assert_eq!(eval_rpn("  7  "), Ok(7.0));
}

#[test]
fn errors() {
    assert_eq!(eval_rpn("1 0 /"), Err(CalcError::DivisionByZero));
    assert_eq!(eval_rpn("1 +"), Err(CalcError::StackUnderflow));
    assert_eq!(eval_rpn("+"), Err(CalcError::StackUnderflow));
    assert_eq!(eval_rpn("2 x *"), Err(CalcError::InvalidToken("x".into())));
    assert_eq!(eval_rpn("1 2"), Err(CalcError::TooManyValues));
    assert_eq!(eval_rpn(""), Err(CalcError::Empty));
    assert_eq!(eval_rpn("   "), Err(CalcError::Empty));
}
