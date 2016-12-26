
//use pqc::types::PExpr;
//use pqc::types::Quest;
use types::*;

//mod parser;

pub fn tokenize_str(expr: &str) -> Vec<String> {
    let mut result = Vec::new();

    let mut current = String::new();
    for c in expr.chars() {
        if c == '(' || c == ')' {
            if current.len() > 0 {
                result.push(current);
                current = String::new();
                //current.clear();
            }
            result.push(c.to_string());
        }
        else if c.is_whitespace() {
            if current.len() > 0 {
                result.push(current);
                current = String::new();
                //current.clear();
            }
        }
        else {
            current.push(c);
        }
    }

    return result;
}

#[derive(Debug)]
pub enum SyntaxError {
    ExpectedOpenParen,
    UnbalancedParens,
}

pub enum ASTType {
    Identifier(String),
    Expression(Vec<String>)
}

// TODO: figure out a proper name for this function
// TODO: handle quotes
fn split_into_ast_blocks(tokens: &Vec<String>) -> Result<Vec<Vec<String>>, SyntaxError> {
    print!("split: {:?}", tokens);
    let mut result = Vec::new();

    let mut parencount = 0;
    let mut current = Vec::new();
    for s in tokens.iter() {
        if s == "(" {
            parencount += 1;
        }
        else if s == ")" {
            parencount -= 1;
        }

        current.push(s.clone());

        if parencount == 0 {
            result.push(current);
            current = Vec::new();
        }
    }

    print!(" [{}] ", parencount);
    
    if parencount != 0 {
        return Err(SyntaxError::UnbalancedParens);
    }
    
    println!(" -> {:?}", result);

    return Ok(result);
}

#[derive(Debug)]
pub enum ASTree {
    Node(String, Vec<ASTree>),
    Value(String),
    //None
}

fn generate_ast(tokens: &Vec<String>) -> Result<ASTree, SyntaxError> {
    let ref function = tokens[1];
    let childblocks = try!(split_into_ast_blocks(&tokens[2..tokens.len()-1].to_vec()));
    let mut childast = Vec::new();
    
    for cb in childblocks.iter() {
        if cb[0] == "(" {
            match generate_ast(cb) {
                Ok(ast) => childast.push(ast),
                Err(SyntaxError::ExpectedOpenParen) => {},
                Err(e) => return Err(e)
            }
        }
        else {
            for v in cb.iter() {
                childast.push(ASTree::Value(v.clone()));
            }
        }
    }
    
    return Ok(ASTree::Node(function.clone(), childast));
}


#[derive(Debug)]
pub enum EvalError {
    Bleh,
    ExpectedOpenParen,
    UnexpectedFunction(String),
}

fn eval_tokens(ast: &ASTree)-> Result<PExpr, EvalError> {


    return Err(EvalError::Bleh);
    /*if tokens[0] != "(" {
        return Err(EvalError::ExpectedOpenParen);
    }

    match tokens[1] {
        "if " => {
        },
        "npc" => {
        },
        _ = > {
            return EvalError::UnexpectedFunction(tokens[1]);
        }
    }
    */
    

    
}

#[derive(Debug)]
pub enum ScriptError {
    SyntaxError(SyntaxError),
    EvalError(EvalError),
}

impl From<SyntaxError> for ScriptError {
    fn from(err: SyntaxError) -> ScriptError {
        ScriptError::SyntaxError(err)
    }
}

impl From<EvalError> for ScriptError {
    fn from(err: EvalError) -> ScriptError {
        ScriptError::EvalError(err)
    }
}

pub fn eval_tokenized_expr(tokens: Vec<String>) -> Result<Vec<PExpr>, ScriptError> {
    let mut result = Vec::new();

    let astblocks = try!(split_into_ast_blocks(&tokens));
    
    for ast in astblocks {
        let astree = try!(generate_ast(&ast));
        println!("astree: {:?}", astree);
        result.push(try!(eval_tokens(&astree)));
    }

    
    
    //result.push(PExpr::Equal(PExpr::Register(Variable {name: String::from("a"), register: PExpr::Register(Variable())}), PExpr::Value(19)));
    result.push(PExpr::Equal(Box::new(PExpr::Integer(5)), Box::new(PExpr::Integer(29))));
    //result.push(PExpr::Equal(PExpr::Integer(5), PExpr::Integer(29)));
    

    return Ok(result);
}


/*




fn parse_script(script: String) -> Quest {

    
}
*/
