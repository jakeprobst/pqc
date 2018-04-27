use parser::PExpr;
use conditional::{Conditional, conditional};

#[derive(Debug)]
pub enum CodeSyntaxError {
    InvalidFunction(String),
    InvalidNumberOfArguments(String, usize, usize),
    ExpectedConditional(String),
    InvalidOperand(String),
}

#[derive(Debug)]
pub enum CodeExpr {
    Noop,
    Block {
        code: Vec<CodeExpr>,
    },
    
    If {
        cond: Conditional,
        then: Box<CodeExpr>,
        else_:  Box<CodeExpr>,
    },
    
}


macro_rules! code_expect_len {
    ($arg:expr, $len:expr) => {
        if $arg.len() != $len {
            return Err(CodeSyntaxError::InvalidNumberOfArguments(format!("{:?}", $arg), $len, $arg.len()));
        }
    }
}

fn block(args: &Vec<PExpr>) -> Result<CodeExpr, CodeSyntaxError> {
    let mut code = Vec::new();

    for arg in args {
        code.push(code_expr(arg)?);
    }
    
    Ok(CodeExpr::Block{
        code: code,
    })
}



fn code_if(args: &Vec<PExpr>) -> Result<CodeExpr, CodeSyntaxError> {
    code_expect_len!(args, 2);

    let condition = conditional(&args[0])?;
    let iftrue = code_expr(&args[1])?;
    
    let ifelse = if args.len() == 3 {
        code_expr(&args[2])?
    }
    else {
        CodeExpr::Noop
    };
    

    Ok(CodeExpr::If {
        cond: condition,
        then: Box::new(iftrue),
        else_: Box::new(ifelse),
    })
}

pub fn code_expr(expr: &PExpr) -> Result<CodeExpr, CodeSyntaxError> {
    match expr {
        PExpr::Block(args) => {
            block(&args)
        },
        PExpr::If(args) => {
            code_if(&args)
        }
        

        //_ => Err(CodeSyntaxError::InvalidFunction(format!("{:?}", expr)))
        _ => Ok(CodeExpr::Noop)
        
    }
}





/*struct CodeBlock {
    
}
*/

