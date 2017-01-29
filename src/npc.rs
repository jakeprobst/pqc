use types::*;

use byteorder::{LittleEndian, WriteBytesExt};
use std::convert::From;

pub enum NpcError{
    NoSuchNpc(String),
    
        
}


// NPC names from scht data, possibly rename to be more informative
#[derive(Debug, Clone)]
pub enum NpcType {
    None,
    Npc01,
    Npc02,
    Npc03,
    Npc04,
    Npc05,
    Npc06,
    Npc07,
    Npc08,
    Npc09,
    Npc0A,
    Npc0B,
    Npc0C,
    Npc0D,
    Npc0E,
    Npc19Military,
    Npc1AMilitary,
    PrincipalTyrel,
    Tekker,
    Banklady,
    Npc1EScientist,
    Nurse,
    Irene,
    Broomop,
    Npc22Hunter,
    Npc24Ranger,
    Npc25Cast,
    Npc26Caseal,
    Npc27FOmarl,
    Npc28FOnewm,
    Npc29FOnewearl,
    Npc2BHUnewearl,
    Npc2CCast,
    Npc2DRAmar,
    Npc30FOmarl,
    Hopkins,
    Npc32FOnewearl,
    Ep2NpcD0,
    Ep2Directrice,
    Ep2NpcD2Millitary,
    Ep2NpcD3,
    Ep2NpcF0Military,
    Ep2NpcF1,
    Ep2NpcF2,
    Ep2Schthack,
    Ep2Leo,
    Ep2Pagini,
    Ep2NpcF6,
    Ep2Nol,
    Ep2Elly,
    Ep2NPCF9,
    Ep2NPCFA,
    Ep2NPCFB,
    Ep2NPCFCMilitary,
    Ep2NPCFD,
    Ep2Banklady,
    Ep2NPCFF,
    Momoka,
}


impl From<String> for NpcType {
    fn from(id: String) -> NpcType {
        match id.as_ref() {
            "npc1" => NpcType::Npc01,
            "npc2" => NpcType::Npc02,
            "npc3" => NpcType::Npc03,
            "npc-fonewm" => NpcType::Npc28FOnewm,
            "npc-hopkins" => NpcType::Hopkins,
            _ => NpcType::None,
        }
    }
}

impl From<NpcType> for u16 {
    fn from(npc_type: NpcType) -> u16 {
        match npc_type {
            NpcType::Npc28FOnewm => 0x28,
            NpcType::Hopkins => 0x31,
            _ => 0xff,
        }
    }
}


#[derive(Debug)]
pub struct Npc {
    pub skin: NpcType,
    pub floor: FloorType,
    pub section: u16,
    pub pos: Point,
    pub dir: u32,
    pub move_flag: u32,
    pub move_distance: f32,
    pub hide_register: f32,
    pub character_id: f32,
    pub function: Function,
}


// TODO: do these zeroes need to be variables of some sort?
//impl<'a> From<&'a Npc> for Vec<u8> {
//fn from(npc: &'a Npc) -> Vec<u8> {
impl Npc {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut ndata = Vec::new();
        ndata.write_u16::<LittleEndian>(u16::from(self.skin.clone()));
        ndata.write_u16::<LittleEndian>(0);
        ndata.write_u32::<LittleEndian>(0);
        ndata.write_u16::<LittleEndian>(0);
        ndata.write_u16::<LittleEndian>(0);
        ndata.write_u16::<LittleEndian>(self.section);
        ndata.write_u16::<LittleEndian>(0);
        ndata.write_u32::<LittleEndian>(0);
        ndata.write_f32::<LittleEndian>(self.pos.x);
        ndata.write_f32::<LittleEndian>(self.pos.y);
        ndata.write_f32::<LittleEndian>(self.pos.z);
        ndata.write_u32::<LittleEndian>(0);
        ndata.write_u32::<LittleEndian>(self.dir);
        ndata.write_u32::<LittleEndian>(0);
        ndata.write_f32::<LittleEndian>(self.move_distance);
        ndata.write_f32::<LittleEndian>(0.0);
        ndata.write_f32::<LittleEndian>(self.hide_register);
        ndata.write_f32::<LittleEndian>(self.character_id);
        ndata.write_f32::<LittleEndian>(if let Function::Id(func) = self.function {
            func
        } else {
            0. // TODO: make this case error out instead? should this even be reachable?
        });
        ndata.write_u32::<LittleEndian>(self.move_flag);
        ndata.write_u32::<LittleEndian>(0);
        ndata
    }
}
