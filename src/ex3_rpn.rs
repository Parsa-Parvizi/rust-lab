//! تمرین ۳: stack با Vec، match، مدیریت خطا
//! RPN (Reverse Polish Notation):  "3 4 +" = 7   |   "5 1 2 + 4 * + 3 -" = 14

#[derive(Debug, PartialEq)]
pub enum CalcError {
    StackUnderflow,       // عملگر به عملوند کافی نرسید: "1 +"
    DivisionByZero,       // "1 0 /"
    InvalidToken(String), // توکن نامعتبر: "2 x *"
    TooManyValues,        // در پایان بیش از یک مقدار روی stack: "1 2"
    Empty,                // عبارت خالی
}

/// عملگرها: + - * /   (عملوند چپ = مقداری که زودتر push شده)
/// راهنما: split_whitespace, tok.parse::<f64>().map_err(...)
pub fn eval_rpn(expr: &str) -> Result<f64, CalcError> {
    todo!()
}
