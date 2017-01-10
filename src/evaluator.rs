use types::*;
use monster::*;
use object::*;
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

fn eval_generic_identifier(args: &Vec<PExpr>) -> Result<String, SyntaxError> {
    if args.len() != 1 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("generic-identifier"), 1, args.len()));
    }
    
    expect_type!(args[0], PExpr::Identifier)
}

fn eval_generic_integer(args: &Vec<PExpr>) -> Result<u32, SyntaxError> {
    if args.len() != 1 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("generic-integer"), 1, args.len()));
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

fn eval_position(args: &Vec<PExpr>) -> Result<Point, SyntaxError> {
    if args.len() != 3 {
        return Err(SyntaxError::InvalidNumberOfArguments(String::from("pos"), 3, args.len()));
    }

    let x = try!(expect_type!(args[0], PExpr::Integer));
    let y = try!(expect_type!(args[1], PExpr::Integer));
    let z = try!(expect_type!(args[2], PExpr::Integer));

    Ok(Point{x:x as f32, y:y as f32, z:z as f32})
}

struct QuestBuilder {
    // quest data
    episode: u32,
    on_success: PExpr,
    on_failure: PExpr,
    floors: Vec<FloorType>,
    objects: Vec<Object>,
    variables: Vec<Variable>,
    npcs: Vec<NPC>,
    waves: Vec<Wave>,
    
    // meta data
    floor_label_ids: HashMap<String, FloorType>,
    next_wave_label: u32,
    wave_label_ids: HashMap<String, u32>,
}

impl QuestBuilder {
    fn new() -> QuestBuilder {
        QuestBuilder {
            episode: 0,
            on_success: PExpr::Noop,
            on_failure: PExpr::Noop,
            floors: Vec::new(),
            objects: Vec::new(),
            variables: Vec::new(),
            npcs: Vec::new(),
            waves: Vec::new(),

            floor_label_ids: HashMap::new(),
            next_wave_label: 1,
            wave_label_ids: HashMap::new(),
        }
    }

    fn floor_id_from_identifier(&self, ident: String) -> Result<FloorType, SyntaxError> {
        match self.floor_label_ids.get(&ident) {
            Some(f) => Ok(f.clone()),
            None => Err(SyntaxError::UnknownFloor(ident.clone()))
        }
    }

    fn eval_set_episode(&mut self, args: &Vec<PExpr>) -> Result<(), SyntaxError> {
        self.episode = try!(eval_generic_integer(&args));
        Ok(())
    }
    
    // TODO: boolean variables?
    fn eval_variable(&mut self, args: &Vec<PExpr>) -> Result<(), SyntaxError> {
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

        self.variables.push(Variable {
            name: name,
            value: value,
        });
        
        Ok(())
    }
    
    // TODO: convert identifier to actual floortype
    fn eval_set_floor(&mut self, args: &Vec<PExpr>) -> Result<(), SyntaxError> {
        if args.len() != 2 {
            return Err(SyntaxError::InvalidNumberOfArguments(String::from("set-floor"), 2, args.len()));
        }
        
        let label = try!(expect_type!(args[0], PExpr::Identifier));
        let floor_id = try!(eval_map(&try!(expect_type!(args[1], PExpr::Map))));
        
        self.floor_label_ids.insert(label, floor_id);
        self.floors.push(floor_id);

        Ok(())
    }
    
