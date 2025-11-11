#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

pub mod lang_manager;

pub use self::lang_manager::TranslationKey;
pub use self::lang_manager::load_translation;
pub use self::lang_manager::load_translation_file;


#[cfg(test)]
mod tests {}