
//use tokenize_str;

mod types;
mod parser;
mod evaluator;



fn main() {
    let script = "(if (equal asd 3 qw) (set asd 4) (set asd (+ 2 3))) (npc +urmom+ (floor p2) (npc-say \"hue hue hue\"))";
    //let script = "(if (equal asd 3) (set asd 4) (set asd (+ 2 \"a string\")))";
    /*let script = "(set-floors
         [p2 pioneer2]
         [f1 forest1-1]
         [c1 caves1-3]
         [c2 caves2-2])";*/
    println!("{}", script);
        
    //let expr = try!(parser::parse_script_to_expr(script));
    /*let tokens = parser::tokenize_str(script);
    println!("tokens: {:?}", tokens);
     */
    match parser::parse_script_to_expr(script) {
        Ok(expr) => {
            match evaluator::eval_quest(expr) {
                Ok(quest) => {},
                Err(why) => {
                    println!("err: {:?}", why);
                }
            }
        }
        Err(why) => {
            println!("err: {:?}", why);
            
        }
    }
    
}

