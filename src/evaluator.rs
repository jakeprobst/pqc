use types::*;
use monster::*;
use std::fmt::Write;
use std::collections::HashMap;

#[derive(Debug)]
pub enum SyntaxError {
    UnknownFunction(String),
    InvalidFunction(String),
    InvalidNumberOfArguments(String, u32, usize),
    InvalidArgument(String, String, String),
    UnknownMonster(String),
    UnknownFloor(String),
    WaveAlreadyDefined(String),
}

// TODO: replace module_path with function_path once it exists
macro_rules! expect_type {
    ($arg:expr, $t:path) => {
        match $arg {
            $t(ref var) => Ok(var.clone()),
            _ => {
                Err(SyntaxError::InvalidArgument(String::from(module_path!()),
                                                        $arg.to_string(),
                                                        String::from("expected different type")))
            }
        }
    }
}



// TODO: boolean variables?
fn eval_variable(args: &Vec<PExpr>) -> Result<Variable, SyntaxError> {
    if args.len() < 1 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("variable"), 1, args.len()));
    }
    
    let name = try!(expect_type!(args[0], PExpr::Identifier));
    let value = if args.len() == 2 {
        match &args[1] {
            &PExpr::Integer(ref v) => VariableValue::Integer(v.clone()),
            &PExpr::Float(ref v) => VariableValue::Float(v.clone()),
            &PExpr::StringLiteral(ref v) => VariableValue::String(v.clone()),
            _ => return Err(SyntaxError::InvalidArgument(String::from("variable"), args[1].to_string(), String::from("invalid type")))
        }
    }
    else {
        VariableValue::None
    };

    Ok(Variable {
        name: name,
        value: value,
    })
}

fn eval_set_episode(args: &Vec<PExpr>) -> Result<u32, SyntaxError> {
    if args.len() != 1 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("set-episode"), 1, args.len()));
    }
    
    expect_type!(args[0], PExpr::Integer)
}

fn eval_map(args: &Vec<PExpr>) -> Result<FloorType, SyntaxError> {
    if args.len() != 3 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("map"), 3, args.len()));
    }

    let area = try!(expect_type!(args[0], PExpr::Identifier));
    let subarea = try!(expect_type!(args[1], PExpr::Integer));
    let layout = try!(expect_type!(args[2], PExpr::Integer));
    
    Ok(FloorType::new(area, subarea, layout))
}

// TODO: convert identifier to actual floortype
fn eval_set_floor(args: &Vec<PExpr>) -> Result<(String, FloorType), SyntaxError> {
    if args.len() != 2 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("set-floor"), 2, args.len()));
    }
    
    let label = try!(expect_type!(args[0], PExpr::Identifier));
    let map = try!(eval_map(&try!(expect_type!(args[1], PExpr::Map))));

    Ok((label, map))
}

// TODO: be more strict about this?
fn eval_quest_success(args: &Vec<PExpr>) -> Result<PExpr, SyntaxError> {
    if args.len() < 1 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("quest-success"), 1, args.len()));
    }

    Ok(PExpr::Block(args.clone()))
}

fn eval_floor(args: &Vec<PExpr>) -> Result<String, SyntaxError> {
    if args.len() != 1 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("floor"), 1, args.len()));
    }

    expect_type!(args[0], PExpr::Identifier)
}

fn eval_position(args: &Vec<PExpr>) -> Result<Point, SyntaxError> {
    if args.len() != 3 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("pos"), 3, args.len()));
    }

    let x = try!(expect_type!(args[0], PExpr::Integer));
    let y = try!(expect_type!(args[1], PExpr::Integer));
    let z = try!(expect_type!(args[2], PExpr::Integer));

    Ok(Point{x:x as f32, y:y as f32, z:z as f32})
}

fn eval_direction(args: &Vec<PExpr>) -> Result<u32, SyntaxError> {
    if args.len() != 1 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("dir"), 1, args.len()));
    }

    expect_type!(args[0], PExpr::Integer)
}


fn eval_section(args: &Vec<PExpr>) -> Result<u32, SyntaxError> {
    if args.len() != 1 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("section"), 1, args.len()));
    }

    expect_type!(args[0], PExpr::Integer)
}

// TODO: type -> enum
// TODO: per-monster attributes
fn eval_spawn(args: &Vec<PExpr>, wave: u32, section: u16) -> Result<Monster, SyntaxError> {
    if args.len() < 3 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("spawn"), 3, args.len()));
    }

    let mtype = try!(expect_type!(args[0], PExpr::Identifier));
    let id = MonsterType::from(mtype);
    
    let pos = try!(eval_position(&try!(expect_type!(args[1], PExpr::Position))));
    let dir = try!(eval_direction(&try!(expect_type!(args[2], PExpr::Direction))));

    Ok(Monster {
        //id: try!(get_monster_id(&mtype)),
        id: id,
        wave_id: wave,
        section: section,
        dir:dir,
        pos: pos,
    })
}

fn eval_next_wave(args: &Vec<PExpr>, wave_label_ids: &mut HashMap<String, u32>) -> Result<u32, SyntaxError> {
    if args.len() != 1 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("next-wave"), 1, args.len()));
    }

    let next = try!(expect_type!(args[0], PExpr::Identifier));

    let possible_wave_id = wave_label_ids.len()+1;
    Ok(*wave_label_ids.entry(next).or_insert(possible_wave_id as u32))
}

