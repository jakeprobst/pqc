use types::*;

use byteorder::{LittleEndian, WriteBytesExt};
use std::convert::From;

// TODO: unexpected attr error
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
    //AlRappy,
    Hildebear,
    //Hildeblue,
    Dragon,

    EvilShark,
    PalShark,
    GuilShark,
    PoisonLily,
    //NarLily,
    PofuillySlime,
    //PouillySlime,
    NanoDragon,
    PanArms,
    GrassAssassin,
    DeRolLe,
    
    Gilchic,
    Dubchic,
    SinowBlue,
    SinowGold,
    Canadine,
    Canane,
    Garanz,
    DubchicSwitch,
    VolOptPartA,
    VolOpt,
    
    Dimenian,
    LaDimenian,
    SoDimenian,
    Claw,
    Bulclaw,
    Delsaber,
    DarkBelra,
    ChaosSorcerer,
    DarkGunner,
    ChaosBringer,
    DarkFalz,
    
    BarbaRay,

    GolDragon,
    
    Gee,
    GiGue,
    UlGibbon,
    ZolGibbon,
    Gibbles,
    Merillias,
    Meriltas,
    Mericarol,
    Merikle,
    Mericus,
    GalGryphon,
    
    Dolmdarl,
    Dolmolm,
    ReconBox, // 5, 10?
    Morfos,
    SinowBeril,
    SinowSpigell,
    Delbiter,
    Deldepth,
    SinowZoa,
    SinowZele,
    OlgaFlow,
    
    DelLily,
    IllGill,
    Epsilon,
    
    Boota,
    ZeBoota,
    BaBoota,
    SandRappy,
    //DelRappy,
    SatelliteLizard,
    Yowie,
    Dorphon,
    //DorphonEclair,
    Astark,
    Zu,
    //Pazuzu,
    
    Goran,
    PyroGoran,
    GoranDetonator,
    MerissaA,
    //MerissaAA,
    Girtablulu,
    SaintMillion,
    Shambertin,
    //Kondrieu,
    
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
}

#[derive(Debug)]
pub enum MonsterAttribute {
    GroupId(u32),
    IdleDistance(f32),
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
            "dubchic" => MonsterType::Dubchic,
            "garanz" => MonsterType::Garanz,
            "sinow-blue" => MonsterType::SinowBlue,
            "sinow-gold" => MonsterType::SinowGold,
            "canadine" => MonsterType::Canadine,
            "canane" => MonsterType::Canane,
            "dubchic-switch" => MonsterType::DubchicSwitch,
            "poison-lily" => MonsterType::PoisonLily,
            "del-lily" => MonsterType::DelLily,
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
            "gi-gue" => MonsterType::GiGue,
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
    pub attributes: Vec<MonsterAttribute>,
}

