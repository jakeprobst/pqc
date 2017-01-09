
//use tokenize_str;
extern crate byteorder;

mod types;
mod monster;
mod parser;
mod evaluator;
mod dotbin;
mod dotdat;

use std::fmt;
use std::cmp::min;


/*fn load_file(path: &str) -> String {
    
}*/

fn printhex(data: Vec<u8>) {
    for line in 0..(data.len()/16)+1 {
        print!("{:08X}  ", line*16);
        for i in 0..min(data.len() - line * 16, 16) {
            print!("{:02X} ", data[line*16+i]);
        }
        println!("");
    }
}


fn main() {
    //let script = "(if (equal asd 3 qw) (set asd 4) (set asd (+ 2 3))) (npc +urmom+ (floor p2) (npc-say \"hue hue hue\"))";
    //let script = "(if (equal asd 3) (set asd 4) (set asd (+ 2 \"a string\")))";
    /*let script = "(set-floors
         [p2 pioneer2]
         [f1 forest1-1]
         [c1 caves1-3]
    [c2 caves2-2])";*/
    let script = "\
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
";
    println!("{}", script);
        
    //let expr = try!(parser::parse_script_to_expr(script));
    /*let tokens = parser::tokenize_str(script);
    println!("tokens: {:?}", tokens);
     */

    // cant try! in main
    match parser::parse_script_to_expr(script) {
        Ok(expr) => {
            match evaluator::eval_quest(expr) {
                Ok(quest) => {
                    match dotdat::generate_dat(&quest) {
                        Ok(dat) => {
                            println!("dat: {:?}", dat);
                            printhex(dat);
                        }
                        Err(why) => {
                            println!("binerr: {:?}", why);
                        }
                    }
                    
                },
                Err(why) => {
                    println!("evalerr: {:?}", why);
                }
            }
        }
        Err(why) => {
            println!("parseerr: {:?}", why);
            
        }
    }
    
}

