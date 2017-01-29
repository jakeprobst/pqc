use types::*;
use std::ops::Fn;


#[derive(Debug)]
pub enum ParseError {
    ExpectedOpenParen,
    UnbalancedParens,
    UnknownFunction(String),
}

#[derive(Debug)]
pub enum ASTree {
    Node(String, Vec<ASTree>),
    Value(String),
}

pub fn tokenize_script(script: &str) -> Vec<String> {
    let mut result = Vec::new();

    let mut current = String::new();
    let mut in_quote = false;
    let mut in_comment = false;
    for c in script.chars() {
        if in_quote {
            current.push(c);
            if c == '"' {
                in_quote = false;
            }
        }
        else if in_comment {
            if c == '\n' {
                in_comment = false;
            }
        }
        else if c == '#' {
            in_comment = true;
        }
        else if c == '(' || c == ')' {
            if current.len() > 0 {
                result.push(current);
                current = String::new();
            }
            result.push(c.to_string());
        }
        else if c.is_whitespace() {
            if current.len() > 0 {
                result.push(current);
                current = String::new();
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

macro_rules! eval_statement_match {
    ($func:expr, $args:expr,
     { $( [$name:expr, $typ:path] ),* }) => {
        {
            match $func {
                $(
                    $name => {
                        let mut vz = Vec::new();
                        for count in 0..$args.len() {
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
                // quest meta
                ["quest-name", PExpr::QuestName],
                ["quest-description", PExpr::QuestDescription],
                ["quest-description-long", PExpr::QuestDescriptionLong],
                
                // general operations
                ["block", PExpr::Block],
                ["equal", PExpr::Equal],
                ["if", PExpr::If],
                ["set", PExpr::Set],

                // math
                ["+", PExpr::Plus],

                // pso meta
                ["get-difficulty", PExpr::GetDifficulty],
                ["on-floor-load", PExpr::OnFloorLoad],
                ["set-player-location", PExpr::SetPlayerLocation],
                ["quest-success", PExpr::QuestSuccess],
                ["quest-failure", PExpr::QuestFailure],
                ["set-episode", PExpr::SetEpisode],
                ["set-floor", PExpr::SetFloor],
                ["variable", PExpr::Variable],

                // pso stuff
                ["give-meseta", PExpr::GiveMeseta],
                ["play-bgm", PExpr::PlayBgm],
                ["window-message", PExpr::WindowMessage],

                // general
                ["floor", PExpr::Floor],
                ["map", PExpr::Map],
                ["section", PExpr::Section],
                ["pos", PExpr::Position],
                ["dir", PExpr::Direction],
                
                // npcs
                ["npc", PExpr::Npc],
                //["npc-action", PExpr::NpcAction],
                ["npc-say", PExpr::NpcSay],
                ["skin", PExpr::Skin],

                // objects
                ["collision-event", PExpr::CollisionEvent],

                ["radius", PExpr::Radius],
                ["action", PExpr::Action],
                
                // doors
                ["door", PExpr::Door],
                ["type", PExpr::Type],

                // wave
                ["wave", PExpr::Wave],
                ["spawn", PExpr::Spawn],
                ["next-wave", PExpr::NextWave],
                ["delay", PExpr::Delay],
                ["unlock", PExpr::Unlock],
                ["start-wave", PExpr::StartWave],


                // monster attributes
                ["idle-distance", PExpr::IdleDistance],

                // objects
                ["object", PExpr::Object]

                    
            })
        }
        &ASTree::Value(ref val) => {
            if let Ok(i) = val.parse::<u32>() {
                Ok(PExpr::Integer(i))
            }
            else if val.starts_with('"') {
                Ok(PExpr::StringLiteral(val.chars().skip(1).take(val.len()-2).collect()))
            }
            else if val == "true" {
                Ok(PExpr::Boolean(true))
            }
            else if val == "false" {
                Ok(PExpr::Boolean(false))
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


pub fn parse_script_to_expr(script: &str) -> Result<Vec<PExpr>, ParseError> {
    
    let tokens = tokenize_script(script);
    Ok(try!(eval_tokenized_expr(tokens)))
}

