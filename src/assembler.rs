use encoding::{Encoding, EncoderTrap};
use encoding::all::UTF_16LE;
use std::collections::{HashMap, BTreeMap};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

use types::*;
use npc::*;
use opcode::*;
use parser::PExpr;

#[derive(Debug)]
pub enum AssemblerError {
    InvalidArgument(String, String, String),
}

macro_rules! expect_type {
    ($arg:expr, $t:path) => {
        match $arg {
            $t(ref var) => Ok(var.clone()),
            _ => {
                Err(AssemblerError::InvalidArgument(String::from(format!("{}:{}", module_path!(), line!())),
                                                        $arg.to_string(),
                                                        String::from("expected different type")))
            }
        }
    }
}

#[derive(Debug)]
struct RegisterPool {
   registers: Vec<Register>,
}

impl RegisterPool {
    fn new() -> RegisterPool {
        RegisterPool {
            // a handful of registers are reserved
            // see http://qedit.info/index.php?title=Reservedregisters
            registers: (1..256).rev().map(|r| r as Register).collect()
        }
    }

    fn take(&mut self) -> Register {
        self.registers.pop().unwrap() // oh god if we use more than 256 registers at once...
   }

    fn release(&mut self, reg: Register) {
        self.registers.push(reg)
    }
}


enum ConditionalOperand {
    Register(Register),
    ImmediateS(i32),
}

#[derive(Debug)]
pub struct Assembler {
    next_label_id: u16,
    opcodes: Vec<OpCode>,
    register_pool: RegisterPool,
    named_registers: HashMap<String, Register>,
    npc_labels: HashMap<String, u16>,
    npc_ids: HashMap<String, f32>,
    func_labels: HashMap<String, u16>,
        
}

impl Assembler {
    pub fn new(mut quest: &mut Quest) -> Assembler {
        let mut assembler = Assembler {
            next_label_id: 2,
            opcodes: Vec::new(),
            register_pool: RegisterPool::new(),
            named_registers: HashMap::new(),
            npc_labels: HashMap::new(),
            npc_ids: HashMap::new(),
            func_labels: HashMap::new(),
        };

        // precompile_* functions?
        for (label, npc) in quest.npcs.iter() {
            let nid = assembler.get_label();
            assembler.npc_labels.insert(label.clone(), nid);
            assembler.npc_ids.insert(label.clone(), npc.character_id);
        }

        assembler.set_variables(&quest.variables);
        assembler.assemble_0(&quest);
        //for func in quest.functions.iter() {
        //    assembler.assemble_function(func, &quest)
        //}
        //assembler.assemble_functions(&mut quest);
        
        assembler.assemble_npcs(&mut quest.npcs);

        assembler
    }

    fn get_label(&mut self) -> u16 {
        self.next_label_id += 1;
        self.next_label_id
    }

    fn set_variables(&mut self, vars: &Vec<Variable>) {
        for var in vars.iter() {
            self.named_registers.insert(var.name.clone(), self.register_pool.take());
        }
    }
    

    fn assemble_0(&mut self, quest: &Quest) {
        //code.push(OpCode::SetEpisode(quest.episode));
        self.opcodes.push(OpCode::Label(0));
        self.opcodes.push(OpCode::SetEpisode(quest.episode));

        // TODO: quest success
        // TODO: set_floor_handler
        // TODO: map_designate
        // TODO: set variable defaults
        
        self.opcodes.push(OpCode::Ret);
        
        self.opcodes.push(OpCode::Label(1));
        self.opcodes.push(OpCode::Ret);
    }

    
    fn assemble_block(&mut self, expr: &Vec<PExpr>) -> Vec<OpCode> {
        let mut pasm = Vec::new();
        for e in expr.iter() {
            pasm.append(&mut self.assemble_expr(&e));
        }

        pasm
    }


    /*fn assemble_conditional_operand(&mut self, expr: &PExpr) -> (ConditionalOperand, Option<Vec<OpCode>>) {
        match expr {
            &PExpr::Identifier(var) =>
                (ConditionalOperand::Register(self.named_registers.get(var).unwrap()), None),
            &PExpr::Integer(val) =>
                (ConditionalOperand::ImmediateS(val), None),
            &PExpr::Boolean(val) =>
                (ConditionalOperand::ImmediateS(val as i32), None),
            &PExpr::Float(val) =>
                (ConditionalOperand::ImmediateF(val), None)
            &PExpr::GetDifficulty(ref args) => {
                let mut code = Vec::new();
                let reg = self.register_pool.take();
                code.push(OpCode::GetDifficultyLevel2(reg));
                (ConditionalOperand::Register(), code)
            }
                
            _ => panic!("no match")
        }

        
    }*/

