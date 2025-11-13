use crate::ir::{Block, Function, Instruction, Terminator, TranslationUnit, Value};

pub fn get_example_translation_unit() -> TranslationUnit {
    let mut translation_unit = TranslationUnit::new("cook");

    let mut block = Block::new();

    let str = Value::const_str("Hello, World!\n".to_owned());

    block.add_instruction(Instruction::Asm(vec![
        0x6A, 0x1,                                  // push 1
        0x58,                                       // pop rax
        0x6A, 0x1,                                  // push 1
        0x5F,                                       // pop rdi
        0x48, 0xbe                                  // put following 64-bit immediate into rsi
    ]));
    
    block.add_instruction(Instruction::AsmValue(str));

    block.add_instruction(Instruction::Asm(vec![
        0x6a, 14,                                   // push 14
        0x5a,                                       // pop rdx
        0x0f, 0x05,                                 // syscall
        0x6a, 0x3c,                                 // push 60
        0x58,                                       // pop rax
        0x48, 0xbf,                                 // put following 64-bit immediate into rdi
    ]));

    block.add_instruction(Instruction::AsmValue(Value::const_i64(0)));

    block.add_instruction(Instruction::Asm(vec![
        0x0f, 0x05,                                 // syscall
    ]));

    block.set_terminator(Terminator::Return);
    translation_unit.add_function(Function::new("_start", block));

    translation_unit
}
