use std::collections::HashMap;
use crate::codegen::Codegen;
use crate::ir::{Block, Function, Instruction, Terminator, TranslationUnit, Value};
use crate::outputs::elf::{ElfFile, ElfHeader, ElfRelocation, ElfSectionHeader, ElfSymbol};
use crate::outputs::serialization::Serializable;

enum Section {
    Text,
    Rodata
}

struct Relocation {
    src_symbol: usize,
    dst_section: Section,
    dst_offset: usize
}

struct Symbol {
    section: Section,
    offset: usize,
    size: usize,
    name: Option<String>,
    elf_type: u8
}

pub struct CompilerX64Elf {
    pub(crate) text: Vec<u8>,
    pub(crate) rodata: Vec<u8>,
    pub(crate) relocations: Vec<Relocation>,
    pub(crate) symbols: Vec<Symbol>
}

impl CompilerX64Elf {

    pub fn new() -> CompilerX64Elf {
        CompilerX64Elf { 
            text: vec![],
            rodata: vec![],
            relocations: vec![],
            symbols: vec![]
        }
    }

    fn compile_block(&mut self, block: &Block) {
        for instr in block.instructions.iter() {
            self.compile_instruction(instr);
        }

        self.compile_terminator(block.terminator.clone().expect("Attempt to compile block with no terminator"));

    }

    fn compile_terminator(&mut self, terminator: Terminator) {
        match terminator {
            Terminator::Return => {
                self.text.extend(vec![0xC3]);
            },
            Terminator::Jump(_) => {
                let vec: Vec<u8> = Vec::new();
                self.text.extend(vec);
            },
        }
    }

    fn compile_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Asm(x) => {
                self.text.extend(x)
            },

            Instruction::AsmValue(val) => {
                self.compile_value(val)
            }
        }
    }

    fn compile_function(&mut self, function: &Function) {
        self.compile_block(&*function.start_block)
    }

    fn compile_value(&mut self, value: &Value) {
        match value {
            Value::Const(val) => {
                self.text.extend(val.serialize(false))
            },
            Value::ConstRef(val) => {
                self.symbols.push(Symbol {
                    section: Section::Rodata,
                    size: 8,
                    offset: self.rodata.len(),
                    name: None,
                    elf_type: 1
                });

                self.relocations.push(Relocation {
                    src_symbol: self.symbols.len(),
                    dst_section: Section::Text,
                    dst_offset: self.text.len()
                });

                self.rodata.extend(val.serialize(false));
                self.text.extend(vec![0, 0, 0, 0, 0, 0, 0, 0]);
            }
        }
    }
}

impl Codegen for CompilerX64Elf {
    type OutputFormat = ElfFile;
    fn compile_translation_unit(&mut self, translation_unit: TranslationUnit) -> ElfFile {
        for (name, func) in &translation_unit.functions {

            let function_start = self.text.len();
            self.compile_function(&func);

            self.symbols.push(Symbol {
                offset: function_start,
                elf_type: 2,
                size: self.text.len() - function_start,
                section: Section::Text,
                name: Some(name.to_string())
            })
        }

        let section_names: Vec<String> = vec!["".to_owned(), ".text".to_owned(), ".shstrtab".to_owned(), ".symtab".to_owned(), ".strtab".to_owned(), ".rodata".to_owned(), ".rel.text".to_owned()];

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
                e_shnum: 7,
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
                    //sh_addr: 0x40000000,
                    sh_addr: 0,
                    sh_offset: 0x80,
                    sh_size: self.text.len() as u64,
                    sh_link: 0,
                    sh_info: 0,
                    sh_addralign: 0,
                    sh_entsize: 0
                },
                ElfSectionHeader {
                    sh_name: 7,
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
                    sh_name: 17,
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
                    sh_name: 25,
                    sh_type: 3,
                    sh_flags: 0x20,
                    sh_addr: 0,
                    sh_size: 0,
                    sh_info: 0,
                    sh_link: 0,
                    sh_entsize: 0,
                    sh_addralign: 0,
                    sh_offset: 0
                },
                ElfSectionHeader {
                    sh_name: 33,
                    sh_type: 1,
                    sh_flags: 2,
                    sh_addr: 0,
                    sh_addralign: 0,
                    sh_link: 0,
                    sh_info: 0,
                    sh_size: self.rodata.len() as u64,
                    sh_offset: 0,
                    sh_entsize: 0,
                },
                ElfSectionHeader {
                    sh_name: 41,
                    sh_type: 9,
                    sh_flags: 0,
                    sh_addr: 0,
                    sh_offset: 0,
                    sh_size: self.relocations.len() as u64 * 16,
                    sh_link: 3,
                    sh_info: 1,
                    sh_addralign: 0,
                    sh_entsize: 16,
                }
            ],
            data: vec![]
        };

        let header_len = elf.serialized_length();
        elf.elf_section_headers[1].sh_offset = header_len as u64;

        elf.data.extend_from_slice(&self.text);

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

        for sym in self.symbols.iter() {
            let mut name_idx = 0;

            let shndx = match sym.section {
                Section::Text => 1,
                Section::Rodata => 5
            };

            match &sym.name {
                Some(x) => {
                    name_idx = symbol_table_names.serialized_length();
                    symbol_table_names.push(x.to_string());
                }
                _ => (),
            }

            let symbol = ElfSymbol {
                st_name: name_idx as u32,
                st_info: (1<<4)+(sym.elf_type&0xf),
                st_other: 0,
                st_shndx: shndx,
                st_size: sym.size as u64,
                st_value: sym.offset as u64
            };

            symbol_table.push(symbol);
        }

        elf.elf_section_headers[3].sh_size = symbol_table.iter().fold(0, |acc, x|
            acc + (x.serialized_length() as u64));

        elf.data.extend(symbol_table.iter().flat_map(|symbol| {symbol.serialize(false)}));

        elf.elf_section_headers[4].sh_offset = elf.serialized_length() as u64;
        elf.elf_section_headers[4].sh_size = symbol_table_names.iter().fold(0, |acc, x| acc + (x.to_string().serialized_length() as u64));

        elf.data.extend(symbol_table_names.iter().flat_map(|symbol_name| {symbol_name.to_string().serialize(false)}));
        elf.elf_section_headers[5].sh_offset = elf.serialized_length() as u64;
        elf.data.extend_from_slice(&self.rodata);

        elf.elf_section_headers[6].sh_offset = elf.serialized_length() as u64;

        elf.data.extend(
            self.relocations.iter().map(|reloc| {ElfRelocation{
                r_offset: reloc.dst_offset as u64,
                r_info: ((reloc.src_symbol<<32) + (1&0xffffffff)) as u64
            }}).collect::<Vec<ElfRelocation>>().serialize(false)
        );



        elf
    }
}
