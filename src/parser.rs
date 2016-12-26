
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
    InvalidArgumentCount(String, usize, usize),
    InvalidArgument(String, String),
}

// TODO: better name for this
macro_rules! allowed_types {
    ($label:expr, $ast:expr, [ $( $t:path ),* ]) => {
        {
            let evalast = try!(eval_ast($ast));
            match evalast {
                $(
                    $t(..) => 
                        evalast,
                    
                )*
                _ => {
                    println!("bad! {:?}", evalast);
                    return Err(EvalError::InvalidArgument(String::from($label),
                                                   if let &ASTree::Node(ref fname, _) = $ast {
                                                       fname.clone()
                                                   } 
                                                   else {
                                                       String::new()
                                                   }));
                }
            }
        }
    }
}

fn eval_ast(ast: &ASTree)-> Result<PExpr, EvalError> {
    match ast {
        &ASTree::Node(ref function, ref args) => {
            match &function as &str {
                "if" => {
                    return eval_if(&args);
                }
                "equal" => {
                    return eval_equal(&args);
                }
                "set" => {
                    return eval_set(&args);
                }
                "+" => {
                    return eval_plus(&args);
                }
                _ => {
                    /*if let Ok(i) = function.parse::<u32>() {
                        return Ok(PExpr::Integer(i));
                    }
                    return Ok(PExpr::Variable(function.clone()));*/
                    unreachable!();

                }
            }
        }
        &ASTree::Value(ref val) => {
            if let Ok(i) = val.parse::<u32>() {
                return Ok(PExpr::Integer(i));
            }
            return Ok(PExpr::Variable(val.clone()));
        }
    }
    //return 
}

fn eval_if(args: &Vec<ASTree>) -> Result<PExpr, EvalError> {
    if args.len() != 3 {
        return Err(EvalError::InvalidArgumentCount(String::from("if"), 3, args.len()));
    }
    println!("if: {:?}", args[0]);
    //println!("if: {:?}", try!(eval_ast(&args[0])));

    println!("y");
    //let cond: Result<PExpr, EvalError>   = allowed_types!("if", &args[0], [PExpr::Equal]);
    let cond = allowed_types!("if", &args[0], [PExpr::Equal]);
    //let cond2 = try!(cond);
    println!("z");
    let btrue  = allowed_types!("if", &args[1], [PExpr::If, PExpr::Block, PExpr::Assign]);
    let bfalse = allowed_types!("if", &args[2], [PExpr::If, PExpr::Block, PExpr::Assign]);

    //return Err(EvalError::Bleh);
    return Ok(PExpr::If(Box::new(cond), Box::new(btrue), Box::new(bfalse)));
}

fn eval_equal(args: &Vec<ASTree>) -> Result<PExpr, EvalError> {
    println!("equal: {:?}", args);
    if args.len() != 2 {
        return Err(EvalError::InvalidArgumentCount(String::from("equal"), 2, args.len()));
    }
    println!("a: {:?}", try!(eval_ast(&args[0])));
    println!("b: {:?}", try!(eval_ast(&args[1])));

    let left  = allowed_types!("equal", &args[0], [PExpr::Integer, PExpr::Variable]);
    //let a = allowed_types!("equal", &args[0], [PExpr::Integer, PExpr::Variable]);
    println!("b");
    let right = allowed_types!("equal", &args[1], [PExpr::Integer, PExpr::Variable]);
    println!("c");

    println!("l: {:?}", left);
    println!("r: {:?}", right);

    return Ok(PExpr::Equal(Box::new(left), Box::new(right)));
}

fn eval_set(args: &Vec<ASTree>) -> Result<PExpr, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::InvalidArgumentCount(String::from("set"), 2, args.len()));
    }

    let var = allowed_types!("set", &args[0], [PExpr::Variable]);
    let val = allowed_types!("set", &args[1], [PExpr::Integer, PExpr::Variable, PExpr::Plus]);

    return Ok(PExpr::Assign(Box::new(var), Box::new(val)));
    //return Err(EvalError::Bleh);
}

fn eval_plus(args: &Vec<ASTree>) -> Result<PExpr, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::InvalidArgumentCount(String::from("+"), 2, args.len()));
    }

    let left  = allowed_types!("+", &args[0], [PExpr::Integer, PExpr::Variable]);
    let right = allowed_types!("+", &args[1], [PExpr::Integer, PExpr::Variable]);

    return Ok(PExpr::Plus(Box::new(left), Box::new(right)));
    //return Err(EvalError::Bleh);
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
        result.push(try!(eval_ast(&astree)));
    }

    
    
    //result.push(PExpr::Equal(PExpr::Register(Variable {name: String::from("a"), register: PExpr::Register(Variable())}), PExpr::Value(19)));
    //result.push(PExpr::Equal(Box::new(PExpr::Integer(5)), Box::new(PExpr::Integer(29))));
    //result.push(PExpr::Equal(PExpr::Integer(5), PExpr::Integer(29)));
    

    return Ok(result);
}


/*




fn parse_script(script: String) -> Quest {

    
}
*/
