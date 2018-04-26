use std::fmt;
use monster::*;
use object::*;
use npc::*;
use std::collections::HashMap;
use parser::PExpr;


#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Section = u32;
/*pub struct Section {
    
}*/

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Copy)]
pub enum FloorType {
    Pioneer2,
    Forest1,
    Forest2,
    Dragon,
    Caves1(u32),
    Caves2(u32),
    Caves3(u32),
    DeRolLe,
    Mines1(u32),
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

// TODO: TryFrom where it dies on bad map layout numbers?
// TODO: ep2/4
// TODO: check e1 boss values for correctness?
impl FloorType {
    pub fn new(area: String, subarea: u32, layout: u32) -> FloorType {
        match (area.as_ref(), subarea, layout) {
            ("pioneer2", _, _) => FloorType::Pioneer2,
            ("forest", 1, _) => FloorType::Forest1,
            ("forest", 2, _) => FloorType::Forest2,
            ("caves", 1, _) => FloorType::Caves1(layout),
            ("caves", 2, _) => FloorType::Caves2(layout),
            ("caves", 3, _) => FloorType::Caves3(layout),
            ("mines", 1, _) => FloorType::Mines1(layout),
            ("mines", 2, _) => FloorType::Mines2(layout),
            ("ruins", 1, _) => FloorType::Ruins1(layout),
            ("ruins", 2, _) => FloorType::Ruins2(layout),
            ("ruins", 3, _) => FloorType::Ruins3(layout),
            ("dragon", _, _) => FloorType::Dragon,
            ("de-rol-le", _, _) => FloorType::DeRolLe,
            ("vol-opt", _, _) => FloorType::VolOpt,
            ("dark-falz", _, _) => FloorType::DarkFalz,
            _ => panic!("bad map")
        }
    }
}

// TODO: ep2/4
// TODO: check e1 boss values for correctness?
impl<'a> From<&'a FloorType> for u32 {
    fn from(floor: &'a FloorType) -> u32 {
        match floor {
            &FloorType::Pioneer2 => 0,
            &FloorType::Forest1 => 1,
            &FloorType::Forest2 => 2,
            &FloorType::Caves1(..) => 3,
            &FloorType::Caves2(..) => 4,
            &FloorType::Caves3(..) => 5,
            &FloorType::Mines1(..) => 6,
            &FloorType::Mines2(..) => 7,
            &FloorType::Ruins1(..) => 8,
            &FloorType::Ruins2(..) => 9,
            &FloorType::Ruins3(..) => 10,
            &FloorType::Dragon => 11,
            &FloorType::DeRolLe => 12,
            &FloorType::VolOpt => 13,
            &FloorType::DarkFalz => 14,
            _ => 0
        }
    }
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
    Boolean(bool),
    Integer(i32),
    Float(f32),
    String(String),
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: VariableValue,
    //pub register: Register,
}

/*pub struct Function {
    pub name: String,
    pub id: f32,
    pub expr: PExpr,
}*/

#[derive(Debug, Clone)]
pub enum Function {
    Id(f32),
    Expr(PExpr),
}



/*macro_rules! print_expr {
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
}*/



#[derive(Debug)]
pub struct Quest {
    pub episode: u32,

    // TODO: redundant names
    pub quest_name: String,
    pub quest_description: String,
    pub quest_description_long: String,
    
    //pub on_start: PExpr,
    pub on_success: PExpr,
    pub on_failure: PExpr,
    
    //pub floors: HashMap<String, FloorType>,
    pub floors: Vec<FloorType>,
    pub objects: Vec<Object>,
    //pub npcs: Vec<Npc>,
    pub npcs: HashMap<String, Npc>,
    pub variables: Vec<Variable>,
    pub functions: HashMap<String, PExpr>,
    pub waves: Vec<Wave>,
}