    fn assemble_conditional(&mut self, expr: &PExpr, label: u16) -> Vec<OpCode> {
        //let (val1, code1) = self.assemble_conditional_operand(&args[0]);
        //let (val2, code2) = self.assemble_conditional_operand(&args[1]);

        /*let result = Vec::new();
        
        match expr {
            // PExpr::Identifier(ref arg)
            &PExpr::Equal(ref args) => {
                let (val1, code1) = self.assemble_conditional_operand(&args[0]);
                let (val2, code2) = self.assemble_conditional_operand(&args[1]);

                match (val1, val2) {
                    (ConditionalOperand::Register(reg1), ConditionalOperand::Register(reg2)) => {
                        vec![OpCode::JmpEq(reg1, reg2, label)]
                    }
                    
                    _ => panic!("no match")
                }
            }
            _ => panic!("no match")
        }*/
        
        
        match expr {
            &PExpr::Equal(ref args) => {
                println!("eq, {:?}", args);
                match (&args[0], &args[1]) {
                    (&PExpr::Identifier(ref var1), &PExpr::Identifier(ref var2)) => {
                        vec![OpCode::JmpEq(*self.named_registers.get(var1).unwrap(),
                                           *self.named_registers.get(var2).unwrap(),
                                           label)]
                    }
                    (&PExpr::Identifier(ref var), &PExpr::Number(ref val)) => {
                        vec![OpCode::JmpIEq(*self.named_registers.get(var).unwrap(),
                                           *val as i32,
                                           label)]
                    }
                    (&PExpr::Identifier(ref var), &PExpr::Boolean(ref val)) => {
                        vec![OpCode::JmpIEq(*self.named_registers.get(var).unwrap(),
                                           *val as i32,
                                           label)]
                    }
                    _ => panic!()
                }
            }
            _ => panic!()
        }
    }
    
    fn assemble_if(&mut self, expr: &Vec<PExpr>) -> Vec<OpCode> {
        let true_label = self.get_label();
        let end_label = self.get_label();
        
        let mut cond_expr = self.assemble_conditional(&expr[0], true_label);
        let mut true_expr = self.assemble_expr(&expr[1]);
        let mut false_expr = self.assemble_expr(&expr[2]);

        let mut pasm = Vec::new();
        pasm.append(&mut cond_expr);
        pasm.append(&mut false_expr);
        pasm.push(OpCode::Jmp(end_label));
        pasm.push(OpCode::Label(true_label));
        pasm.append(&mut true_expr);
        pasm.push(OpCode::Label(end_label));
        pasm
    }


    fn assemble_npc_say(&mut self, expr: &Vec<PExpr>) -> Vec<OpCode> {
        let mut pasm = Vec::new();
        let npc_id = if let PExpr::Identifier(ref nlabel) = expr[0] {
            *self.npc_ids.get(nlabel).unwrap()
        }
        else {
            -1.
        };

        
        let npc_str = if let PExpr::StringLiteral(ref msg) = expr[1] {
            msg.clone()
        }
        else {
            String::from("azazazaz")
        };

        pasm.push(OpCode::Message(npc_id as u32, npc_str));
        
        for emsg in expr.iter().skip(2) {
            let npc_str = if let &PExpr::StringLiteral(ref msg) = emsg {
                msg.clone()
            }
            else {
                String::from("azazazaz")
            };
            pasm.push(OpCode::AddMsg(npc_str));
        }
        

        pasm.push(OpCode::MesEnd);
        
        pasm
    }
    
    fn assemble_set(&mut self, expr: &Vec<PExpr>) -> Vec<OpCode> {
        let mut pasm = Vec::new();

        let reg = if let PExpr::Identifier(ref vlabel) = expr[0] {
            *self.named_registers.get(vlabel).unwrap()
        }
        else {
            0
        };

        if let PExpr::Boolean(ref val) = expr[1] {
            if *val {
                pasm.push(OpCode::Set(reg));
            }
            else {
                pasm.push(OpCode::Clear(reg));
            }
        }
        else {
            // TODO
        }
        
        pasm
    }


    
    fn assemble_expr(&mut self, expr: &PExpr) -> Vec<OpCode> {
        match expr {
            &PExpr::Block(ref e) => self.assemble_block(&e),
            &PExpr::If(ref e) => self.assemble_if(&e),
            &PExpr::NpcSay(ref e) => self.assemble_npc_say(&e),
            &PExpr::Set(ref e) => self.assemble_set(&e),
            
            _ => Vec::new(),
        }
        
    }

    

    
    fn assemble_npcs(&mut self, npcs: &mut HashMap<String, Npc>) {
        for (nlabel, npc) in npcs.iter_mut() {
            let mut npc_func = if let Function::Expr(ref expr) = npc.function {
                self.assemble_expr(&expr)
            }
            else {
                Vec::new() // panic?
            };

            let nid = *self.npc_labels.get(nlabel).unwrap();
            npc.function = Function::Id(nid as f32);
            self.opcodes.push(OpCode::Label(nid));
            self.opcodes.append(&mut npc_func);
            println!("npcop: {:?}", self.opcodes);
        }
    }


    pub fn as_bytes(&mut self) -> (Vec<u8>, Vec<u8>) {
        let mut pasm = Vec::new();
        let mut func_table = Vec::new();

        let mut maxsize = 1;
        for opcode in self.opcodes.iter() {
            if let &OpCode::Label(label) = opcode {
                maxsize += 1;
            }
        }

        for _ in 0..maxsize {
            func_table.push(0xffffffff);
        }
        
        self.opcodes.push(OpCode::Nop);
        for opcode in self.opcodes.iter() {
            if let &OpCode::Label(label) = opcode {
                func_table[label as usize] = pasm.len();
            }
            else {
                pasm.append(&mut opcode.as_bytes());
            }
        }

        let mut func_table_bytes = Vec::new();
        for func in func_table {
            func_table_bytes.write_u32::<LittleEndian>(func as u32);
        }
        
        (pasm, func_table_bytes)
    }

}

