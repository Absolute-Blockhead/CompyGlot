//=========\\
// IMPORTS \\
//=========\\
use fluent::{FluentBundle, FluentMessage, FluentResource};
use std::borrow::Cow;
use std::fs::DirEntry;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::{fs, io};


//==================\\
// DEFINE FUNCTIONS \\
//==================\\
pub fn load_translation_file(language: &str, file: Option<&str>) -> io::Result<FluentBundle<FluentResource>> {

    let path: String = if let Some(file_name) = file {

        format!("./locale/{}/{}", language, file_name)

    } else { format!("./locale/{}", language) };

    let mut bundle = FluentBundle::default();

    let resource: FluentResource = FluentResource::try_new(fs::read_to_string(path)?).map_err(|_| io::Error::new(ErrorKind::NotFound, "Failed to read Fluent resource file"))?;
    bundle.add_resource(resource).map_err(|_| io::Error::new(ErrorKind::InvalidData, "Failed to add Fluent resource to the bundle."))?;

    Ok(bundle)

}//End of Function

// I'm not sure how having multiple resources in a bundle works, I must do some more research on that.
pub fn load_translation(language: &str) -> io::Result<FluentBundle<FluentResource>> {

    let mut bundle = FluentBundle::default();
    let mut entries: Vec<PathBuf> = fs::read_dir(&format!("./locale/{}/", language))?
        .map(|resource: io::Result<DirEntry>| resource.map(|error: DirEntry| error.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    for entry in entries {

        let resource: FluentResource = FluentResource::try_new(fs::read_to_string(entry)?)
            .map_err(|_| io::Error::new(ErrorKind::NotFound, "Failed to read Fluent resource file"))?;

        bundle.add_resource(resource)
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, "Failed to add Fluent resource to the bundle."))?;

    }//End of For-Each Loop

    Ok(bundle)

}//End of Function


//========================\\
// DEFINE DATA STRUCTURES \\
//========================\\
pub struct TranslationKey(String, ());
impl TranslationKey {

    // --- CONSTRUCTOR --- \\
    pub fn new(locale: &FluentBundle<FluentResource>, translation_id: &str) -> TranslationKey {

        let formatted_value: Cow<str> = locale
            .get_message(translation_id)
            .and_then(|message: FluentMessage| message.value())
            .map_or_else(|| {

                println!("The message behind '{}' doesn't exist.", translation_id);
                "".into()//Default value if None

            },
            |value| locale.format_pattern(value, None, &mut vec![]));

        TranslationKey(formatted_value.to_string(), ())

    }//End of Constructor

    pub fn value(&self) -> &str { self.0.as_str() }

}//End of Implementation