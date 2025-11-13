use std::collections::HashMap;

use crate::outputs::serialization::{Serializable, ToBytes};

pub mod sample;

pub struct TranslationUnit {
    name: String,
    pub(crate) functions: HashMap<String, Function>,
    pub(crate) globals: HashMap<String, Value>
}

impl TranslationUnit {
    fn new(name: &'static str) -> TranslationUnit {
        TranslationUnit {
            name: name.to_owned(),
            functions: HashMap::new(),
            globals: HashMap::new()
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

    fn from(instructions: Vec<Instruction>, terminator: Terminator) -> Block {
        Block {
            instructions: instructions,
            terminator: Some(terminator)
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
    Asm(Vec<u8>),
    AsmValue(Value)
}

#[derive(Clone)]
pub enum ConstValue {
    UInt8(u8),
    Int64(i64),
    Array(Vec<ConstValue>)
}

impl Serializable for ConstValue {
    fn serialize(&self, big_endian: bool) -> Vec<u8> {
        match self {
            Self::UInt8(num) => {num.to_bytes(big_endian)}
            Self::Int64(num) => {num.to_bytes(big_endian)}
            Self::Array(vals) => {vals.serialize(big_endian)}
        }
    }
}

#[derive(Clone)]
pub enum Value {
    Const(ConstValue),
    ConstRef(ConstValue)
}


impl Value {
    fn const_i64(num: i64) -> Value {
        Value::Const(ConstValue::Int64(num))
    }

    fn const_str(string: String) -> Value {
        Value::ConstRef(
            ConstValue::Array(
                string.serialize(false).into_iter().map(|x| ConstValue::UInt8(x)).collect()
            )
        )
    }
}

#[derive(Clone)]
pub enum Terminator {
    Jump(Box<Block>),
    Return
}

