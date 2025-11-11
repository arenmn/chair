use std::collections::HashMap;

use crate::ir::{Block, Function, Statement, Terminator, TranslationUnit};

fn get_example_function() -> Function {
    Function {
        name: "main".to_owned(),
        start_block: Block {
            statements: vec![
                Statement::Asm(vec![])
            ],
            terminator: Terminator::Return
        }
    }
}

fn get_example_translation_unit() -> TranslationUnit {
    let mut funcs = HashMap::new();

    funcs.insert("main".to_owned(), get_example_function());

    TranslationUnit {
        name: "cook".to_owned(),
        functions: funcs,
        entrypoint: "main".to_owned()
    }
}