    // TODO: be more strict about this?
    fn eval_quest_success(&mut self, args: &Vec<PExpr>) -> Result<(), SyntaxError> {
        if args.len() < 1 {
            return Err(SyntaxError::InvalidNumberOfArguments(String::from("quest-success"), 1, args.len()));
        }

        self.on_success = PExpr::Block(args.clone());
        Ok(())
    }

    
    // TODO: per-monster attributes
    fn eval_spawn(&self, args: &Vec<PExpr>, wave: u32, section: u16) -> Result<Monster, SyntaxError> {
        if args.len() < 3 {
            return Err(SyntaxError::InvalidNumberOfArguments(String::from("spawn"), 3, args.len()));
        }

        let mtype = try!(expect_type!(args[0], PExpr::Identifier));
        let id = MonsterType::from(mtype);
        
        let pos = try!(eval_position(&try!(expect_type!(args[1], PExpr::Position))));
        let dir = try!(eval_generic_integer(&try!(expect_type!(args[2], PExpr::Direction))));

        let mut attributes = Vec::new();
        for arg in args.iter().skip(3) {
            attributes.push(match arg {
                &PExpr::IdleDistance(ref attr) => {
                    match &attr[0] {
                        &PExpr::Integer(int) => MonsterAttribute::IdleDistance(int as f32),
                        &PExpr::Float(float) => MonsterAttribute::IdleDistance(float),
                        _ => return Err(SyntaxError::InvalidArgument(String::from("idle-distance"),
                                                                     arg.to_string(),
                                                                     String::from("unexpected type")))
                    }
                }
                _ => return Err(SyntaxError::InvalidArgument(String::from("spawn"),
                                                             arg.to_string(),
                                                             String::from("unknown attribute")))
            });
        }
        
        Ok(Monster {
            //id: try!(get_monster_id(&mtype)),
            id: id,
            wave_id: wave,
            section: section,
            dir:dir,
            pos: pos,
            attributes: attributes,
        })
    }


    fn eval_next_wave(&mut self, args: &Vec<PExpr>) -> Result<u32, SyntaxError> {
        if args.len() != 1 {
            return Err(SyntaxError::InvalidNumberOfArguments(String::from("next-wave"), 1, args.len()));
        }

        let next = try!(expect_type!(args[0], PExpr::Identifier));

        let possible_wave_id = self.wave_label_ids.len()+1;
        Ok(*self.wave_label_ids.entry(next).or_insert(possible_wave_id as u32))
    }


    // TODO: disallow multiple delays
    fn eval_wave(&mut self, args: &Vec<PExpr>) -> Result<(), SyntaxError> {
        if args.len() < 3 {
            return Err(SyntaxError::InvalidNumberOfArguments(String::from("wave"), 3, args.len()));
        }

        let label = try!(expect_type!(args[0], PExpr::Identifier));
        let possible_wave_id = self.wave_label_ids.len() + 1;
        let wave_id = *self.wave_label_ids.entry(label).or_insert(possible_wave_id as u32);
        
        let floor_label = try!(eval_generic_identifier(&try!(expect_type!(args[1], PExpr::Floor))));
        let floor_id = try!(self.floor_id_from_identifier(floor_label));
        
        let section = try!(eval_generic_integer(&try!(expect_type!(args[2], PExpr::Section))));
        let mut monsters = Vec::new();
        let mut next = Vec::new();
        let mut unlock = Vec::new();
        let mut delay = u32::max_value();

        for arg in args.iter().skip(3) {
            match arg {
                &PExpr::Spawn(ref args) => monsters.push(try!(self.eval_spawn(&args, wave_id, section as u16))),
                &PExpr::NextWave(ref args) => next.push(try!(self.eval_next_wave(&args))),
                &PExpr::Delay(ref args) => delay = try!(eval_generic_integer(&args)),
                &PExpr::Unlock(ref args) => unlock.push(try!(eval_generic_integer(&args)) as u16),
                _ => return Err(SyntaxError::InvalidArgument(String::from("wave"), arg.to_string(),
                                                             String::from("expected spawn, unlock, or delay")))
            }
        }

        self.waves.push(Wave {
            id: wave_id,
            floor: floor_id.clone(),
            section: section,
            monsters: monsters,
            next: next,
            unlock: unlock,
            delay: delay as u16,
        });

        Ok(())
    }
    
