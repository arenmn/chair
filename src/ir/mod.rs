use std::collections::HashMap;

mod sample;

pub struct TranslationUnit {
    pub name: String,
    pub functions: HashMap<String, Function>,
    pub entrypoint: String
}

pub struct Function {
    pub name: String,
    pub start_block: Block
}

pub struct Block { 
    pub statements: Vec<Statement>, 
    pub terminator: Terminator 
}

pub enum Statement {
    Asm(Vec<u8>)
}

pub enum Terminator {
    Jump(Box<Block>),
    Return
}