impl<'a> From<&'a Monster> for Vec<u8> {
    fn from(monster: &'a Monster) -> Vec<u8> {
        match monster.id {
            MonsterType::Booma => raw_booma(&monster),
            MonsterType::Gobooma => raw_gobooma(&monster),
            MonsterType::Gigobooma => raw_gigobooma(&monster),
            MonsterType::SavageWolf => raw_savage_wolf(&monster),
            MonsterType::BarbarousWolf => raw_barbarous_wolf(&monster),
            MonsterType::Monest => raw_monest(&monster),
            MonsterType::RagRappy => raw_rag_rappy(&monster),
            MonsterType::SandRappy => raw_sand_rappy(&monster),
            MonsterType::Hildebear => raw_hildebear(&monster),
            MonsterType::Gilchic => raw_gilchic(&monster),
            MonsterType::Dubchic => raw_dubchic(&monster),
            MonsterType::Garanz => raw_garanz(&monster),
            MonsterType::SinowBlue => raw_sinow_blue(&monster),
            MonsterType::SinowGold => raw_sinow_gold(&monster),
            MonsterType::Canadine => raw_canadine(&monster),
            MonsterType::Canane => raw_canane(&monster),
            MonsterType::DubchicSwitch => raw_dubchic_switch(&monster),
            MonsterType::PoisonLily => raw_poison_lily(&monster),
            MonsterType::DelLily => raw_del_lily(&monster),
            MonsterType::PofuillySlime => raw_pofuilly_slime(&monster),
            MonsterType::PalShark => raw_pal_shark(&monster),
            MonsterType::GuilShark => raw_guil_shark(&monster),
            MonsterType::NanoDragon => raw_nano_dragon(&monster),
            MonsterType::EvilShark => raw_evil_shark(&monster),
            MonsterType::PanArms => raw_pan_arms(&monster),
            MonsterType::Claw => raw_claw(&monster),
            MonsterType::Dimenian => raw_dimenian(&monster),
            MonsterType::DarkBelra => raw_dark_belra(&monster),
            MonsterType::Delsaber => raw_delsaber(&monster),
            MonsterType::ChaosSorcerer => raw_chaos_sorcerer(&monster),
            MonsterType::Bulclaw => raw_bulclaw(&monster),
            MonsterType::DarkGunner => raw_dark_gunner(&monster),
            MonsterType::ChaosBringer => raw_chaos_bringer(&monster),
            MonsterType::LaDimenian => raw_la_dimenean(&monster),
            MonsterType::SoDimenian => raw_so_dimenian(&monster),
            MonsterType::GrassAssassin => raw_grass_assassin(&monster),
            MonsterType::Dragon => raw_dragon(&monster),
            MonsterType::DeRolLe => raw_de_rol_le(&monster),
            MonsterType::VolOptPartA => raw_vol_opt_part_a(&monster),
            MonsterType::VolOpt => raw_vol_opt(&monster),
            MonsterType::DarkFalz => raw_dark_falz(&monster),
            MonsterType::Epsilon => raw_epsilon(&monster),
            MonsterType::Gee => raw_gee(&monster),
            MonsterType::GiGue => raw_gi_gue(&monster),
            MonsterType::ReconBox => raw_recon_box(&monster),
            MonsterType::Gibbles => raw_gibbles(&monster),
            MonsterType::Morfos => raw_morfos(&monster),
            MonsterType::IllGill => raw_ill_gill(&monster),
            MonsterType::Mericarol => raw_mericarol(&monster),
            MonsterType::Mericus => raw_mericus(&monster),
            MonsterType::Merikle => raw_merikle(&monster),
            MonsterType::Merillias => raw_merillias(&monster),
            MonsterType::Meriltas => raw_meriltas(&monster),
            MonsterType::SinowBeril => raw_sinow_beril(&monster),
            MonsterType::SinowSpigell => raw_silow_spigell(&monster),
            MonsterType::UlGibbon => raw_ul_gibbon(&monster),
            MonsterType::ZolGibbon => raw_zol_gibbon(&monster),
            MonsterType::Dolmdarl => raw_dolmdarl(&monster),
            MonsterType::Dolmolm => raw_dolmolm(&monster),
            MonsterType::Delbiter => raw_delbiter(&monster),
            MonsterType::Deldepth => raw_deldepth(&monster),
            MonsterType::SinowZoa => raw_sinow_zoa(&monster),
            MonsterType::SinowZele => raw_sinow_zele(&monster),
            MonsterType::GalGryphon => raw_gal_gryphon(&monster),
            MonsterType::OlgaFlow => raw_olga_flow(&monster),
            MonsterType::BarbaRay => raw_barba_ray(&monster),
            MonsterType::GolDragon => raw_gol_dragon(&monster),
            MonsterType::Boota => raw_boota(&monster),
            MonsterType::ZeBoota => raw_ze_boota(&monster),
            MonsterType::BaBoota => raw_ba_boota(&monster),
            MonsterType::SatelliteLizard => raw_satellite_lizard(&monster),
            MonsterType::Yowie => raw_yowie(&monster),
            MonsterType::Dorphon => raw_dorphon(&monster),
            MonsterType::Astark => raw_astark(&monster),
            MonsterType::Girtablulu => raw_girtablulu(&monster),
            MonsterType::MerissaA => raw_merissa_a(&monster),
            MonsterType::Goran => raw_goran(&monster),
            MonsterType::GoranDetonator => raw_goran_detonator(&monster),
            MonsterType::PyroGoran => raw_pyro_goran(&monster),
            MonsterType::Zu => raw_zu(&monster),
            MonsterType::SaintMillion => raw_saint_million(&monster),
            MonsterType::Shambertin => raw_shambertin(&monster),
            MonsterType::None => Vec::new() // picnic!
        }
    }
}


