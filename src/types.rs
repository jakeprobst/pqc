
use std::fmt;



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

#[derive(Debug, Clone)]
pub enum MonsterType {
    Booma,
    Gibooma,
    Gigobooma,
    // etc
}

#[derive(Debug, Clone)]
pub enum CavesLayout {
    Caves1,
    Caves2,
    Caves3,
    Caves4,
    Caves5,
}

#[derive(Debug, Clone)]
pub enum MinesLayout {
    Mines1,
    Mines2,
    Mines3,
    Mines4,
    Mines5,
}

#[derive(Debug, Clone)]
pub enum RuinsLayout {
    Ruins1,
    Ruins2,
    Ruins3,
    Ruins4,
    Ruins5,
}

#[derive(Debug, Clone)]
pub enum TempleLayout {
    Temple1,
    Temple2,
    Temple3,
}

#[derive(Debug, Clone)]
pub enum SpaceshipLayout {
    Spaceship1,
    Spaceship2,
    Spaceship3,
}

#[derive(Debug, Clone)]
pub enum MountainLayout {
    Mountain1,
    Mountain2,
    Mountain3,
}

#[derive(Debug, Clone)]
pub enum SeabedLayout {
    Seabed1,
    Seabed2,
    Seabed3,
}

#[derive(Debug, Clone)]
pub enum TowerLayout {
    Tower1,
    Tower2,
    Tower3,
    Tower4,
    Tower5,
}
    
#[derive(Debug, Clone)]
pub enum FloorType {
    Pioneer2,
    Forest1,
    Forest2,
    Dragon,
    Caves1(CavesLayout),
    Caves2(CavesLayout),
    Caves3(CavesLayout),
    DeRolLe,
    Mines(MinesLayout),
    Mines2(MinesLayout),
    VolOpt,
    Ruins1(RuinsLayout),
    Ruins2(RuinsLayout),
    Ruins3(RuinsLayout),
    DarkFalz,
    // palace, spaceship
    Lab,
    Temple(TempleLayout),
    BarbaRay,
    Spaceship(SpaceshipLayout),
    GolDragon,
    CCA,
    JungleEast,
    JungleNorth,
    Mountain(MountainLayout),
    Seaside,
    SeasideNight,
    GalGryphon,
    SeabedUpper(SeabedLayout),
    SeabedLower(SeabedLayout),
    OlgaFlow,
    Tower(TowerLayout),
}

#[derive(Debug)]
pub struct Floor {
    pub label: String,
    pub floor: FloorType,
}

#[derive(Debug)]
pub struct Monster {
    pub mtype: MonsterType,
    pub floor: String,
    pub pos: Point,
   
    
}

#[derive(Debug)]
pub struct NPC {
}

#[derive(Debug)]
pub struct Wave {
    pub label: String,
    pub monsters: Vec<Monster>,
    pub next: Vec<String>,
    pub unlock: Vec<String>,
    pub delay: u32,
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

    // monsters
    
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
    
    pub floors: Vec<Floor>,
    pub objects: Vec<Object>,
    //pub monsters: Vec<Monster>,
    pub variables: Vec<Variable>,
    pub npcs: Vec<NPC>,
    pub waves: Vec<Wave>,
}





