use types::*;

use byteorder::{LittleEndian, WriteBytesExt};
use std::convert::From;

pub enum MonsterError {
    NoSuchMonster(String)
}

// TODO: reorder these in some reasonable way, copied from qedit order
// TODO: rare enemies
#[derive(Debug, Clone)]
pub enum MonsterType {
    None,
    Booma,
    Gobooma,
    Gigobooma,
    SavageWolf,
    BarbarousWolf,
    Monest,
    RagRappy,
    SandRappy,
    Hildebear,
    Gilchic,
    Dubchic,
    Garanz,
    SinowBlue,
    SinowGold,
    InvisibleSinow, // ?? in qedit list
    Canadine,
    ZondingCanadine, // ?? might be a flag?
    Canane,
    DubchicSwitch,
    PoisonLily,
    DelLily,
    PofuillySlime,
    PalShark,
    GuilShark,
    NanoDragon,
    EvilShark,
    PanArms,
    Claw,
    Dimenian,
    DarkBelra,
    Delsaber,
    ChaosSorcerer,
    Bulclaw,
    DarkGunner,
    ChaosBringer,
    LaDimenian,
    SoDimenian,
    GrassAssassin,
    Dragon,
    DeRolLe,
    VolOptPartA,
    VolOpt,
    DarkFalz,
    Epsilon,
    Gee,
    GiGue,
    ReconBox, // 5, 10?
    //ReconBox,
    Gibbles,
    Morfos,
    IllGill,
    Mericarol,
    Merikle,
    Mericus,
    Merillias,
    Meriltas,
    SinowBeril,
    SinowSpigell,
    UlGibbon,
    ZolGibbon,
    Dolmdarl,
    Dolmolm,
    Delbiter,
    Deldepth,
    SinowZoa,
    SinowZele,
    GalGryphon,
    OlgaFlow,
    BarbaRay,
    GolDragon,
    Boota,
    ZeBoota,
    BaBoota,
    SatelliteLizard,
    Yowie,
    Dorphon,
    Astark,
    Girtablulu,
    MerissaA,
    Goran,
    GoranDetonator,
    PyroGoran,
    Zu,
    /*Npc01,
    NPC02,
    NPC03,
    NPC04,
    NPC05,
    NPC06,
    NPC07,
    NPC08,
    NPC09,
    NPC0A,
    NPC0B,
    NPC0C,
    NPC0D,
    NPC0E,
    NPC19Military,
    NPC1AMilitary,
    PrincipalTyrel,
    Tekker,
    Banklady,
    NPC1EScientist,
    Nurse,
    Irene,
    Broomop,
    NPC22Hunter,
    NPC24Ranger,
    NPC25Cast,
    NPC26Caseal,
    NPC27FOmarl,
    NPC28FOnewm,
    NPC29FOnewearl,
    NPC2BHUnewearl,
    NPC2CCast,
    NPC2DRAmar,
    NPC30FOmarl,
    Hopkins,
    NPC32FOnewearl,
    Ep2NPCD0,
    Ep2Directrice,
    Ep2NPCD2Millitary,
    Ep2NPCD3,
    Ep2NPCF0Military,
    Ep2NPCF1,
    Ep2NPCF2,
    Ep2Schthack,
    Ep2Leo,
    Ep2Pagini,
    Ep2NPCF6,
    Ep2Nol,
    Ep2Elly,
    Ep2NPCF9,
    Ep2NPCFA,
    Ep2NPCFB,
    Ep2NPCFCMilitary,
    Ep2NPCFD,
    Ep2Banklady,
    Ep2NPCFF,
    Momoka,*/
    SaintMillion,
    Shambertin,
}

