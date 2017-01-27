
use types::*;
use assembler::*;

#[derive(Debug)]
pub enum BinError {
    Bleh,
}


// TODO: handle header types other than v3?
pub fn generate_bin(mut quest: &mut Quest) -> Result<Vec<u8>, BinError> {
    //TODO: header, function table

    let mut gencode = Assembler::new(&mut quest);

    // gencode.as_bytecode()
    // gencode.as_label_table() // label tabel, lable table?
    Ok(gencode.as_bytes())
}

















