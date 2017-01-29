use encoding::{Encoding, EncoderTrap};
use encoding::all::UTF_16LE;
use byteorder::{LittleEndian, WriteBytesExt};

use types::*;
use assembler::*;


#[derive(Debug)]
pub enum BinError {
    Bleh,
    Assembler(AssemblerError),
    
}


// TODO: handle header types other than v3?
pub fn generate_bin(mut quest: &mut Quest) -> Result<Vec<u8>, BinError> {
    //TODO: header, function table
    let mut bin = Vec::new();
    
    let mut gencode = Assembler::new(&mut quest);

    let (mut opcodes, mut func_table) = gencode.as_bytes();

    bin.write_u32::<LittleEndian>(0x394); // header size
    bin.write_u32::<LittleEndian>((0x394+opcodes.len()) as u32);
    bin.write_u32::<LittleEndian>((0x394+opcodes.len()+func_table.len()) as u32);
    bin.write_u32::<LittleEndian>(0xffffffff);
    bin.write_u16::<LittleEndian>(0); // TODO: language
    bin.write_u16::<LittleEndian>(999); // TODO: quest_number

    let mut quest_name = UTF_16LE.encode(quest.quest_name.as_str(), EncoderTrap::Ignore).unwrap();
    while quest_name.len() < 64 {
        quest_name.push(0);
    }
    bin.append(&mut quest_name);

    let mut quest_desc = UTF_16LE.encode(quest.quest_description.as_str(), EncoderTrap::Ignore).unwrap();
    while quest_desc.len() < 256 {
        quest_desc.push(0);
    }
    bin.append(&mut quest_desc);
    
    let mut quest_desc_long = UTF_16LE.encode(quest.quest_description_long.as_str(), EncoderTrap::Ignore).unwrap();
    while quest_desc_long.len() < 576 {
        quest_desc_long.push(0);
    }
    bin.append(&mut quest_desc_long);

    //bin.write_u32::<LittleEndian>(0); // padding?

    bin.append(&mut opcodes);
    bin.append(&mut func_table);
    
    
    Ok(bin)
}

















