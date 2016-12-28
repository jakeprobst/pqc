
//use tokenize_str;

mod types;
mod parser;



fn main() {
    //let script = "(if (equal asd 3) (set asd 4) (set asd (+ 2 3))) (npc +urmom+ (floor p2) (npc-say \"huehuehue\"))";
    //let script = "(if (equal asd 3) (set asd 4) (set asd (+ 2 \"a string\")))";
    /*let script = "(set-floors
         [p2 pioneer2]
         [f1 forest1-1]
         [c1 caves1-3]
         [c2 caves2-2])";*/
    println!("{}", script);
        

    let tokens = parser::tokenize_str(script);
    println!("tokens: {:?}", tokens);
    match parser::eval_tokenized_expr(tokens) {
        Ok(expr) => {
            println!("expr: {:?}", expr);
        }
        Err(why) => {
            println!("err: {:?}", why);
            
        }
    }
}

