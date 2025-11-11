use crate::outputs::serialization::{add_bytes, Serializable};

pub struct ElfHeader {
    pub e_ident_magic: [u8; 4],
    pub e_ident_class: u8,
    pub e_ident_data: u8,
    pub e_ident_version: u8,
    pub e_ident_abi: u8,
    pub e_ident_abi_version: u8,
    pub e_ident_pad: [u8; 7],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

pub struct ElfProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64
}

pub struct ElfSectionHeader {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64
}

pub struct ElfFile {
    pub elf_header: ElfHeader,
    pub elf_program_headers: Vec<ElfProgramHeader>,
    pub elf_section_headers: Vec<ElfSectionHeader>,
    pub data: Vec<u8>
}

pub struct ElfSymbol {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64
}

impl Serializable for ElfHeader {
    fn serialize(&self, be: bool) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend(&self.e_ident_magic);
        add_bytes(&mut vec, self.e_ident_class, be);
        add_bytes(&mut vec, self.e_ident_data, be);
        add_bytes(&mut vec, self.e_ident_version, be);
        add_bytes(&mut vec, self.e_ident_abi, be);
        add_bytes(&mut vec, self.e_ident_abi_version, be);
        vec.extend(&self.e_ident_pad);
        add_bytes(&mut vec, self.e_type, be);
        add_bytes(&mut vec, self.e_machine, be);
        add_bytes(&mut vec, self.e_version, be);
        add_bytes(&mut vec, self.e_entry, be);
        add_bytes(&mut vec, self.e_phoff, be);
        add_bytes(&mut vec, self.e_shoff, be);
        add_bytes(&mut vec, self.e_flags, be);
        add_bytes(&mut vec, self.e_ehsize, be);
        add_bytes(&mut vec, self.e_phentsize, be);
        add_bytes(&mut vec, self.e_phnum, be);
        add_bytes(&mut vec, self.e_shentsize, be);
        add_bytes(&mut vec, self.e_shnum, be);
        add_bytes(&mut vec, self.e_shstrndx, be);
        vec
    }

    fn serialized_length(&self) -> usize {
        0x40
    }
}

impl Serializable for ElfProgramHeader {
    fn serialize(&self, be: bool) -> Vec<u8> {
        let mut vec = Vec::new();

        add_bytes(&mut vec, self.p_type, be);
        add_bytes(&mut vec, self.p_flags, be);
        add_bytes(&mut vec, self.p_offset, be);
        add_bytes(&mut vec, self.p_vaddr, be);
        add_bytes(&mut vec, self.p_paddr, be);
        add_bytes(&mut vec, self.p_filesz, be);
        add_bytes(&mut vec, self.p_memsz, be);
        add_bytes(&mut vec, self.p_align, be);

        vec
    }

    fn serialized_length(&self) -> usize {
        0x38
    }
}

impl Serializable for ElfSymbol {
    fn serialize(&self, be: bool) -> Vec<u8> {
        let mut vec = Vec::new();

        add_bytes(&mut vec, self.st_name, be);
        add_bytes(&mut vec, self.st_info, be);
        add_bytes(&mut vec, self.st_other, be);
        add_bytes(&mut vec, self.st_shndx, be);
        add_bytes(&mut vec, self.st_value, be);
        add_bytes(&mut vec, self.st_size, be);

        vec
    }

    fn serialized_length(&self) -> usize {
        0x18
    }
}

impl Serializable for ElfSectionHeader {
    fn serialize(&self, be: bool) -> Vec<u8> {
        let mut vec = Vec::new();

        add_bytes(&mut vec, self.sh_name, be);
        add_bytes(&mut vec, self.sh_type, be);
        add_bytes(&mut vec, self.sh_flags, be);
        add_bytes(&mut vec, self.sh_addr, be);
        add_bytes(&mut vec, self.sh_offset, be);
        add_bytes(&mut vec, self.sh_size, be);
        add_bytes(&mut vec, self.sh_link, be);
        add_bytes(&mut vec, self.sh_info, be);
        add_bytes(&mut vec, self.sh_addralign, be);
        add_bytes(&mut vec, self.sh_entsize, be);

        vec
    }

    fn serialized_length(&self) -> usize {
        0x40
    }
}

impl Serializable for ElfFile {
    fn serialize(&self, be: bool) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend(self.elf_header.serialize(be));
        vec.extend(self.elf_program_headers.iter().flat_map(|x| x.serialize(be)));
        vec.extend(self.elf_section_headers.iter().flat_map(|x| x.serialize(be)));
        vec.extend(&self.data);

        vec
    }
}
