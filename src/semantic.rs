


use parser::PExpr;
use types::{FloorType, Section};
use std::collections::HashMap;


#[derive(Debug)]
pub enum SyntaxError {
    UnknownFunction(String),
    InvalidFunction(String),
    InvalidNumberOfArguments(String, u32, usize),
    InvalidArgument(String, String, String),
    UnknownVariable(String),
    UnknownMonster(String),
    UnknownFloor(String),
    WaveAlreadyDefined(String),
}

/*pub struct SymbolValue {
    
}

pub struct Symbol {
    identifier: String,
    value: SymbolValue,
}*/





#[derive(Debug)]
pub enum Conditional {
}

#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
pub struct Direction {
    dir: u16,
}

impl Direction {
    fn deg(deg: u32) -> Direction {
        Direction {
            dir: (deg*0xffff/360) as u16,
        }
    }
    fn raw(dir: u16) -> Direction {
        Direction {
            dir: dir,
        }
    }
}

#[derive(Debug)]
pub struct PlayerLocation {
    section: Section,
    pos: Position,
    dir: Direction,
}

#[derive(Debug)]
pub enum Expr {
    Noop,
    QuestName {
        name: String,
    },
    QuestDescription {
        desc: String,
    },
    QuestDescriptionLong {
        desc: String,
    },
    SetEpisode {
        episode: u8,
    },
    SetPlayerLocation {
        floor: FloorType,
        p1: PlayerLocation,
        p2: PlayerLocation,
        p3: PlayerLocation,
        p4: PlayerLocation,
    }
    
}


macro_rules! expect_type {
    ($arg:expr, $t:path) => {
        match $arg {
            $t(ref var) => Ok(var.clone()),
            _ => {
                Err(SyntaxError::InvalidArgument(String::from(format!("{}:{}", module_path!(), line!())),
                                                        $arg.to_string(),
                                                        String::from("expected different type")))
            }
        }
    }
}

macro_rules! expect_variable {
    ($set:expr, $var:expr) => {
        match $set.get(&$var) {
            Some(v) => Ok(v.clone()),
            None => Err(SyntaxError::UnknownVariable($var))
        }
    }
}


macro_rules! expect_len {
    ($arg:expr, $len:expr) => {
        if $arg.len() != $len {
            return Err(SyntaxError::InvalidNumberOfArguments(format!("{:?}", $arg), $len, $arg.len()));
        }
    }
}


fn quest_name(args: &Vec<PExpr>) -> Result<Expr, SyntaxError> {
    expect_len!(args, 1);
    
    Ok(Expr::QuestName {
        name: expect_type!(args[0], PExpr::StringLiteral)?
    })
}

fn quest_description(args: &Vec<PExpr>) -> Result<Expr, SyntaxError> {
    expect_len!(args, 1);
    
    Ok(Expr::QuestDescription {
        desc: expect_type!(args[0], PExpr::StringLiteral)?
    })
}

fn quest_description_long(args: &Vec<PExpr>) -> Result<Expr, SyntaxError> {
    expect_len!(args, 1);
    
    Ok(Expr::QuestDescriptionLong {
        desc: expect_type!(args[0], PExpr::StringLiteral)?
    })
}

fn set_episode(args: &Vec<PExpr>) -> Result<Expr, SyntaxError> {
    expect_len!(args, 1);

    Ok(Expr::SetEpisode {
        episode: expect_type!(args[0], PExpr::Number)? as u8
    })
}

fn map(map: &PExpr) -> Result<FloorType, SyntaxError> {
    let m = expect_type!(map, PExpr::Map)?;
    expect_len!(m, 3);

    Ok(FloorType::new(expect_type!(m[0], PExpr::Identifier)?,
                      expect_type!(m[1], PExpr::Number)? as u32,
                      expect_type!(m[2], PExpr::Number)? as u32))
}

fn set_floor(args: &Vec<PExpr>, floors: &mut HashMap<String, FloorType>) -> Result<(), SyntaxError> {
    expect_len!(args, 2);

    let id = expect_type!(args[0], PExpr::Identifier)?;
    let floor = map(&args[1])?;
    floors.insert(id, floor);
    Ok(())
}

fn floor(floor: &PExpr, floors: &HashMap<String, FloorType>) -> Result<FloorType, SyntaxError> {
    let f = expect_type!(floor, PExpr::Floor)?;
    expect_len!(f, 1);

    Ok(expect_variable!(floors, expect_type!(f[0], PExpr::Identifier)?)?)
}

fn section(sec: &PExpr) -> Result<Section, SyntaxError> {
    let section = expect_type!(sec, PExpr::Section)?;
    expect_len!(section, 1);

    Ok(expect_type!(section[0], PExpr::Number)? as u32)
    
}

fn position(pos : &PExpr) -> Result<Position, SyntaxError> {
    let p = expect_type!(pos, PExpr::Position)?;
    expect_len!(p, 3);
    
    Ok(Position {
        x: expect_type!(p[0], PExpr::Number)?,
        y: expect_type!(p[1], PExpr::Number)?,
        z: expect_type!(p[2], PExpr::Number)?,
    })
}

fn direction(dir : &PExpr) -> Result<Direction, SyntaxError> {
    let d = expect_type!(dir, PExpr::Direction)?;
    expect_len!(d, 1);

    Ok(Direction::deg(expect_type!(d[0], PExpr::Number)? as u32))
}



fn player_location(sec: &PExpr, pos: &PExpr, dir: &PExpr) -> Result<PlayerLocation, SyntaxError> {
    Ok(PlayerLocation {
        section: section(sec)?,
        pos: position(pos)?,
        dir: direction(dir)?,
    })
}

fn set_player_location(args: &Vec<PExpr>, floors: &HashMap<String, FloorType>) -> Result<Expr, SyntaxError> {
    expect_len!(args, 13);

    Ok(Expr::SetPlayerLocation {
        floor: floor(&args[0], floors)?,
        p1: player_location(&args[1], &args[2], &args[3])?,
        p2: player_location(&args[4], &args[5], &args[6])?,
        p3: player_location(&args[7], &args[8], &args[9])?,
        p4: player_location(&args[10], &args[11], &args[12])?,
    })
}




pub struct Semantic {
    pub expressions: Vec<PExpr>,
    //symbols: HashMap<String, Symbol>,
    floors: HashMap<String, FloorType>,
}



impl Semantic {
    pub fn new(expr: Vec<PExpr>) -> Semantic {
        Semantic {
            expressions: expr,
            floors: HashMap::new(),
        }
    }
    
    pub fn semantic(mut self) -> Result<Vec<Expr>, SyntaxError> {
        let mut semexp = Vec::new();
        
        for ex in self.expressions {
            match ex {
                PExpr::QuestName(args) => semexp.push(quest_name(&args)?),
                PExpr::QuestDescription(args) => semexp.push(quest_description(&args)?),
                PExpr::QuestDescriptionLong(args) => semexp.push(quest_description_long(&args)?),
                PExpr::SetEpisode(args) => semexp.push(set_episode(&args)?),
                
                PExpr::SetFloor(args) => set_floor(&args, &mut self.floors)?,
                
                PExpr::SetPlayerLocation(args) => semexp.push(set_player_location(&args, &self.floors)?),
                _ => {}
                //_ => {return Err(SyntaxError::InvalidFunction(ex.to_string()))}
            }
        }

        Ok(semexp)
    }
}