struct RawMonsterData {
    id: u16,
    unknown1: u16,
    unknown2: u32,
    unknown3: u16,
    unknown4: u16,
    section: u16,
    wave_id: u32,
    x: f32,
    y: f32,
    z: f32,
    xrot: u32,
    yrot: u32,
    zrot: u32,
    field1: f32,
    field2: f32,
    field3: f32,
    field4: f32,
    field5: f32,
    skin: u32,
    field6: u32
}

impl RawMonsterData {
    pub fn new(id: u32, section: u16, wave_id: u32, pos: Point, yrot: u32) -> RawMonsterData {
        RawMonsterData {
            id: id as u16,
            unknown1: 0,
            unknown2: 0,
            unknown3: 0,
            unknown4: 0,
            section: section,
            wave_id: wave_id,
            x: pos.x,
            y: pos.y,
            z: pos.z,
            xrot: 0,
            yrot: yrot,
            zrot: 0,
            field1: 0.0,
            field2: 0.0,
            field3: 0.0,
            field4: 0.0,
            field5: 0.0,
            skin: 0,
            field6: 0,
        }
    }

    pub fn unknown1<'a>(&'a mut self, unknown1: u16) -> &'a mut RawMonsterData {
        self.unknown1 = unknown1;
        self
    }

    pub fn unknown2<'a>(&'a mut self, unknown2: u32) -> &'a mut RawMonsterData {
        self.unknown2 = unknown2;
        self
    }

    pub fn unknown3<'a>(&'a mut self, unknown3: u16) -> &'a mut RawMonsterData {
        self.unknown3 = unknown3;
        self
    }

    pub fn unknown4<'a>(&'a mut self, unknown4: u16) -> &'a mut RawMonsterData {
        self.unknown4 = unknown4;
        self
    }

    pub fn xrot<'a>(&'a mut self, xrot: u32) -> &'a mut RawMonsterData {
        self.xrot = xrot;
        self
    }

    pub fn zrot<'a>(&'a mut self, zrot: u32) -> &'a mut RawMonsterData {
        self.zrot = zrot;
        self
    }

    pub fn field1<'a>(&'a mut self, field1: f32) -> &'a mut RawMonsterData {
        self.field1 = field1;
        self
    }

    pub fn field2<'a>(&'a mut self, field2: f32) -> &'a mut RawMonsterData {
        self.field2 = field2;
        self
    }

    pub fn field3<'a>(&'a mut self, field3: f32) -> &'a mut RawMonsterData {
        self.field3 = field3;
        self
    }

    pub fn field4<'a>(&'a mut self, field4: f32) -> &'a mut RawMonsterData {
        self.field4 = field4;
        self
    }

    pub fn field5<'a>(&'a mut self, field5: f32) -> &'a mut RawMonsterData {
        self.field5 = field5;
        self
    }

    pub fn skin<'a>(&'a mut self, skin: u32) -> &'a mut RawMonsterData {
        self.skin = skin;
        self
    }

    pub fn field6<'a>(&'a mut self, field6: u32) -> &'a mut RawMonsterData {
        self.field6 = field6;
        self
    }
    
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut mdata = Vec::new();
        mdata.write_u16::<LittleEndian>(self.id);
        mdata.write_u16::<LittleEndian>(self.unknown1);
        mdata.write_u32::<LittleEndian>(self.unknown2);
        mdata.write_u16::<LittleEndian>(self.unknown3);
        mdata.write_u16::<LittleEndian>(self.unknown4);
        mdata.write_u16::<LittleEndian>(self.section);
        mdata.write_u16::<LittleEndian>(self.wave_id as u16);
        mdata.write_u32::<LittleEndian>(self.wave_id);
        mdata.write_f32::<LittleEndian>(self.x);
        mdata.write_f32::<LittleEndian>(self.y);
        mdata.write_f32::<LittleEndian>(self.z);
        mdata.write_u32::<LittleEndian>(self.xrot);
        mdata.write_u32::<LittleEndian>(self.yrot);
        mdata.write_u32::<LittleEndian>(self.zrot);
        mdata.write_f32::<LittleEndian>(self.field1);
        mdata.write_f32::<LittleEndian>(self.field2);
        mdata.write_f32::<LittleEndian>(self.field3);
        mdata.write_f32::<LittleEndian>(self.field4);
        mdata.write_f32::<LittleEndian>(self.field5);
        mdata.write_u32::<LittleEndian>(self.skin);
        mdata.write_u32::<LittleEndian>(self.field6);
        mdata
    }
}

