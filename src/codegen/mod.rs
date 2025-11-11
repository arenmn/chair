use crate::ir::TranslationUnit;

pub mod x64_elf;

pub trait Codegen {
    type OutputFormat;
    fn compile_translation_unit(&mut self, translation_unit: TranslationUnit) -> Self::OutputFormat;
}