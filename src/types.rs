
use std::fmt;



type Register = u8;

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Object {
}

pub enum MonsterType {
    Booma,
    Gibooma,
    Gigobooma,
    // etc
}

pub enum CavesLayout {
    Caves1,
    Caves2,
    Caves3,
    Caves4,
    Caves5,
}

pub enum MinesLayout {
    Mines1,
    Mines2,
    Mines3,
    Mines4,
    Mines5,
}

pub enum RuinsLayout {
    Ruins1,
    Ruins2,
    Ruins3,
    Ruins4,
    Ruins5,
}

pub enum TempleLayout {
    Temple1,
    Temple2,
    Temple3,
}

pub enum SpaceshipLayout {
    Spaceship1,
    Spaceship2,
    Spaceship3,
}

pub enum MountainLayout {
    Mountain1,
    Mountain2,
    Mountain3,
}

pub enum SeabedLayout {
    Seabed1,
    Seabed2,
    Seabed3,
}

pub enum TowerLayout {
    Tower1,
    Tower2,
    Tower3,
    Tower4,
    Tower5,
}
    
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

pub struct Floor {
    label: String,
    floor: FloorType,
}

pub struct Monster {
    pub mtype: MonsterType,
    pub floor: Floor,
    pub coord: Point,
    
}

pub struct NPC {
}

pub struct Wave {
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub register: Register,
}

#[derive(Debug)]
pub enum PExpr {
    Noop,
    Integer(u32),
    Identifier(String),
    String(String),
    // general operations
    If(Vec<PExpr>),
    Block(Vec<PExpr>),
    Equal(Vec<PExpr>),
    Set(Vec<PExpr>),

    // math
    Plus(Vec<PExpr>),

    // general
    Position(Vec<PExpr>),
    Floor(Vec<PExpr>),

    // general meta pso
    //SetEpisode(Box<PExpr>),
    SetEpisode(Vec<PExpr>),
    SetFloor(Vec<PExpr>),
    PlayerSet(Vec<PExpr>),
    QuestReward(Vec<PExpr>),

    // npcs
    Npc(Vec<PExpr>),
    NpcSay(Vec<PExpr>),

    // doors
    Door(Vec<PExpr>),

    // wave
    Wave(Vec<PExpr>),

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


pub struct Quest {
    pub episode: u32,

    pub on_start: PExpr,
    pub on_success: PExpr,
    pub on_failure: PExpr,

    pub objects: Vec<Object>,
    //pub monsters: Vec<Monster>,
    pub npcs: Vec<NPC>,
    pub waves: Vec<Wave>,
}

pub enum Eval {
    EMonster(Monster),
    EObject(Object),
    EPExpr(PExpr),
}






