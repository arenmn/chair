use std::fs::write;

mod ir;
mod linking;
mod codegen;
mod outputs;

use crate::outputs::serialization::*;
use crate::codegen::Codegen;
use crate::codegen::x64_elf::CompilerX64Elf;
use crate::ir::sample::get_example_translation_unit;

fn main() {

    let elf = CompilerX64Elf::new().compile_translation_unit(get_example_translation_unit());

    write("a.o", elf.serialize(false)).expect("file write shit fuck");
    println!("written program to a.o");
}

/*
fn main() {

    println!("Hello, world!");
    let mut elf = ElfHeader {
        e_ident_magic: [0x7F, 0x45, 0x4c, 0x46],
        e_ident_class: 2,
        e_ident_data: 1,
        e_ident_version: 1,
        e_ident_abi: 3,
        e_ident_abi_version: 67,
        e_ident_pad: [0,0,0,0,0,0,0],
        e_type: 2,
        e_machine: 0x3E,
        e_version: 1,
        e_entry: 0,
        e_phoff: 0x40,
        e_shoff: 0,
        e_flags: 0,
        e_ehsize: 0x40,
        e_phentsize: 0x38,
        e_phnum: 2,
        e_shentsize: 0x40,
        e_shnum: 0,
        e_shstrndx: 0
    };

    let mut prog = vec![
        0x6Au8, 0x1,                    // push 1
        0x58,                           // pop rax
        0x6A, 0x1,                      // push 1
        0x5F,                           // pop rdi
        0x68, 0x00, 0x20, 0x00, 0x40,   // push addr to string
        0x5e,                           // pop rsi
        0x6a, 14,                       // push 14
        0x5a,                           // pop rdx
        0x0f, 0x05,                     // syscall
        0x6a, 0x3c,                     // push 60
        0x58,                           // pop rax
        0x6a, 0,                        // push 0
        0x5f,                           // pop rdi
        0x0f, 0x05                      // syscall
    ];

    let mut rodata = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0A, 0x00];

    let elf_ph_text = ElfProgramHeader {
        p_type: 1,
        p_flags: 1 | 2 | 4,
        p_offset: 0x1000,
        p_vaddr: 0x40001000,
        p_paddr: 0,
        p_filesz: prog.len() as u64,
        p_memsz: prog.len() as u64,
        p_align: 0x1000
    };

    let mut final_vec = Vec::new();

    let elf_ph_rodata = ElfProgramHeader {
        p_type: 1,
        p_flags: 4,
        p_offset: 0x2000,
        p_vaddr: 0x40002000,
        p_paddr: 0,
        p_filesz: rodata.len() as u64,
        p_memsz: rodata.len() as u64,
        p_align: 0x1000
    };

    elf.e_entry = 0x40001000;


    final_vec.append(&mut elf.serialize(false));
    final_vec.append(&mut elf_ph_text.serialize(false));
    final_vec.append(&mut elf_ph_rodata.serialize(false));
    final_vec.resize(0x1000, 0);
    final_vec.append(&mut prog);
    final_vec.resize(0x2000, 0);
    final_vec.append(&mut rodata);


    write("a.out", final_vec).expect("file write shit fuck");
    println!("written program to a.out");

}
*/
