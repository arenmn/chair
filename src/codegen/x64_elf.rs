use std::collections::HashMap;
use crate::codegen::Codegen;
use crate::ir::{Block, Function, Instruction, Terminator, TranslationUnit};
use crate::outputs::elf::{ElfFile, ElfHeader, ElfSectionHeader, ElfSymbol};
use crate::outputs::serialization::Serializable;

pub struct CompilerX64Elf {
    pub(crate) function_positions: HashMap<String, usize>,
    pub(crate) function_sizes: HashMap<String, usize>
}

impl CompilerX64Elf {

    pub fn new() -> CompilerX64Elf {
        CompilerX64Elf { function_positions: HashMap::new(), function_sizes: HashMap::new() }
    }

    fn compile_block(&mut self, block: &Block) -> Vec<u8> {
        let mut x: Vec<u8> = block.instructions.iter().flat_map(
            |instruction| self.compile_instruction(instruction)
        ).collect();

        x.extend(self.compile_terminator(block.terminator.clone().expect("Attempt to compile block with no terminator")));

        x
    }

    fn compile_terminator(&mut self, terminator: Terminator) -> Vec<u8> {
        match terminator {
            Terminator::Return => {vec![0xC3]},
            Terminator::Jump(_) => {vec![]},
        }
    }

    fn compile_instruction(&mut self, instruction: &Instruction) -> Vec<u8> {
        match instruction {
            Instruction::Asm(x) => x.clone()
        }
    }

    fn compile_function(&mut self, function: &Function) -> Vec<u8> {
        self.compile_block(&*function.start_block)
    }
}

impl Codegen for CompilerX64Elf {
    type OutputFormat = ElfFile;
    fn compile_translation_unit(&mut self, translation_unit: TranslationUnit) -> ElfFile {
        let mut text: Vec<u8> = vec![];

        for (name, func) in &translation_unit.functions {
            let compiled_function = self.compile_function(&func);

            let function_start = text.len();

            self.function_positions.insert(name.to_string(), function_start);
            self.function_sizes.insert(name.to_string(), compiled_function.len());

            text.extend(compiled_function);
        }

        let section_names: Vec<String> = vec!["".to_owned(), ".text".to_owned(), ".shstrtab".to_owned(), ".symtab".to_owned(), ".strtab".to_owned()];

        let mut elf = ElfFile {
            elf_header: ElfHeader {
                e_ident_magic: [0x7F, 0x45, 0x4c, 0x46],
                e_ident_class: 2,
                e_ident_data: 1,
                e_ident_version: 1,
                e_ident_abi: 3,
                e_ident_abi_version: 67,
                e_ident_pad: [0,0,0,0,0,0,0],
                e_type: 1,
                e_machine: 0x3E,
                e_version: 1,
                e_entry: 0,
                e_phoff: 0,
                e_shoff: 0x40,
                e_flags: 0,
                e_ehsize: 0x40,
                e_phentsize: 0x38,
                e_phnum: 0,
                e_shentsize: 0x40,
                e_shnum: 5,
                e_shstrndx: 2
            },
            elf_program_headers: vec![],
            elf_section_headers: vec![
                ElfSectionHeader {
                    sh_name: 0,
                    sh_type: 0,
                    sh_flags: 0,
                    sh_addr: 0,
                    sh_offset: 0,
                    sh_size: 0,
                    sh_link: 0,
                    sh_info: 0,
                    sh_addralign: 0,
                    sh_entsize: 0
                },
                ElfSectionHeader {
                    sh_name: 1,
                    sh_type: 1,
                    sh_flags: 2 | 4,
                    sh_addr: 0x40000000,
                    sh_offset: 0x80,
                    sh_size: text.len() as u64,
                    sh_link: 0,
                    sh_info: 0,
                    sh_addralign: 0,
                    sh_entsize: 0
                },
                ElfSectionHeader {
                    sh_name: 2,
                    sh_type: 3,
                    sh_flags: 0x20,
                    sh_addr: 0,
                    sh_size: section_names.iter().fold(0, |acc, x|
                        acc + (x.to_string().serialized_length() as u64)
                    ),
                    sh_info: 0,
                    sh_link: 0,
                    sh_offset: 0,
                    sh_addralign: 0,
                    sh_entsize: 0
                },
                ElfSectionHeader {
                    sh_name: 3,
                    sh_type: 2,
                    sh_flags: 0,
                    sh_addr: 0,
                    sh_offset: 0,
                    sh_entsize: 0x18,
                    sh_link: 4,
                    sh_size: 0,
                    sh_info: 1,
                    sh_addralign: 0,
                },
                ElfSectionHeader {
                    sh_name: 4,
                    sh_type: 3,
                    sh_flags: 0x20,
                    sh_addr: 0,
                    sh_size: 0,
                    sh_info: 0,
                    sh_link: 0,
                    sh_entsize: 0,
                    sh_addralign: 0,
                    sh_offset: 0
                }
            ],
            data: vec![]
        };

        let header_len = elf.serialized_length();
        elf.elf_section_headers[1].sh_offset = header_len as u64;

        elf.data.extend(text);

        let header_text_len = elf.serialized_length();

        elf.data.extend(section_names.iter().flat_map(|section_name| {section_name.to_string().serialize(false)}));
        elf.elf_section_headers[2].sh_offset = header_text_len as u64;

        let symbol_table_start = elf.serialized_length();

        elf.elf_section_headers[3].sh_offset = symbol_table_start as u64;
        let mut symbol_table: Vec<ElfSymbol> = vec![
            ElfSymbol {
                st_name: 0,
                st_info: 0,
                st_other: 0,
                st_size: 0,
                st_shndx: 0,
                st_value: 0
            }
        ];
        let mut symbol_table_names: Vec<String> = vec!["".to_owned()];

        for name in self.function_positions.keys() {
            let idx = symbol_table.len();

            symbol_table_names.push(name.to_string());

            let symbol = ElfSymbol {
                st_name: idx as u32,
                st_info: (1<<4)+(2&0xf),
                st_other: 0,
                st_shndx: 1,
                st_size: self.function_sizes[name] as u64,
                st_value: self.function_positions[name] as u64
            };

            symbol_table.push(symbol);
        }

        elf.elf_section_headers[3].sh_size = symbol_table.iter().fold(0, |acc, x|
            acc + (x.serialized_length() as u64));

        elf.data.extend(symbol_table.iter().flat_map(|symbol| {symbol.serialize(false)}));

        elf.elf_section_headers[4].sh_offset = elf.serialized_length() as u64;
        elf.elf_section_headers[4].sh_size = symbol_table_names.iter().fold(0, |acc, x| acc + (x.to_string().serialized_length() as u64));

        elf.data.extend(symbol_table_names.iter().flat_map(|symbol_name| {symbol_name.to_string().serialize(false)}));

        elf
    }
}
