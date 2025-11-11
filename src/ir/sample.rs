use std::collections::HashMap;

use crate::ir::{Block, Function, Instruction, Terminator, TranslationUnit};

fn get_example_function() -> Function {
    Function {
        name: "_start".to_owned(),
        start_block: Box::from(Block {
            instructions: vec![
                Instruction::Asm(vec![0x6a, 0x3c, 0x58, 0x6a, 0, 0x5f, 0x0f, 0x05])
            ],
            terminator: Terminator::Return
        })
    }
}

pub fn get_example_translation_unit() -> TranslationUnit {
    let mut funcs = HashMap::new();

    funcs.insert("_start".to_owned(), get_example_function());

    TranslationUnit {
        name: "cook".to_owned(),
        functions: funcs,
    }
}
