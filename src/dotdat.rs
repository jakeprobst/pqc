

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
    Bleh,
}



fn monster_type_id(monster: &Monster) -> u16 {
    //match monster.id

    0x63
}

// TODO: figure out defaults for all these unknown fields (and maybe figure them out??)
fn monster_unknown1(monster: &Monster) -> u16 {
    0
}

fn monster_unknown2(monster: &Monster) -> u32 {
    0
}

fn monster_unknown3(monster: &Monster) -> u16 {
    0
}

fn monster_unknown4(monster: &Monster) -> u16 {
    0
}

fn monster_field1(monster: &Monster) -> f32 {
    0.0
}

fn monster_field2(monster: &Monster) -> u32 {
    0
}

fn monster_field3(monster: &Monster) -> u32 {
    0
}

fn monster_field4(monster: &Monster) -> f32 {
    0.0
}

fn monster_field5(monster: &Monster) -> f32 {
    0.0
}

fn monster_skin(monster: &Monster) -> u32 {
    0
}

fn monster_field6(monster: &Monster) -> u32 {
    0
}

// TODO: what is clone count?
// TODO: make this work for npcs as well as monsters?
fn monster_raw_bytes(monster: &Monster, section: u16, wave_id: u32) -> Vec<u8> {
    let mut mdata = Vec::new();
    //mdata.write_u16::<LittleEndian>(monster_type_id(&monster));
    mdata.write_u16::<LittleEndian>(monster_type_id(&monster));
    mdata.write_u16::<LittleEndian>(monster_unknown1(&monster));
    mdata.write_u32::<LittleEndian>(monster_unknown2(&monster));
    mdata.write_u16::<LittleEndian>(monster_unknown3(&monster));
    mdata.write_u16::<LittleEndian>(monster_unknown4(&monster));
    mdata.write_u16::<LittleEndian>(section); // unknown
    mdata.write_u16::<LittleEndian>(wave_id as u16);
    mdata.write_u32::<LittleEndian>(wave_id);
    mdata.write_f32::<LittleEndian>(monster.pos.x);
    mdata.write_f32::<LittleEndian>(monster.pos.y);
    mdata.write_f32::<LittleEndian>(monster.pos.z);
    mdata.write_u32::<LittleEndian>(0); // x rotation
    mdata.write_u32::<LittleEndian>(monster.dir);
    mdata.write_u32::<LittleEndian>(0); // z rotation
    mdata.write_f32::<LittleEndian>(monster_field1(&monster));
    mdata.write_u32::<LittleEndian>(monster_field2(&monster));
    mdata.write_u32::<LittleEndian>(monster_field3(&monster));
    mdata.write_f32::<LittleEndian>(monster_field4(&monster));
    mdata.write_f32::<LittleEndian>(monster_field5(&monster));
    mdata.write_u32::<LittleEndian>(monster_skin(&monster));
    mdata.write_u32::<LittleEndian>(monster_field6(&monster));

    // assert length! (0x48)
    
    mdata
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

/*fn floor_wave_raw_bytes(waves: &Vec<Wave>) -> Result<u8, DatError> {
    Ok(Vec::new())
}*/

// TODO: this function is ugly, needs a rework
fn generate_wave_data(waves: &Vec<Wave>) -> Result<Vec<u8>, DatError> {
    //let mut floor_wave_data = BTreeMap::new();
    
    /*for wave in waves.iter() {
        let this_floor = floor_wave_data.entry(&wave.floor).or_insert(Vec::new());
        (*this_floor).append(&mut floor_wave_raw_bytes(&wave, wave.id));
    }*/

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
    
    /*let mut wave_datas = BTreeMap::new();
    let mut event_datas = BTreeMap::new();
    let mut event_offsets = BTreeMap::new();
    
    for wave in waves.iter() {
        let wave_data = wave_datas.entry(&wave.floor).or_insert(Vec::new());
        let event_data = event_datas.entry(&wave.floor).or_insert(Vec::new());
        let event_offset = event_offsets.entry(&wave.floor).or_insert(0);
        
        wave_data.append(&mut wave_raw_bytes(&wave, *event_offset));
        
        for next in wave.next.iter() {
            event_data.write_u8(0x0C);
            event_data.write_u32::<LittleEndian>(*next);
            *event_offset += 5;
        }
        for unlock in wave.unlock.iter() {
            event_data.write_u8(0x0A);
            event_data.write_u16::<LittleEndian>(*unlock);
            *event_offset += 3;
        }
        event_data.write_u8(0x01);
        *event_offset += 1;
    }

    for (floor, event_data) in &mut event_datas {
        while event_data.len() % 8 != 0 {
            event_data.write_u8(0xFF);
        }
    }

    let mut full_wave_data = Vec::new();

    for (floor, _) in wave_datas {
        let sub_wave_data = Vec::new();
        let wave_data = wave_datas.get(floor);
        let event_data = event_datas.get(floor);

        sub_wave_data.write_u32::<LittleEndian>(wave.data.len() + 16);
        sub_wave_data.write_u32::<LittleEndian>(0x00000010); // 10 00 00 00
        sub_wave_data.append(&mut wave_data);
        sub_wave_data.append(&mut event_data);
        
        full_wave_data.write_u32::<LittleEndian>(WAVE_HEADER_ID);
        full_wave_data.write_u32::<LittleEndian>(sub_wave_data.len() as u32 + 16);
        full_wave_data.write_u32::<LittleEndian>(u32::from(floor));
        full_wave_data.write_u32::<LittleEndian>(sub_wave_data.len() as u32);
    }
    

    Ok(full_wave_data)*/
}

pub fn generate_dat(quest: &Quest) -> Result<Vec<u8>, DatError> {
    let mut dat = Vec::new();

    /*let mut floor_label_ids = HashMap::new();
    for fl in quest.floors.iter() {
        floor_label_ids.insert(fl.label.clone(), fl.id.clone());
    }

    let mut wave_label_ids = HashMap::new();
    let mut wave_label_id_count = 7;
    for wave in quest.waves.iter() {
        if let Some(..) = wave_label_ids.get(&wave.label) {
            return Err(DatError::WaveNameAlreadyExists(wave.label.clone()));
            
        }
        wave_label_ids.insert(wave.label.clone(), wave_label_id_count);
        wave_label_id_count += 1;
    }*/

    //dat.append(&mut try!(generate_object_data(&quest.waves, &floors)));
    dat.append(&mut try!(generate_monster_data(&quest.waves)));
    dat.append(&mut try!(generate_wave_data(&quest.waves)));
    
    
    // 4?
    // 5?




    Ok(dat)
}
