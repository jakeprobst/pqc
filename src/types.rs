use std::fmt;
use monster::*;
use std::collections::HashMap;

type Register = u8;

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub enum ObjectType {
    SetPlayerLocation(u32), // player #, 
}

#[derive(Debug)]
pub struct Object {
    pub otype: ObjectType,
    pub floor: String,
    pub pos: Point,
    pub dir: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FloorType {
    Pioneer2,
    Forest1,
    Forest2,
    Dragon,
    Caves1(u32),
    Caves2(u32),
    Caves3(u32),
    DeRolLe,
    Mines(u32),
    Mines2(u32),
    VolOpt,
    Ruins1(u32),
    Ruins2(u32),
    Ruins3(u32),
    DarkFalz,
    // palace, spaceship
    Lab,
    Temple(u32),
    BarbaRay,
    Spaceship(u32),
    GolDragon,
    CCA,
    JungleEast,
    JungleNorth,
    Mountain(u32),
    Seaside,
    SeasideNight,
    GalGryphon,
    SeabedUpper(u32),
    SeabedLower(u32),
    OlgaFlow,
    Tower(u32),
}

impl FloorType {
    pub fn new(area: String, subarea: u32, layout: u32) -> FloorType {
        match (area.as_ref(), subarea, layout) {
            ("caves", 1, _) => FloorType::Caves1(layout),
            ("caves", 2, _) => FloorType::Caves2(layout),
            ("caves", 3, _) => FloorType::Caves3(layout),
            _ => panic!("bad map")
        }
    }
}

impl<'a> From<&'a FloorType> for u32 {
    fn from(floor: &'a FloorType) -> u32 {
        match floor {
            &FloorType::Caves1(..) | &FloorType::Caves2(..) | &FloorType::Caves3(..) => 3,
            _ => 0
        }
    }
}

#[derive(Debug)]
pub struct NPC {
}

#[derive(Debug)]
pub struct Wave {
    pub id: u32,
    pub floor: FloorType,
    pub section: u32,
    pub monsters: Vec<Monster>,
    pub next: Vec<u32>,
    pub unlock: Vec<u16>,
    pub delay: u16,
}

#[derive(Debug)]
pub enum VariableValue {
    None,
    //Boolean()
    Integer(u32),
    Float(f32),
    String(String),
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: VariableValue,
    //pub register: Register,
}

#[derive(Debug, Clone)]
pub enum PExpr {
    Noop,
    Integer(u32),
    Float(f32),
    Identifier(String),
    StringLiteral(String),
    
    // general operations
    Block(Vec<PExpr>),
    Equal(Vec<PExpr>),
    If(Vec<PExpr>),
    Set(Vec<PExpr>),
    Variable(Vec<PExpr>),

    // math
    Plus(Vec<PExpr>),

    // general
    Floor(Vec<PExpr>),
    Map(Vec<PExpr>),
    Section(Vec<PExpr>),
    Position(Vec<PExpr>),
    Direction(Vec<PExpr>),

    // general meta pso
    OnFloorLoad(Vec<PExpr>),
    SetPlayerLocation(Vec<PExpr>),
    QuestSuccess(Vec<PExpr>),
    QuestFailure(Vec<PExpr>),
    SetEpisode(Vec<PExpr>),
    SetFloor(Vec<PExpr>),

    // npcs
    Npc(Vec<PExpr>),
    NpcAction(Vec<PExpr>),
    NpcSay(Vec<PExpr>),

    // doors
    Door(Vec<PExpr>),

    // wave
    Wave(Vec<PExpr>),
    Delay(Vec<PExpr>),
    NextWave(Vec<PExpr>),
    Spawn(Vec<PExpr>),
    Unlock(Vec<PExpr>),

}

macro_rules! print_expr {
    ($f:expr, $func:expr, $args:expr) => {
        {
            write!($f, "(");
            write!($f, $func);
            for a in $args.iter() {
                write!($f, " {}", a);
            }
            write!($f, ")")
        }
    }
}


impl fmt::Display for PExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &PExpr::If(ref args) => print_expr!(f, "if", args),
            &PExpr::Equal(ref args) => print_expr!(f, "equal", args),
            &PExpr::Set(ref args) => print_expr!(f, "set", args),
            &PExpr::Plus(ref args) => print_expr!(f, "+", args),
            &PExpr::SetEpisode(ref args) => print_expr!(f, "set-episode", args),
            
            &PExpr::Integer(ref args) => {
                write!(f, "{}", args)
            },
            &PExpr::Identifier(ref args) => {
                write!(f, "{}", args)
            },
            _ => write!(f, "!!add {:?} to fmt::Display for PExpr!!", self),
        }
    }
}


#[derive(Debug)]
pub struct Quest {
    pub episode: u32,

    //pub on_start: PExpr,
    pub on_success: PExpr,
    pub on_failure: PExpr,
    
    pub floors: HashMap<String, FloorType>,
    pub objects: Vec<Object>,
    //pub monsters: Vec<Monster>,
    pub variables: Vec<Variable>,
    pub npcs: Vec<NPC>,
    pub waves: Vec<Wave>,
}





