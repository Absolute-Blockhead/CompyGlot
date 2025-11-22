use std::borrow::Cow;
use compyglot::*;
use fluent_bundle::{FluentArgs, FluentBundle, FluentMessage, FluentResource, FluentValue};
use std::error::Error;
use unic_langid::LanguageIdentifier;

//===============\\
// MAIN FUNCTION \\
//===============\\
fn main() -> Result<(), Box<dyn Error>> {

    println!("This is a better way of testing the file features of this library");

    // Declare the global language bundle
    let locale = load_translation_file("en-US", Some("standard.ftl"))?;

    let identifier: TranslationKey = TranslationKey::new(&locale, "hello-world");
    let message: TranslationKey = TranslationKey::new(&locale, "i-am-cool");

    let mut arguments: FluentArgs = FluentArgs::new();
    arguments.set("application-name", "Compyglot Test");

    let app_info: TranslationKey = TranslationKey::new_with_args(&locale, "software-info", &arguments);

    println!("{}", identifier.value());
    println!("{}", message.value());
    println!("{}", app_info.value());

    Ok(())

}//End of Function

fn test_fluent_with_strings() {

    let ftl_string = String::from("hello-world = Hello World!\nintro = Welcome,{$name}.");

    let resource: FluentResource = FluentResource::try_new(ftl_string).expect("Failed to create Fluent resource.");
    let language_id: LanguageIdentifier = "en-US".parse().expect("Failed to parse language identifier.");
    let mut bundle = FluentBundle::new(vec![language_id]);
    bundle.add_resource(resource).expect("Failed to add Fluent resource to the bundle.");

    let message: FluentMessage = bundle.get_message("hello-world").expect("Failed to get message.");
    let pattern = message.value().expect("Failed to get message.");
    let value: Cow<str> = bundle.format_pattern(&pattern, None, &mut vec![]);

    assert_eq!(value, "Hello World!");
    println!("{}", value);

    let mut arguments: FluentArgs = FluentArgs::new();
    arguments.set("name", FluentValue::from("Will"));

    let message: FluentMessage = bundle.get_message("intro").expect("Failed to get message.");
    let pattern = message.value().expect("Failed to get message.");
    let value: Cow<str> = bundle.format_pattern(&pattern, Some(&arguments), &mut vec![]);

    println!("{}", value);

}//End of Function