fn eval_delay(args: &Vec<PExpr>) -> Result<u32, SyntaxError> {
    if args.len() != 1 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("delay"), 1, args.len()));
    }

    expect_type!(args[0], PExpr::Integer)
}


fn eval_unlock(args: &Vec<PExpr>) -> Result<u32, SyntaxError> {
    if args.len() != 1 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("unlock"), 1, args.len()));
    }

    expect_type!(args[0], PExpr::Integer)
}

// TODO: disallow multiple delays
fn eval_wave(args: &Vec<PExpr>, floors: &HashMap<String, FloorType>,
             wave_label_ids: &mut HashMap<String, u32>) -> Result<Wave, SyntaxError> {
    if args.len() < 3 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("wave"), 3, args.len()));
    }

    let label = try!(expect_type!(args[0], PExpr::Identifier));
    /*if wave_label_ids.contains_key(&label) {
        return Err(SyntaxError::WaveAlreadyDefined(label.clone()));
    }
    let wave_id = wave_label_ids.len() as u32;
    wave_label_ids.insert(label.clone(), wave_id);*/
    let possible_wave_id = wave_label_ids.len() + 1;
    let wave_id = *wave_label_ids.entry(label).or_insert(possible_wave_id as u32);
    
    let floor_label = try!(eval_floor(&try!(expect_type!(args[1], PExpr::Floor))));
    let floor = match floors.get(&floor_label) {
        Some(f) => f,
        None => return Err(SyntaxError::UnknownFloor(floor_label.clone()))
    };
    
    let section = try!(eval_section(&try!(expect_type!(args[2], PExpr::Section))));
    let mut monsters = Vec::new();
    let mut next = Vec::new();
    let mut unlock = Vec::new();
    let mut delay = u32::max_value();

    for arg in args.iter().skip(3) {
        match arg {
            &PExpr::Spawn(ref args) => monsters.push(try!(eval_spawn(&args, wave_id, section as u16))),
            &PExpr::NextWave(ref args) => next.push(try!(eval_next_wave(&args, wave_label_ids))),
            &PExpr::Delay(ref args) => delay = try!(eval_delay(&args)),
            &PExpr::Unlock(ref args) => unlock.push(try!(eval_unlock(&args)) as u16),
            _ => return Err(SyntaxError::InvalidArgument(String::from("wave"), arg.to_string(),
                                                     String::from("expected spawn, unlock, or delay")))
        }
    }

    Ok(Wave {
        //label: label,
        id: wave_id,
        floor: floor.clone(),
        section: section,
        monsters: monsters,
        next: next,
        unlock: unlock,
        delay: delay as u16,
    })
}

// TODO: fill out object
fn eval_set_player_location(args: &Vec<PExpr>) -> Result<Vec<Object>, SyntaxError> {
    if args.len() != 9 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("player-set"), 9, args.len()));
    }

    let floor = try!(eval_floor(&try!(expect_type!(args[0], PExpr::Floor))));

    let mut players = Vec::new();
    for i in 0..4 {
        let pos = try!(eval_position(&try!(expect_type!(args[(i * 2) + 1], PExpr::Position))));
        let dir = try!(eval_direction(&try!(expect_type!(args[(i * 2) + 2], PExpr::Direction))));
        players.push(Object {
            otype: ObjectType::SetPlayerLocation(i as u32),
            floor: floor.clone(),
            pos: pos,
            dir: dir,
        });
    }

    return Ok(players);
}

pub fn eval_quest(expr: Vec<PExpr>) -> Result<Quest, SyntaxError> {
    let mut quest = Quest {
        episode: 0,
        
        on_success: PExpr::Noop,
        on_failure: PExpr::Noop,
        
        objects: Vec::new(),
        floors: HashMap::new(),
        //let monsters: Vec<Monster> = Vec::new();
        variables: Vec::new(),
        npcs: Vec::new(),
        waves: Vec::new(),
    };

    let mut wave_label_ids = HashMap::new();
    
    for e in expr.iter() {
        println!("z: {:#?}", e);

        match e {
            &PExpr::SetEpisode(ref args) => {
                quest.episode = try!(eval_set_episode(&args));
            },
            &PExpr::SetPlayerLocation(ref args) => {
                quest.objects.append(&mut try!(eval_set_player_location(&args)));
            }
            &PExpr::QuestSuccess(ref args) => {
                quest.on_success = try!(eval_quest_success(&args));
            }
            &PExpr::Variable(ref args) => {
                quest.variables.push(try!(eval_variable(&args)));
            }
            &PExpr::Wave(ref args) => {
                let wave = try!(eval_wave(&args, &quest.floors, &mut wave_label_ids));
                quest.waves.push(wave);
            },
            &PExpr::SetFloor(ref args) => {
                //quest.floors.push(try!(eval_set_floor(&args)));
                let (label, floor_id) = try!(eval_set_floor(&args));
                quest.floors.insert(label, floor_id);
            },
            _ => println!("error in {}", e)
        }
        

        /*match result {
            Ok(z) => {}
            Err(why) => {
                println!("error {:?} in {}", why, e);
            }
        }*/
    }
    println!("quest: {:#?}", quest);

    Ok(quest)
}
