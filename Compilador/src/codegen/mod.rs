use crate::ir::IRProgram;
use target_lexicon::OperatingSystem;

pub mod windows;
pub mod unix;

pub fn generate_code(ir: IRProgram, os: OperatingSystem) -> String {
    match os {
        OperatingSystem::Windows => windows::generate_windows_asm(ir),
        _ => unix::generate_unix_asm(ir),
    }
}
