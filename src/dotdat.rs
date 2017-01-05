

use std::collections::BTreeMap;
use byteorder::{LittleEndian, WriteBytesExt};

use types::*;
use evaluator::*;


#[derive(Debug)]
pub enum DatError {
    Bleh,
}



fn monster_raw_bytes() -> Vec<u8> {
    let mut monster = Vec::new();
    //monster.write_u32::<LittleEndian>();

    monster
}

// TODO: requires a lot of data to be filled in from the .rel files (or just hardcode!)
fn generate_monster_data(quest: &Quest) -> Result<Vec<u8>, DatError> {


    Ok(monster_raw_bytes())
}



pub fn generate_dat(quest: &Quest) -> Result<Vec<u8>, DatError> {
    let mut dat = Vec::new();

    let mut floors = BTreeMap::new();
    for fl in quest.floors.iter() {
        floors.insert(fl.label.clone(), fl.floor.clone());
    }


    
    
    // objects
    
    // npcs/monsters
    //dat.append(try!(generate_npc_data(&quest)));
    dat.append(&mut try!(generate_monster_data(&quest)));
    
    // waves
    
    // 4?
    // 5?




    Ok(dat)
}
