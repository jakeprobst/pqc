




use codeblock::CodeSyntaxError;
use operand::{Operand, operand};
use parser::PExpr;


#[derive(Debug)]
pub enum Conditional {
    Eq(Operand, Operand),
}


fn conditional_equal(args: &Vec<PExpr>) -> Result<Conditional, CodeSyntaxError> {
    code_expect_len!(args, 2);
    let op1 = operand(&args[0])?;
    let op2 = operand(&args[1])?;

    Ok(Conditional::Eq(op1, op2))
}

pub fn conditional(arg: &PExpr) -> Result<Conditional, CodeSyntaxError> {
    match arg {
        PExpr::Equal(args) => conditional_equal(args),

        _ => Err(CodeSyntaxError::ExpectedConditional(arg.to_string()))
    }
}
