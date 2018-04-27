




use codeblock::CodeSyntaxError;
use parser::PExpr;

#[derive(Debug)]
 pub enum Operand {
    Builtin(String),
    Constant(String),
    Identifier(String),
    Number(f32),
}

pub fn operand(arg: &PExpr) -> Result<Operand, CodeSyntaxError> {
    match arg {
        PExpr::Identifier(ident) => {
            if ident.starts_with("$") {
                Ok(Operand::Builtin(ident.clone()))
            }
            else if ident.starts_with("@") {
                Ok(Operand::Constant(ident.clone()))
            }
            else {
                Ok(Operand::Identifier(ident.clone()))
            }
        },
        PExpr::Number(num) => Ok(Operand::Number(*num)),
        _ => Err(CodeSyntaxError::InvalidOperand(arg.to_string()))
    }
}
