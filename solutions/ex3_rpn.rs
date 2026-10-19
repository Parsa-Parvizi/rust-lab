pub fn eval_rpn(expr: &str) -> Result<f64, CalcError> {
    let mut stack: Vec<f64> = Vec::new();
    for tok in expr.split_whitespace() {
        match tok {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().ok_or(CalcError::StackUnderflow)?;
                let a = stack.pop().ok_or(CalcError::StackUnderflow)?;
                let r = match tok {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    _ => {
                        if b == 0.0 {
                            return Err(CalcError::DivisionByZero);
                        }
                        a / b
                    }
                };
                stack.push(r);
            }
            _ => {
                let n = tok
                    .parse::<f64>()
                    .map_err(|_| CalcError::InvalidToken(tok.to_string()))?;
                stack.push(n);
            }
        }
    }
    match stack.len() {
        0 => Err(CalcError::Empty),
        1 => Ok(stack[0]),
        _ => Err(CalcError::TooManyValues),
    }
}
