
//use tokenize_str;
extern crate byteorder;
extern crate encoding;

mod types;
mod monster;
mod parser;
mod npc;
mod object;
mod evaluator;
mod assembler;
mod dotbin;
mod dotdat;
mod opcode;
mod prs;

use std::fmt;
use std::cmp::min;
use std::env;
use std::io::prelude::*;
use std::fs::File;

/*fn load_file(path: &str) -> String {
    
}*/

fn printhex(data: &Vec<u8>) {
    for line in 0..(data.len()/16)+1 {
        print!("{:08X}  ", line*16);
        for i in 0..min(data.len() - line * 16, 16) {
            print!("{:02X} ", data[line*16+i]);
        }
        println!("");
    }
}

fn load_file(path: &str) -> String {
    println!("path: {}", path);
    let mut f = File::open(path).unwrap();
    let mut s = String::new();

    f.read_to_string(&mut s);

    return s;
}

fn main() {

    //let script = "(if (equal asd 3 qw) (set asd 4) (set asd (+ 2 3))) (npc +urmom+ (floor p2) (npc-say \"hue hue hue\"))";
    //let script = "(if (equal asd 3) (set asd 4) (set asd (+ 2 \"a string\")))";
    /*let script = "(set-floors
         [p2 pioneer2]
         [f1 forest1-1]
         [c1 caves1-3]
    [c2 caves2-2])";*/
/*    let script = "\
(set-episode 1)
(set-floor c1 (map caves 1 3)) 
(wave a1 (floor c1) (section 12)
  (spawn evil-shark (pos 30 40 50) (dir 90))
  (spawn evil-shark (pos 35 45 55) (dir 180) (idle-distance 10))
  (spawn evil-shark (pos 35 45 55) (dir 270) (idle-distance 20))
  (delay 10)
  (next-wave a2))
(wave a2 (floor c1) (section 12)
  (spawn pal-shark (pos 10 11 12) (dir 90))
  (spawn guil-shark (pos 13 14 15) (dir 180))
  (spawn nano-dragon (pos 16 17 18) (dir 270))
  (spawn grass-assassin (pos 19 20 21) (dir 0))
  (delay 30))
    ";*/
    let script = load_file(env::args().nth(1).unwrap().as_str());
    println!("script: {}", script);
        
    //let expr = try!(parser::parse_script_to_expr(script));
    /*let tokens = parser::tokenize_str(script);
    println!("tokens: {:?}", tokens);
     */

    // cant try! in main
    match parser::parse_script_to_expr(script.as_str()) {
        Ok(expr) => {
            match evaluator::eval_quest(expr) {
                Ok(mut quest) => {
                    println!("quest: {:#?}", quest);
                    match dotbin::generate_bin(&mut quest) {
                        Ok(bin) => {
                            printhex(&bin);
                            let mut f = File::create(env::args().nth(1).unwrap() +  &".bin").unwrap();
                            f.write_all(&bin);

                            let prsbin = prs::compress(&bin);
                            let mut fb = File::create(env::args().nth(1).unwrap() +  &".prs.bin").unwrap();
                            fb.write_all(&prsbin);
                        }
                        Err(why) => {
                            println!("bin err: {:?}", why);
                        }
                    }
                    println!("");
                    //println!("{:#?}", quest.npcs);
                    match dotdat::generate_dat(&quest) {
                        Ok(dat) => {
                            printhex(&dat);
                            let mut f = File::create(env::args().nth(1).unwrap() +  &".dat").unwrap();
                            f.write_all(&dat);
                            
                            let prsdat = prs::compress(&dat);
                            let mut fb = File::create(env::args().nth(1).unwrap() +  &".prs.dat").unwrap();
                            fb.write_all(&prsdat);
                        }
                        Err(why) => {
                            println!("bin err: {:?}", why);
                        }
                    }
                    
                },
                Err(why) => {
                    println!("eval err: {:?}", why);
                }
            }
        }
        Err(why) => {
            println!("parser err: {:?}", why);
            
        }
    }
    
}

