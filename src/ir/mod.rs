use std::collections::HashMap;

pub mod sample;

pub struct TranslationUnit {
    pub name: String,
    pub functions: HashMap<String, Function>
}

pub struct Function {
    pub name: String,
    pub start_block: Box<Block>
}

pub struct Block { 
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator 
}

pub enum Instruction {
    Asm(Vec<u8>)
}

pub enum Terminator {
    Jump(Box<Block>),
    Return
}

