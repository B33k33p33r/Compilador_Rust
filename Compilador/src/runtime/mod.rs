pub mod windows;
pub mod unix;

use target_lexicon::OperatingSystem;

pub fn generate_runtime(os: OperatingSystem) -> String {
    match os {
        OperatingSystem::Windows => windows::get_runtime(),
        _ => unix::get_runtime(),
    }
}
