use types::*;

use byteorder::{LittleEndian, WriteBytesExt};
use std::convert::From;

pub enum MonsterError {
    NoSuchMonster(String)
}

#[derive(Debug, Clone)]
pub enum MonsterType {
    None,
    Booma,
    Gibooma,
    Gigobooma,

    EvilShark,
    PalShark,
    GuilShark,
    NanoDragon,
    GrassAssassin,
        
    // etc
}

impl From<String> for MonsterType {
    fn from(id: String) -> MonsterType {
        match id.as_ref() {
            "evil-shark" => MonsterType::EvilShark,
            "pal-shark" => MonsterType::PalShark,
            "guil-shark" => MonsterType::GuilShark,
            "nano-dragon" => MonsterType::NanoDragon,
            "grass-assassin" => MonsterType::GrassAssassin,

            _ => MonsterType::None
        }
    }
}




#[derive(Debug)]
pub struct Monster {
    pub id: MonsterType,
    //pub floor: String,
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
