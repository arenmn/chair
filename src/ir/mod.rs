use std::collections::HashMap;

pub mod sample;

pub struct TranslationUnit {
    name: String,
    pub(crate) functions: HashMap<String, Function>
}

impl TranslationUnit {
    fn new(name: &'static str) -> TranslationUnit {
        let map = HashMap::new();

        TranslationUnit {
            name: name.to_owned(),
            functions: map
        }
    }

    fn add_function(&mut self, function: Function) {
        self.functions.insert(function.name.to_string(), function);
    }
}

pub struct Function {
    pub(crate) name: String,
    pub(crate) start_block: Box<Block>
}

impl Function {
    fn new(name: &'static str, start_block: Block) -> Function {
        Function {
            name: name.to_owned(),
            start_block: Box::from(start_block)
        }
    }
}

#[derive(Clone)]
pub struct Block { 
    pub(crate) instructions: Vec<Instruction>,
    pub(crate) terminator: Option<Terminator> 
}

impl Block {
    fn new() -> Block {
        Block {
            instructions: vec![],
            terminator: None
        }
    }

    fn set_terminator(&mut self, terminator: Terminator) {
        self.terminator = Some(terminator);
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

#[derive(Clone)]
pub enum Instruction {
    Asm(Vec<u8>)
}

#[derive(Clone)]
pub enum Terminator {
    Jump(Box<Block>),
    Return
}