fn raw_booma(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(68, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_gobooma(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(68, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(1)
        .as_bytes()
}

fn raw_gigobooma(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(68, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(2)
        .as_bytes()
}

fn raw_savage_wolf(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(67, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_barbarous_wolf(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(67, monster.section, monster.wave_id, monster.pos, monster.dir)
        .field2(1.)
        .as_bytes()
}

fn raw_monest(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(66, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(2)
        .unknown4(7589)
        .field2(5.) // start number?
        .field3(30.) // total number?
        .as_bytes()
}

fn raw_rag_rappy(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(65, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown3(1)
        .as_bytes()
}

fn raw_sand_rappy(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(65, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown3(1)
        .as_bytes()
}

fn raw_hildebear(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(64, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_gilchic(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(128, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(1)
        .as_bytes()
}

fn raw_dubchic(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(128, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_garanz(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(129, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown3(6)
        .as_bytes()
}

fn raw_sinow_blue(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(130, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(6)
        .unknown4(6431)
        .as_bytes()
}

// TODO: invisible attr (field1: 1/0)
fn raw_sinow_gold(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(130, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(6)
        .unknown4(6431)
        .field2(1.)
        .as_bytes()
}

// TODO: zonde-ing (field1: 1/0)
fn raw_canadine(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(131, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_canane(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(132, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_dubchic_switch(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(133, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_poison_lily(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(97, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(5)
        .unknown4(6100)
        .as_bytes()
}

fn raw_del_lily(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(97, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(5)
        .unknown4(6100)
        .as_bytes()
}

fn raw_pofuilly_slime(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(97, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

// TODO: idle distance
fn raw_pal_shark(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(99, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(5)
        .unknown4(0xffff)
        .skin(1)
        .as_bytes()
}

// TODO: idle distance
fn raw_guil_shark(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(99, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(5)
        .unknown4(6167)
        .skin(2)
        .as_bytes()
}

fn raw_nano_dragon(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(98, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(5)
        .unknown4(6184)
        .as_bytes()
}

fn raw_evil_shark(monster: &Monster) -> Vec<u8> {
    let mut idle_distance = 0.;
    for attr in monster.attributes.iter() {
        match attr {
            &MonsterAttribute::IdleDistance(dist) => idle_distance = dist,
            _ => {},
        }
    }
    RawMonsterData::new(99, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(5)
        .unknown4(0xffff)
        .field2(idle_distance)
        .as_bytes()
}

fn raw_pan_arms(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(101, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_claw(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(168, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_dimenian(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(166, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown3(8)
        .unknown4(0xffff)
        .as_bytes()
}

fn raw_dark_belra(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(165, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(8)
        .unknown4(7220)
        .as_bytes()
}

fn raw_delsaber(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(160, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown3(8)
        .field2(7.)
        .as_bytes()
}

fn raw_chaos_sorcerer(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(161, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown3(8)
        .unknown4(0xffff)
        .as_bytes()
}

fn raw_bulclaw(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(167, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(2454)
        .unknown3(8)
        .unknown4(6730)
        .as_bytes()
}

fn raw_dark_gunner(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(162, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(2006)
        .unknown3(9)
        .unknown4(6136)
        .field2(1.)
        .as_bytes()
}

fn raw_chaos_bringer(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(164, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(2249)
        .unknown3(9)
        .unknown4(6403)
        .as_bytes()
}

fn raw_la_dimenean(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(166, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown3(8)
        .unknown4(0xffff)
        .skin(1)
        .as_bytes()
}

fn raw_so_dimenian(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(166, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown3(8)
        .unknown4(0xffff)
        .skin(2)
        .as_bytes()
}

fn raw_grass_assassin(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(96, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(4)
        .unknown4(6149)
        .as_bytes()
}

fn raw_dragon(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(192, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(4)
        .unknown4(6149)
        .as_bytes()
}

fn raw_de_rol_le(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(193, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(4)
        .unknown4(6149)
        .as_bytes()
}

fn raw_vol_opt_part_a(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(194, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(4)
        .unknown4(6149)
        .as_bytes()
}

fn raw_vol_opt(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(197, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(4)
        .unknown4(6149)
        .as_bytes()
}

fn raw_dark_falz(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(200, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .unknown3(4)
        .unknown4(6149)
        .as_bytes()
}

fn raw_epsilon(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(224, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_gee(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(217, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_gi_gue(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(218, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

// unknown2: 0xffff -> 10/ 0xffff/2 -> 5?
fn raw_recon_box(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(223, monster.section, monster.wave_id, monster.pos, monster.dir)
        .unknown2(0xffff)
        .as_bytes()
}

fn raw_gibbles(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(216, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_morfos(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(222, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_ill_gill(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(225, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_mericarol(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(214, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_mericus(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(214, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(2)
        .as_bytes()
}

fn raw_merikle(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(214, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(1)
        .as_bytes()
}

fn raw_merillias(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(213, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_meriltas(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(213, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(1)
        .as_bytes()
}

fn raw_sinow_beril(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(212, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_silow_spigell(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(212, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(1)
        .as_bytes()
}

fn raw_ul_gibbon(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(215, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_zol_gibbon(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(215, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(1)
        .as_bytes()
}

fn raw_dolmdarl(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(221, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_dolmolm(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(221, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(1)
        .as_bytes()
}

fn raw_delbiter(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(220, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_deldepth(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(219, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_sinow_zoa(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(224, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_sinow_zele(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(224, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(1)
        .as_bytes()
}

fn raw_gal_gryphon(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(192, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_olga_flow(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(202, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_barba_ray(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(203, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_gol_dragon(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(204, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_boota(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(277, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_ze_boota(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(277, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(1)
        .as_bytes()
}

fn raw_ba_boota(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(277, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(2)
        .as_bytes()
}

fn raw_satellite_lizard(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(273, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_yowie(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(273, monster.section, monster.wave_id, monster.pos, monster.dir)
        .field2(1.)
        .as_bytes()
}

fn raw_dorphon(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(278, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_astark(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(272, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_girtablulu(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(275, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_merissa_a(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(274, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_goran(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(279, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_goran_detonator(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(279, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(2)
        .as_bytes()
}

fn raw_pyro_goran(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(279, monster.section, monster.wave_id, monster.pos, monster.dir)
        .skin(1)
        .as_bytes()
}

fn raw_zu(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(276, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_saint_million(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(281, monster.section, monster.wave_id, monster.pos, monster.dir)
        .as_bytes()
}

fn raw_shambertin(monster: &Monster) -> Vec<u8> {
    RawMonsterData::new(281, monster.section, monster.wave_id, monster.pos, monster.dir)
        .field1(1.)
        .skin(1)
        .as_bytes()
}

