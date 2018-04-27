use types::*;
//use std::ops::Fn;
use std;
use semantic::Semantic;

#[derive(Debug, Clone)]
pub enum PExpr {
    Noop,
    //Integer(i32),
    //Float(f32),
    Number(f32),
    Boolean(bool),
    Identifier(String),
    StringLiteral(String),

    // quest meta
    QuestName(Vec<PExpr>),
    QuestDescription(Vec<PExpr>),
    QuestDescriptionLong(Vec<PExpr>),
    
    // general operations
    Block(Vec<PExpr>),
    Cond(Vec<PExpr>),
    Equal(Vec<PExpr>),
    If(Vec<PExpr>),
    Set(Vec<PExpr>),
    Variable(Vec<PExpr>),
    Let(Vec<PExpr>),
    Var(Vec<PExpr>),

    // math
    Plus(Vec<PExpr>),

    // general
    Floor(Vec<PExpr>),
    Map(Vec<PExpr>),
    Section(Vec<PExpr>),
    Position(Vec<PExpr>),
    Direction(Vec<PExpr>),

    // general meta pso
    GetDifficulty(Vec<PExpr>),
    OnFloorLoad(Vec<PExpr>),
    SetPlayerLocation(Vec<PExpr>),
    QuestSuccess(Vec<PExpr>),
    QuestFailure(Vec<PExpr>),
    SetEpisode(Vec<PExpr>),
    SetFloor(Vec<PExpr>),


    // pso stuff?
    GiveMeseta(Vec<PExpr>),
    PlayBgm(Vec<PExpr>),
    WindowMessage(Vec<PExpr>),
    
    // npcs
    Npc(Vec<PExpr>),
    //NpcAction(Vec<PExpr>),
    NpcSay(Vec<PExpr>),
    Skin(Vec<PExpr>),

    // objects
    CollisionEvent(Vec<PExpr>),

    Radius(Vec<PExpr>),
    Action(Vec<PExpr>),
    
    // doors
    Door(Vec<PExpr>),
    Type(Vec<PExpr>),

    // wave
    Wave(Vec<PExpr>),
    Delay(Vec<PExpr>),
    NextWave(Vec<PExpr>),
    Spawn(Vec<PExpr>),
    Unlock(Vec<PExpr>),
    StartWave(Vec<PExpr>),

    // monster attributes
    IdleDistance(Vec<PExpr>),

    // objects
    Object(Vec<PExpr>),

}


impl std::fmt::Display for PExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


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

pub fn generate_ast(tokens: &Vec<String>) -> Result<ASTree, ParseError> {
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
                ["let", PExpr::Variable],
                ["var", PExpr::Variable],

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
            /*if let Ok(i) = val.parse::<i32>() {
                Ok(PExpr::Integer(i))
            }*/
            if let Ok(i) = val.parse::<f32>() {
                Ok(PExpr::Number(i))
            }
            else if val.starts_with('"') {
                Ok(PExpr::StringLiteral(val.chars().skip(1).take(val.len()-2).collect()))
            }
            /*else if val.starts_with('@') {
                Ok(PExpr::StringLiteral(val.chars().skip(1).take(val.len()-2).collect()))
            }*/
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

fn eval_tokenized_expr(tokens: Vec<String>) -> Result<Vec<PExpr>, ParseError> {
    let mut result = Vec::new();

    let astblocks = split_into_ast_blocks(&tokens)?;
    
    for ast in astblocks {
        let astree = generate_ast(&ast)?;
        result.push(eval_ast(&astree)?);
    }
    
    return Ok(result);
}










pub struct Parser {
    pub tokens: Vec<String>,
}


impl Parser {
    pub fn new(tokens: Vec<String>) -> Parser {
        Parser {
            tokens: tokens
        }
    }
    
    pub fn parse(self) -> Result<Semantic, ParseError> {
        Ok(Semantic::new(eval_tokenized_expr(self.tokens)?))
    }
}
