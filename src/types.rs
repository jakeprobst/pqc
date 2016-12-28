




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
    
pub enum Floor {
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
    // general operations
    //If(Box<PExpr>, Box<PExpr>, Box<PExpr>), // if ($1) then $2 else $3
    If(Vec<PExpr>),
    Block(Vec<PExpr>),
    //Equal(Box<PExpr>, Box<PExpr>),
    Equal(Vec<PExpr>),
    Integer(u32),
    Identifier(String),
    Array(Vec<PExpr>),
    Register(Variable),
    //Set(Box<PExpr>, Box<PExpr>),
    Set(Vec<PExpr>),

    // math
    //Plus(Box<PExpr>, Box<PExpr>),
    Plus(Vec<PExpr>),

    // general meta pso
    //SetEpisode(Box<PExpr>),
    SetEpisode(Vec<PExpr>),
    //SetFloors(Box<PExpr>),
    SetFloor(Vec<PExpr>),
    //FloorMapping(String, String),

    // npcs
    NPC(Vec<PExpr>),

    // doors
    Door(Vec<PExpr>),

    // wave
    Wave(Vec<PExpr>),

    // monsters
    
}

pub struct Quest {
    pub episode: u32,

    pub on_start: PExpr,
    pub on_success: PExpr,
    pub on_failure: PExpr,

    pub objects: Vec<Object>,
    pub monsters: Vec<Monster>,
    pub npcs: Vec<NPC>,
    pub waves: Vec<Wave>,
}

pub enum Eval {
    EMonster(Monster),
    EObject(Object),
    EPExpr(PExpr),
}






