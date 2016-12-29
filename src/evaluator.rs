use types::*;







#[derive(Debug)]
pub enum SyntaxError {
    UnknownFunction(String),
    InvalidFunction(String),
    InvalidNumberOfArguments(String, u32, usize),
    InvalidArgument(String, String),
    Error,
}

fn eval_equal(quest: &mut Quest, args: &Vec<PExpr>) -> Result<(), SyntaxError> {
    println!("qqq!");
    if args.len() != 2 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("equal"), 2, args.len()));
    }
    Ok(())
}

fn eval_set_episode(quest: &mut Quest, args: &Vec<PExpr>) -> Result<(), SyntaxError> {
    if args.len() != 1 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("set-episode"), 1, args.len()));
    }
    match args[0] {
        PExpr::Integer(ep) => quest.episode = ep,
        _ => return Err(SyntaxError::InvalidArgument(String::from("set-episode"), args[0].to_string()))
    }
    Ok(())
}



pub fn eval_quest(expr: Vec<PExpr>) -> Result<Quest, SyntaxError> {
    let mut quest = Quest {
        episode: 0,
        
        on_start: PExpr::Noop,
        on_success: PExpr::Noop,
        on_failure: PExpr::Noop,
        
        objects: Vec::new(),
        //let monsters: Vec<Monster> = Vec::new();
        npcs: Vec::new(),
        waves: Vec::new(),
    };

    
    for e in expr.iter() {
        println!("z: {}", e);

        // I really shouldnt do this, but it makes me laugh
        match match e {
            &PExpr::SetEpisode(ref args) => eval_set_episode(&mut quest, &args),
            _ => return Err(SyntaxError::InvalidFunction(e.to_string()))
            //_ => return Err(SyntaxError::Error)
        } {
            Ok(z) => {}
            Err(why) => {
                println!("error {:?} in {}", why, e);
            }
        }
        

        /*match result {
            Ok(z) => {}
            Err(why) => {
                println!("error {:?} in {}", why, e);
            }
        }*/
    }

    Ok(quest)
}
