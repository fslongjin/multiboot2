//! Exports item [`Multiboot2InformationBuilder`].
use crate::builder::traits::StructAsBytes;
use crate::{BootLoaderNameTag, CommandLineTag, ElfSectionsTag, ModuleTag};

use alloc::boxed::Box;
use alloc::vec::Vec;

/// Builder to construct a valid Multiboot2 information dynamically at runtime.
/// The tags will appear in the order of their corresponding enumeration,
/// except for the END tag.
#[derive(Debug)]
pub struct Multiboot2InformationBuilder {
    boot_loader_name_tag: Option<Box<BootLoaderNameTag>>,
    command_line_tag: Option<Box<CommandLineTag>>,
    elf_sections_tag: Option<Box<ElfSectionsTag>>,
    module_tags: Vec<Box<ModuleTag>>,
}

impl Multiboot2InformationBuilder {
    pub const fn new() -> Self {
        Self {
            boot_loader_name_tag: None,
            command_line_tag: None,
            elf_sections_tag: None,
            module_tags: Vec::new(),
        }
    }

    pub fn bootloader_name_tag(&mut self, boot_loader_name_tag: Box<BootLoaderNameTag>) {
        self.boot_loader_name_tag = Some(boot_loader_name_tag);
    }

    pub fn command_line_tag(&mut self, command_line_tag: Box<CommandLineTag>) {
        self.command_line_tag = Some(command_line_tag);
    }

    pub fn elf_sections_tag(&mut self, elf_sections_tag: Box<ElfSectionsTag>) {
        self.elf_sections_tag = Some(elf_sections_tag);
    }

    pub fn add_module_tag(&mut self, module_tag: Box<ModuleTag>) {
        self.module_tags.push(module_tag);
    }
}