// TODO: TryFrom?
impl From<String> for MonsterType {
    fn from(id: String) -> MonsterType {
        match id.as_ref() {
            "booma" => MonsterType::Booma,
            "gobooma" => MonsterType::Gobooma,
            "gigobooma" => MonsterType::Gigobooma,
            "savage-wolf" => MonsterType::SavageWolf,
            "barbarous-wolf" => MonsterType::BarbarousWolf,
            "monest" => MonsterType::Monest,
            "rag-rappy" => MonsterType::RagRappy,
            "sand-rappy" => MonsterType::SandRappy,
            "hildebear" => MonsterType::Hildebear,
            "gilchic" => MonsterType::Gilchic,
            "garanz" => MonsterType::Garanz,
            "sinow-blue" => MonsterType::SinowBlue,
            "sinow-gold" => MonsterType::SinowGold,
            "canadine" => MonsterType::Canadine,
            "dubchic-switch" => MonsterType::DubchicSwitch,
            "poison-lily" => MonsterType::PoisonLily,
            "pofuilly-slime" => MonsterType::PofuillySlime,
            "pal-shark" => MonsterType::PalShark,
            "guil-shark" => MonsterType::GuilShark,
            "nano-dragon" => MonsterType::NanoDragon,
            "evil-shark" => MonsterType::EvilShark,
            "pan-arms" => MonsterType::PanArms,
            "claw" => MonsterType::Claw,
            "dimenian" => MonsterType::Dimenian,
            "dark-belra" => MonsterType::DarkBelra,
            "delsaber" => MonsterType::Delsaber,
            "chaos-sorcerer" => MonsterType::ChaosSorcerer,
            "bulclaw" => MonsterType::Bulclaw,
            "dark-gunner" => MonsterType::DarkGunner,
            "chaos-bringer" => MonsterType::ChaosBringer,
            "la-dimenean" => MonsterType::LaDimenian,
            "so-dimenian" => MonsterType::SoDimenian,
            "grass-assassin" => MonsterType::GrassAssassin,
            "dragon" => MonsterType::Dragon,
            "de-rol-le" => MonsterType::DeRolLe,
            "vol-opt-part-a" => MonsterType::VolOptPartA,
            "vol-opt" => MonsterType::VolOpt,
            "dark-falz" => MonsterType::DarkFalz,
            "epsilon" => MonsterType::Epsilon,
            "gee" => MonsterType::Gee,
            "recon-box" => MonsterType::ReconBox,
            "gibbles" => MonsterType::Gibbles,
            "morfos" => MonsterType::Morfos,
            "ill-gill" => MonsterType::IllGill,
            "mericarol" => MonsterType::Mericarol,
            "mericus" => MonsterType::Mericus,
            "merikle" => MonsterType::Merikle,
            "merillias" => MonsterType::Merillias,
            "meriltas" => MonsterType::Meriltas,
            "sinow-beril" => MonsterType::SinowBeril,
            "silow-spigell" => MonsterType::SinowSpigell,
            "ul-gibbon" => MonsterType::UlGibbon,
            "zol-gibbon" => MonsterType::ZolGibbon,
            "dolmdarl" => MonsterType::Dolmdarl,
            "dolmolm" => MonsterType::Dolmolm,
            "delbiter" => MonsterType::Delbiter,
            "deldepth" => MonsterType::Deldepth,
            "sinow-zoa" => MonsterType::SinowZoa,
            "sinow-zele" => MonsterType::SinowZele,
            "gal-gryphon" => MonsterType::GalGryphon,
            "olga-flow" => MonsterType::OlgaFlow,
            "barba-ray" => MonsterType::BarbaRay,
            "gol-dragon" => MonsterType::GolDragon,
            "boota" => MonsterType::Boota,
            "ze-boota" => MonsterType::ZeBoota,
            "ba-boota" => MonsterType::BaBoota,
            "satellite-lizard" => MonsterType::SatelliteLizard,
            "yowie" => MonsterType::Yowie,
            "dorphon" => MonsterType::Dorphon,
            "astark" => MonsterType::Astark,
            "girtablulu" => MonsterType::Girtablulu,
            "merissa-a" => MonsterType::MerissaA,
            "goran" => MonsterType::Goran,
            "goran-detonator" => MonsterType::GoranDetonator,
            "pyro-goran" => MonsterType::PyroGoran,
            "zu" => MonsterType::Zu,
            "saint-million" => MonsterType::SaintMillion,
            "shambertin" => MonsterType::Shambertin,
            _ => MonsterType::None
        }
    }
}




#[derive(Debug)]
pub struct Monster {
    pub id: MonsterType,
    pub wave_id: u32,
    pub section: u16,
    pub pos: Point,
    pub dir: u32, // TODO: degree type?
}

