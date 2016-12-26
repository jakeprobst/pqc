
//use tokenize_str;

mod types;
mod parser;



fn main() {
    let script = "(if (eq asd 3) (set asd 4) (thing asdf (+ 2 3) plus)) (npc +urmom+ (floor p2) (npc-say \"huehuehue\"))";
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

