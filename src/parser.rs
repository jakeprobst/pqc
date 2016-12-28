use types::*;
use std::ops::Fn;


#[derive(Debug)]
pub enum ParseError {
    ExpectedOpenParen,
    UnbalancedParens,
    UnknownFunction(String),
}

/*#[derive(Debug)]
pub enum EvalError {
    Bleh,
    ExpectedOpenParen,
    UnexpectedFunction(String),
    UnknownFunction(String),
    InvalidArgumentCount(String, usize, usize),
    InvalidArgument(String, String),
}*/

#[derive(Debug)]
pub enum ASTree {
    Node(String, Vec<ASTree>),
    Value(String),
}

/*#[derive(Debug)]
pub enum ScriptError {
    SyntaxError(SyntaxError),
    //EvalError(EvalError),
}

impl From<SyntaxError> for ScriptError {
    fn from(err: SyntaxError) -> ScriptError {
        ScriptError::SyntaxError(err)
    }
}*/

/*impl From<EvalError> for ScriptError {
    fn from(err: EvalError) -> ScriptError {
        ScriptError::EvalError(err)
    }
}*/


// TODO: handle quotes
pub fn tokenize_str(expr: &str) -> Vec<String> {
    let mut result = Vec::new();

    let mut current = String::new();
    let mut in_quote = false;
    for c in expr.chars() {
        if in_quote {
            current.push(c);
            if c == '"' {
                in_quote = false;
            }
        }
        else if c == '(' || c == ')' {
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
        else if c == '"' {
            current.push(c);
            in_quote = true;
        }
        else {
            current.push(c);
        }
    }

    return result;
}

// TODO: figure out a proper name for this function
fn split_into_ast_blocks(tokens: &Vec<String>) -> Result<Vec<Vec<String>>, ParseError> {
    let mut result = Vec::new();

    let mut parencount = 0;
    let mut bracketcount = 0;
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
        return Err(ParseError::UnbalancedParens);
    }

    return Ok(result);
}

fn generate_ast(tokens: &Vec<String>) -> Result<ASTree, ParseError> {
    let ref function = tokens[1];
    let childblocks = try!(split_into_ast_blocks(&tokens[2..tokens.len()-1].to_vec()));
    let mut childast = Vec::new();
    
    for cb in childblocks.iter() {
        if cb[0] == "(" {
            match generate_ast(cb) {
                Ok(ast) => childast.push(ast),
                Err(ParseError::ExpectedOpenParen) => {},
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



// TODO: better name for this
/*macro_rules! allowed_types {
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
}*/

/*
macro_rules! eval_statement_match {
    ($func:expr, $args:expr,
     { $([$name:expr, $typ:path, $len:expr, { $( $var:ident: [ $( $t:path ),* ] ),* } ] ),* }) => {
        {
            match $func {
                $(
                    $name => {
                        if len != -1 && $args.len() != $len {
                            Err(EvalError::InvalidArgumentCount(String::from($name), $len, $args.len()))
                        }
                        /*else if $len == -1 {
                            $(
                                ;
                                let mut $var = Vec::new();
                                for count in 0..$args.len() {
                                    $var.push(allowed_types!($name, &$args[count], [ $( $t ),*]));
                                }
                                Ok($typ($var))
                            )*
                        }*/
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
*/

/*macro_rules! eval_statement_match2 {
    ($func:expr, $args:expr,
     { $( [$name:expr, $typ:path, [ $( $var:ident ),* $( |$array:ident|  )]] ),* }) => {
        {
            match $func {
                $(
                    $name => {
                        let mut len = 0;
                        $( // get number of named args
                            let $var = 0;
                            len += 1;
                        )*
                        if $args.len() != len {
                            Err(EvalError::InvalidArgumentCount(String::from($name), len, $args.len()))
                        }
                        else {
                            /*let mut vz = Vec::new();
                            for count in 0..$args.len() {
                                //vz.push(allowed_types!($name, &$args[count], [ $( $t ),*]));
                                vz.push(try!(eval_ast(&$args[count])));
                            }*/

                            let mut count = 0;
                            $(
                                //let $var = allowed_types!($name, &$args[count], [ $( $t ),*]);
                                let $var = try!(eval_ast(&$args[count]));
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
}*/

macro_rules! eval_statement_match {
    ($func:expr, $args:expr,
     { $( [$name:expr, $typ:path] ),* }) => {
        {
            match $func {
                $(
                    $name => {
                        let mut vz = Vec::new();
                        for count in 0..$args.len() {
                            //vz.push(allowed_types!($name, &$args[count], [ $( $t ),*]));
                            vz.push(try!(eval_ast(&$args[count])));
                        }
                        
                        Ok($typ(vz))
                    }
                )*,
                _ => Err(ParseError::UnknownFunction(String::from($func)))
            }
        }
        
    }
}

fn eval_ast(ast: &ASTree)-> Result<PExpr, ParseError> {
    match ast {
        &ASTree::Node(ref function, ref args) => {
            eval_statement_match!(function as &str, &args, {
                ["if", PExpr::If],
                ["equal", PExpr::Equal],
                ["set", PExpr::Set],
                ["+", PExpr::Plus],
                ["set-episode", PExpr::SetEpisode],
                ["set-floors", PExpr::SetFloors]
            })
            /*eval_statement_match2!(function as &str, &args, {
                ["if", PExpr::If, [cond, btrue, bfalse]],
                ["equal", PExpr::Equal, [left, right]],
                ["set", PExpr::Set, [var, val]],
                ["+", PExpr::Plus, [left, right]],
                ["set-episode", PExpr::SetEpisode, [|ep|]]
                //["set-floors", PExpr::SetFloors, [floors!]]
            })*/
        }
            /*eval_statement_match!(function as &str, &args, {
                // general commands
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
                }],


                //
                ["set-episode", PExpr::SetEpisode, 1, {
                    ep: [PExpr::Integer]
                }],
                ["floors", PExpr::SetFloors, -1, {
                    floors: [PExpr::Array]
                }
                ]
                    

                    
                   
            })
        }*/
        &ASTree::Value(ref val) => {
            if let Ok(i) = val.parse::<u32>() {
                Ok(PExpr::Integer(i))
            }
            else {
                Ok(PExpr::Identifier(val.clone()))
            }
        }
    }
}

pub fn eval_tokenized_expr(tokens: Vec<String>) -> Result<Vec<PExpr>, ParseError> {
    let mut result = Vec::new();

    let astblocks = try!(split_into_ast_blocks(&tokens));
    
    for ast in astblocks {
        let astree = try!(generate_ast(&ast));
        println!("astree: {:?}", astree);
        result.push(try!(eval_ast(&astree)));
    }
    

    return Ok(result);
}


/*




fn parse_script(script: String) -> Quest {

    
}
*/
