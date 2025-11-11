use std::collections::HashMap;

use crate::ir::{Block, Function, Instruction, Terminator, TranslationUnit};

fn get_example_function() -> Function {
    Function {
        name: "_start".to_owned(),
        start_block: Box::from(Block {
            instructions: vec![
                Instruction::Asm(
                    vec![
                        0x6A, 0x1,                                  // push 1
                        0x58,                                       // pop rax
                        0x6A, 0x1,                                  // push 1
                        0x5F,                                       // pop rdi
                        0x48, 0x8d, 0x35, 0x0d, 0x00, 0x00, 0x00,   // lea rsi, [rip+0xe]
                        0x6a, 14,                                   // push 14
                        0x5a,                                       // pop rdx
                        0x0f, 0x05,                                 // syscall
                        0x6a, 0x3c,                                 // push 60
                        0x58,                                       // pop rax
                        0x6a, 0,                                    // push 0
                        0x5f,                                       // pop rdi
                        0x0f, 0x05,                                 // syscall
                        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0A, 0x00
                    ]
                )
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