impl<'a> From<&'a Monster> for Vec<u8> {
    fn from(monster: &'a Monster) -> Vec<u8> {
        match monster.id {
            //MonsterType::Booma => raw_booma(&monster),
            MonsterType::EvilShark => raw_evil_shark(&monster),
            MonsterType::PalShark => raw_pal_shark(&monster),
            MonsterType::GuilShark => raw_guil_shark(&monster),
            MonsterType::NanoDragon => raw_nano_dragon(&monster),
            MonsterType::GrassAssassin => raw_grass_assassin(&monster),
            _ => Vec::new()
        }
    }
}

fn make_monster_data(id: u16, unknown1: u16, unknown2: u32, unknown3: u16, unknown4: u16,
                     section: u16, wave_id: u32, x: f32, y: f32, z: f32, xrot: u32, yrot: u32,
                     zrot: u32, field1: f32, field2: f32, field3: f32, field4: f32, field5: f32,
                     skin: u32, field6: u32) -> Vec<u8> {
    let mut mdata = Vec::new();
    //mdata.write_u16::<LittleEndian>(monster_type_id(&monster));
    mdata.write_u16::<LittleEndian>(id);
    mdata.write_u16::<LittleEndian>(unknown1);
    mdata.write_u32::<LittleEndian>(unknown2);
    mdata.write_u16::<LittleEndian>(unknown3);
    mdata.write_u16::<LittleEndian>(unknown4);
    mdata.write_u16::<LittleEndian>(section);
    mdata.write_u16::<LittleEndian>(wave_id as u16);
    mdata.write_u32::<LittleEndian>(wave_id);
    mdata.write_f32::<LittleEndian>(x);
    mdata.write_f32::<LittleEndian>(y);
    mdata.write_f32::<LittleEndian>(z);
    mdata.write_u32::<LittleEndian>(xrot);
    mdata.write_u32::<LittleEndian>(yrot);
    mdata.write_u32::<LittleEndian>(zrot);
    mdata.write_f32::<LittleEndian>(field1);
    mdata.write_f32::<LittleEndian>(field2);
    mdata.write_f32::<LittleEndian>(field3);
    mdata.write_f32::<LittleEndian>(field4);
    mdata.write_f32::<LittleEndian>(field5);
    mdata.write_u32::<LittleEndian>(skin);
    mdata.write_u32::<LittleEndian>(field6);

    mdata
}

fn raw_evil_shark(mon: &Monster) -> Vec<u8> {
    make_monster_data(0x63, 0, 0xffff, 5, 0xffff, mon.section, mon.wave_id, mon.pos.x, mon.pos.y,
                      mon.pos.z, 0, mon.dir, 0, 0., 0., 0., 0., 0., 0, 0)
}

fn raw_pal_shark(mon: &Monster) -> Vec<u8> {
    make_monster_data(0x63, 0, 0xffff, 5, 0xffff, mon.section, mon.wave_id, mon.pos.x, mon.pos.y,
                      mon.pos.z, 0, mon.dir, 0, 0., 0., 0., 0., 0., 1, 0)
}

fn raw_guil_shark(mon: &Monster) -> Vec<u8> {
    make_monster_data(0x63, 0, 0xffff, 5, 0x1817, mon.section, mon.wave_id, mon.pos.x, mon.pos.y,
                      mon.pos.z, 0, mon.dir, 0, 0., 0., 0., 0., 0., 2, 0)
}

fn raw_nano_dragon(mon: &Monster) -> Vec<u8> {
    make_monster_data(0x62, 0, 0xffff, 5, 0x1817, mon.section, mon.wave_id, mon.pos.x, mon.pos.y,
                      mon.pos.z, 0, mon.dir, 0, 0., 0., 0., 0., 0., 0, 0)
}

fn raw_grass_assassin(mon: &Monster) -> Vec<u8> {
    make_monster_data(0x60, 0, 0xffff, 4, 0x1805, mon.section, mon.wave_id, mon.pos.x, mon.pos.y,
                      mon.pos.z, 0, mon.dir, 0, 0., 0., 0., 0., 0., 0, 0)
}
