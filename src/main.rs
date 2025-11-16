use compyglot::*;
use fluent_bundle::FluentArgs;
use std::error::Error;

//===============\\
// MAIN FUNCTION \\
//===============\\
fn main() -> Result<(), Box<dyn Error>> {

    println!("This is a better way of testing the file features of this library");

    // Declare the global language bundle
    let locale = load_translation_file("en-US", Some("standard.ftl"))?;

    let identifier: TranslationKey = TranslationKey::new(&locale, "hello-world");
    let message: TranslationKey = TranslationKey::new(&locale, "will-is-cool");

    let mut arguments: FluentArgs = FluentArgs::new();
    arguments.set("application-name", "Compyglot Test");

    let app_info: TranslationKey = TranslationKey::new_with_args(&locale, "software-info", &arguments);

    println!("{}", identifier.value());
    println!("{}", message.value());
    println!("{}", app_info.value());

    Ok(())

}//End of Function