    fn eval_set_player_location(&mut self, args: &Vec<PExpr>) -> Result<(), SyntaxError> {
        if args.len() != 13 {
            return Err(SyntaxError::InvalidNumberOfArguments(String::from("player-set"), 13, args.len()));
        }

        let floor = try!(eval_generic_identifier(&try!(expect_type!(args[0], PExpr::Floor))));
        let floor_id = try!(self.floor_id_from_identifier(floor));

        for i in 0..4 {
            let sec = try!(eval_generic_integer(&try!(expect_type!(args[(i * 3) + 1], PExpr::Section))));
            let pos = try!(eval_position(&try!(expect_type!(args[(i * 3) + 2], PExpr::Position))));
            let dir = try!(eval_generic_integer(&try!(expect_type!(args[(i * 3) + 3], PExpr::Direction))));
            self.objects.push(Object {
                otype: ObjectType::SetPlayerLocation,
                floor_id: floor_id.clone(),
                section : sec as u16,
                pos: pos,
                dir: dir,
                attributes: vec![ObjectAttribute::Player(i as u32)],
            });
        }

        Ok(())
    }

    fn eval_object(&mut self, args: &Vec<PExpr>) -> Result<(), SyntaxError> {
        if args.len() < 3 {
            return Err(SyntaxError::InvalidNumberOfArguments(String::from("object"), 3, args.len()));
        }
        
        //let otype = try!(expect_type!(args[0], PExpr::Identifier));
        //let id = ObjectType::from(otype);

        Ok(())
    }

    fn as_quest(self) -> Quest {
        Quest {
            episode: self.episode,
            on_success: self.on_success,
            on_failure: self.on_failure,
            floors: self.floors,
            objects: self.objects,
            variables: self.variables,
            npcs: self.npcs,
            waves: self.waves,
        }
    }
}










pub fn eval_quest(expr: Vec<PExpr>) -> Result<Quest, SyntaxError> {
    /*let mut quest = Quest {
        episode: 0,
        
        on_success: PExpr::Noop,
        on_failure: PExpr::Noop,

        floors: Vec::new(),
        objects: Vec::new(),
        variables: Vec::new(),
        npcs: Vec::new(),
        waves: Vec::new(),
    };
    
    let mut floors = HashMap::new();
    let mut wave_label_ids = HashMap::new();
    
    for e in expr.iter() {
        //println!("z: {:#?}", e);

        match e {
            &PExpr::SetEpisode(ref args) => {
                //quest.episode = try!(eval_set_episode(&args));
                quest.episode = try!(eval_generic_integer(&args));
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
                let wave = try!(eval_wave(&args, &floors, &mut wave_label_ids));
                quest.waves.push(wave);
            },
            &PExpr::SetFloor(ref args) => {
                //quest.floors.push(try!(eval_set_floor(&args)));
                let (label, floor_id) = try!(eval_set_floor(&args));
                floors.insert(label, floor_id);
                quest.floors.push(floor_id);
            },
            &PExpr::Object(ref args) => {
                quest.objects.push(try!(eval_object(&args)));
            }
            _ => println!("error in {}", e)
        }
    }
    println!("quest: {:#?}", quest);

    Ok(quest);*/



    let mut qbuilder = QuestBuilder::new();

    for e in expr.iter() {
        match e {
            &PExpr::SetEpisode(ref args) => try!(qbuilder.eval_set_episode(&args)),
            &PExpr::SetPlayerLocation(ref args) => try!(qbuilder.eval_set_player_location(&args)),
            &PExpr::QuestSuccess(ref args) => try!(qbuilder.eval_quest_success(&args)),
            &PExpr::Variable(ref args) => try!(qbuilder.eval_variable(&args)),
            &PExpr::Wave(ref args) => try!(qbuilder.eval_wave(&args)),
            &PExpr::SetFloor(ref args) => try!(qbuilder.eval_set_floor(&args)),
            &PExpr::Object(ref args) => try!(qbuilder.eval_object(&args)),
            _ => println!("error in {}", e)
        }
    }

    Ok(qbuilder.as_quest())
}
