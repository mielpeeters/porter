use std::fmt::Display;

#[derive(Debug)]
pub enum PorterError {
    // problems in reading files
    FileReadError(String),
    // problems with parsing the toml file
    ParseError,
    // couldn't find the required key
    KeyError(String),
    // array has the wrong length
    WrongLength,
    // there was no colours array
    NoColours,
    // there was no 3 colour values given
    NoRGB,
    // should be an integer...
    NoInteger,
    // there was no images array
    NoImagesDir,
    // wrong type
    WrongType(String, String),
    // No ITEMS
    NoItems,
    // no match found
    NoMatch(String),
    // path convert error
    PathConvert(String),
}

impl std::error::Error for PorterError {}

impl Display for PorterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PorterError::FileReadError(file) => {
                writeln!(f, "Error: problems in reading file {file}")
            }
            PorterError::ParseError => writeln!(f, "Error: problems with parsing the toml file"),
            PorterError::KeyError(s1) => writeln!(f, "Error: couldn't find key '{s1}'"),
            PorterError::WrongLength => writeln!(f, "Error: array has the wrong length"),
            PorterError::NoColours => writeln!(f, "Error: there was no colours array"),
            PorterError::NoRGB => writeln!(f, "Error: there was no 3 colour values given"),
            PorterError::NoInteger => writeln!(f, "Error: should be an integer..."),
            PorterError::NoImagesDir => writeln!(
                f,
                "Error: there was no images_dir supplied in the declaration"
            ),
            PorterError::WrongType(key, tpe) => {
                writeln!(
                    f,
                    "Error: Wrong type supplied for key {key}, should be {tpe}"
                )
            }
            PorterError::NoItems => {
                writeln!(f, "Error: there is no occurrence of ITEMS in the template.")
            }
            PorterError::NoMatch(pattern) => {
                writeln!(f, "Error: there is no match of pattern '{pattern}'.")
            }
            PorterError::PathConvert(path) => writeln!(f, "problem in converting {path}"),
        }
    }
}
