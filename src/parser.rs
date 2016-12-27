
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

// TODO: figure out a proper name for this function
// TODO: handle quotes
fn split_into_ast_blocks(tokens: &Vec<String>) -> Result<Vec<Vec<String>>, SyntaxError> {
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

    if parencount != 0 {
        return Err(SyntaxError::UnbalancedParens);
    }
    
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
    UnknownFunction(String),
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

/*fn eval_ast(ast: &ASTree)-> Result<PExpr, EvalError> {
    match ast {
        &ASTree::Node(ref function, ref args) => {
            match &function as &str {
                "if" => eval_if(&args),
                "equal" => eval_equal(&args),
                "set" => eval_set(&args),
                "+" => eval_plus(&args),

                "set-episode" => eval_set_episode(&args),
                
                _ => Err(EvalError::UnknownFunction(function.clone())),
            }
        }
        &ASTree::Value(ref val) => {
            if let Ok(i) = val.parse::<u32>() {
                Ok(PExpr::Integer(i));
            }
            else {
                Ok(PExpr::Variable(val.clone()));
            }
        }
    }
}*/



/*fn eval_if(args: &Vec<ASTree>) -> Result<PExpr, EvalError> {
    if args.len() != 3 {
        return Err(EvalError::InvalidArgumentCount(String::from("if"), 3, args.len()));
    }

    let cond = allowed_types!("if", &args[0], [PExpr::Equal]);
    let btrue  = allowed_types!("if", &args[1], [PExpr::If, PExpr::Block, PExpr::Assign]);
    let bfalse = allowed_types!("if", &args[2], [PExpr::If, PExpr::Block, PExpr::Assign]);

    return Ok(PExpr::If(Box::new(cond), Box::new(btrue), Box::new(bfalse)));
}*/
macro_rules! eval_statement {
    ($name:tt, $typ:path, $args:expr, $len:expr, { $($var:ident: [$($t:path),*] ),* }) => {
        $name => {
            if $args.len() != $len {
                Err(EvalError::InvalidArgumentCount(String::from($name), $len, $args.len()))
            }
            else {
                let mut count = 0;
                $(
                    let $var = allowed_types!($name, &$args[count], [ $( $t ),*]);
                    count += 1;
                )*;
                Ok($typ($(Box::new($var)),*))
            }
        }
    }
}

macro_rules! eval_statement_match {
    ($func:expr, $args:expr, {
        $(
            [$name:expr, $typ:path, $len:expr,
             {
                 $(
                     $var:ident:
                     [
                         $(
                             $t:path
                         ),*
                     ]
                 ),*
             }
            ]
        ),*
    }) => {
        {
            match $func {
                $(
                    $name => {
                        if $args.len() != $len {
                            Err(EvalError::InvalidArgumentCount(String::from($name), $len, $args.len()))
                        }
                        else {
                            let mut count = 0;
                            $(
                                let $var = allowed_types!($name, &$args[count], [ $( $t ),*]);
                                count += 1;
                            )*;
                            Ok($typ($(Box::new($var)),*))
                        }
                    }
                )*,
                _ => Err(EvalError::UnknownFunction(String::from($func)))
            }
        }
        
    }
}


fn eval_ast(ast: &ASTree)-> Result<PExpr, EvalError> {
    match ast {
        &ASTree::Node(ref function, ref args) => {
            /*eval_statement_match(function as &str, &args, {
                ["if", PExpr::If, 3,
                 {cond: [PExpr::Equal],
                  btrue: [PExpr::If, PExpr::Block, PExpr::Assign],
                  bfalse: [PExpr::If, PExpr::Block, PExpr::Assign]}],
                ["equal", PExpr::Equal, 2,
                 {left: [PExpr::Integer, PExpr::Variable],
                  right: [PExpr::Integer, PExpr::Variable]}],
        });*/
            eval_statement_match!(function as &str, &args, {
                ["if", PExpr::If, 3, {
                    cond: [PExpr::Equal],
                    btrue: [PExpr::If, PExpr::Block, PExpr::Set],
                    bfalse: [PExpr::If, PExpr::Block, PExpr::Set]}],
                ["equal", PExpr::Equal, 2, {
                    left: [PExpr::Integer, PExpr::Variable],
                    right: [PExpr::Integer, PExpr::Variable]
                }],
                ["set", PExpr::Set, 2, {
                    var: [PExpr::Variable],
                    val: [PExpr::Integer, PExpr::Variable, PExpr::Plus]
                }],
                ["+", PExpr::Plus, 2, {
                    left: [PExpr::Integer, PExpr::Variable],
                    right: [PExpr::Integer, PExpr::Variable]
                }]
            })
            /*eval_statements!(&function, &args) {
                
            }*/
            //match &function as &str {
                /*eval_statement!("if", PExpr::If, &args, 3,
                                {cond: [PExpr::Equal],
                                 btrue: [PExpr::If, PExpr::Block, PExpr::Assign],
                                 bfalse: [PExpr::If, PExpr::Block, PExpr::Assign]})*/
                /*eval_statement!("equal", ast, 2, {left: [PExpr::Integer, PExpr::Variable],
                                                  right: [PExpr::Integer, PExpr::Variable]}),
                eval_statement!("set", ast, 3, {var: [PExpr::Integer, PExpr::Variable],
                                                val: [PExpr::Integer, PExpr::Variable, PExpr::Plus]}),
                eval_statement!("plus", ast, 2, {left: [PExpr::Integer, PExpr::Variable],
                                                 right: [PExpr::Integer, PExpr::Variable]}),*/
                //_ => Err(EvalError::UnknownFunction(function.clone())),
            //}
        }
        &ASTree::Value(ref val) => {
            if let Ok(i) = val.parse::<u32>() {
                Ok(PExpr::Integer(i))
            }
            else {
                Ok(PExpr::Variable(val.clone()))
            }
        }
    }
}

/*eval_statement!(eval_if, "if", 3, {cond: [PExpr::Equal],
                                   btrue: [PExpr::If, PExpr::Block, PExpr::Assign],
                                   bfalse: [PExpr::If, PExpr::Block, PExpr::Assign]});
*/

/*fn eval_if(args: &Vec<ASTree>) -> Result<PExpr, EvalError> {
    if args.len() != 3 {
        return Err(EvalError::InvalidArgumentCount(String::from("if"), 3, args.len()));
    }

    let cond = allowed_types!("if", &args[0], [PExpr::Equal]);
    let btrue  = allowed_types!("if", &args[1], [PExpr::If, PExpr::Block, PExpr::Assign]);
    let bfalse = allowed_types!("if", &args[2], [PExpr::If, PExpr::Block, PExpr::Assign]);

    return Ok(PExpr::If(Box::new(cond), Box::new(btrue), Box::new(bfalse)));
}

fn eval_equal(args: &Vec<ASTree>) -> Result<PExpr, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::InvalidArgumentCount(String::from("equal"), 2, args.len()));
    }

    let left  = allowed_types!("equal", &args[0], [PExpr::Integer, PExpr::Variable]);
    let right = allowed_types!("equal", &args[1], [PExpr::Integer, PExpr::Variable]);

    return Ok(PExpr::Equal(Box::new(left), Box::new(right)));
}

fn eval_set(args: &Vec<ASTree>) -> Result<PExpr, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::InvalidArgumentCount(String::from("set"), 2, args.len()));
    }

    let var = allowed_types!("set", &args[0], [PExpr::Variable]);
    let val = allowed_types!("set", &args[1], [PExpr::Integer, PExpr::Variable, PExpr::Plus]);

    return Ok(PExpr::Assign(Box::new(var), Box::new(val)));
}

fn eval_plus(args: &Vec<ASTree>) -> Result<PExpr, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::InvalidArgumentCount(String::from("+"), 2, args.len()));
    }

    let left  = allowed_types!("+", &args[0], [PExpr::Integer, PExpr::Variable]);
    let right = allowed_types!("+", &args[1], [PExpr::Integer, PExpr::Variable]);

    return Ok(PExpr::Plus(Box::new(left), Box::new(right)));
}
*/
//fn eval_set_episode(args:)


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
