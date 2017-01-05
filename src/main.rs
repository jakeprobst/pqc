
//use tokenize_str;
extern crate byteorder;

mod types;
mod parser;
mod evaluator;
mod dotbin;
mod dotdat;

use std::fmt;


/*fn load_file(path: &str) -> String {
    
}*/

fn main() {
    //let script = "(if (equal asd 3 qw) (set asd 4) (set asd (+ 2 3))) (npc +urmom+ (floor p2) (npc-say \"hue hue hue\"))";
    //let script = "(if (equal asd 3) (set asd 4) (set asd (+ 2 \"a string\")))";
    /*let script = "(set-floors
         [p2 pioneer2]
         [f1 forest1-1]
         [c1 caves1-3]
    [c2 caves2-2])";*/
    let script = "(set-episode 1) (set-floor c1 caves1-3) (wave a1
  (spawn evil-shark (floor c1) (pos 30 40 50))
  (spawn evil-shark (floor c1) (pos 35 45 55))
  (spawn evil-shark (floor c1) (pos 35 45 55))
  (delay 10)
  (next-wave a2))";
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

