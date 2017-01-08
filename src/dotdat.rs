

use std::collections::{BTreeMap, HashMap};
use byteorder::{LittleEndian, WriteBytesExt};

use types::*;
use monster::*;
use evaluator::*;


const MONSTER_HEADER_ID: u32 = 2;
const WAVE_HEADER_ID: u32 = 3;


#[derive(Debug)]
pub enum DatError {
    UndefinedFloor(String),
    UndefinedWave(String),
    WaveNameAlreadyExists(String),
}

fn monster_wave_raw_bytes(wave: &Wave, wave_id: u32) -> Vec<u8> {
    let mut wdata = Vec::new();

    for monster in &wave.monsters {
        wdata.append(&mut Vec::<u8>::from(monster));
    }

    wdata
}


// TODO: rename because npcs might need to also be parsed here?
// TODO: requires a lot of data to be filled in from the .rel files (or just hardcode!)
fn generate_monster_data(waves: &Vec<Wave>) -> Result<Vec<u8>, DatError> {
    // map for each area to wave
    let mut floor_monster_data = BTreeMap::new();
    
    for wave in waves.iter() {
        let this_floor = floor_monster_data.entry(&wave.floor).or_insert(Vec::new());
        (*this_floor).append(&mut monster_wave_raw_bytes(&wave, wave.id));
    }

    let mut monster_data = Vec::new();
    
    for (floor, monsters) in floor_monster_data {
        monster_data.write_u32::<LittleEndian>(MONSTER_HEADER_ID);
        monster_data.write_u32::<LittleEndian>(monsters.len() as u32 + 16);
        monster_data.write_u32::<LittleEndian>(u32::from(floor));
        monster_data.write_u32::<LittleEndian>(monsters.len() as u32);

        monster_data.append(&mut monsters.clone());
    }

    Ok(monster_data)
}


fn wave_raw_bytes(wave: &Wave, event_offset: u32) -> Vec<u8> {
    let mut dat = Vec::new();
    dat.write_u32::<LittleEndian>(wave.id);
    dat.write_u32::<LittleEndian>(0x00010000); // 00 00 01 00
    dat.write_u16::<LittleEndian>(wave.section as u16);
    dat.write_u16::<LittleEndian>(wave.id as u16); // this might not be correct!
    dat.write_u16::<LittleEndian>(wave.delay);
    dat.write_u16::<LittleEndian>(0); // wavesetting in qedit TODO: figure this out
    dat.write_u32::<LittleEndian>(event_offset);

    dat
}

fn generate_wave_data(waves: &Vec<Wave>) -> Result<Vec<u8>, DatError> {
    let mut wave_floors = BTreeMap::new();
    for wave in waves.iter() {
        let wave_list = wave_floors.entry(&wave.floor).or_insert(Vec::new());
        wave_list.push(wave);
    }

    let mut full_wave_data = Vec::new();

    for (floor, wave_list) in wave_floors.iter() {
        let mut wave_data = Vec::new();
        let mut event_data = Vec::new();
        let mut event_offset = 0;

        for wave in wave_list.iter() {
            wave_data.append(&mut wave_raw_bytes(&wave, event_offset));
            
            for next in wave.next.iter() {
                event_data.write_u8(0x0C);
                event_data.write_u32::<LittleEndian>(*next);
                event_offset += 5;
            }
            for unlock in wave.unlock.iter() {
                event_data.write_u8(0x0A);
                event_data.write_u16::<LittleEndian>(*unlock);
                event_offset += 3;
            }
            event_data.write_u8(0x01);
            event_offset += 1;
        }

        while event_data.len() % 8 != 0 {
            event_data.write_u8(0xFF);
        }
        
        let mut sub_wave_data = Vec::new();
        sub_wave_data.write_u32::<LittleEndian>((wave_data.len() + 16) as u32);
        sub_wave_data.write_u32::<LittleEndian>(0x00000010); // 10 00 00 00
        sub_wave_data.write_u32::<LittleEndian>(wave_list.len() as u32);
        sub_wave_data.write_u32::<LittleEndian>(0);
        sub_wave_data.append(&mut wave_data);
        sub_wave_data.append(&mut event_data);
        
        full_wave_data.write_u32::<LittleEndian>(WAVE_HEADER_ID);
        full_wave_data.write_u32::<LittleEndian>(sub_wave_data.len() as u32 + 16);
        full_wave_data.write_u32::<LittleEndian>(u32::from(*floor));
        full_wave_data.write_u32::<LittleEndian>(sub_wave_data.len() as u32);
        full_wave_data.append(&mut sub_wave_data);
    }

    Ok(full_wave_data)
}

pub fn generate_dat(quest: &Quest) -> Result<Vec<u8>, DatError> {
    let mut dat = Vec::new();

    //dat.append(&mut try!(generate_object_data(&quest.waves, &floors)));
    dat.append(&mut try!(generate_monster_data(&quest.waves)));
    dat.append(&mut try!(generate_wave_data(&quest.waves)));
    // 4?
    // 5?




    Ok(dat)
}
