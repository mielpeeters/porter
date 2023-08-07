/*!
* Read the `TOML` declaration file and use html module to create the site.
*/

use std::{fmt::Display, fs, path::PathBuf};

use toml::{self, Table, Value};

use crate::{data, html};

#[derive(Debug)]
enum Error {
    // problems in reading files
    FileReadError,
    // problems with parsing the toml file
    ParseError,
    // couldn't find the required key
    KeyError,
    // array has the wrong length
    WrongLength,
    // there was no colours array
    NoColours,
    // there was no 3 colour values given
    NoRGB,
    // should be an integer...
    NoInteger,
    // there was no images array
    NoImages,
    // thats not an image
    NotAnImage,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileReadError => writeln!(f, "problems in reading files"),
            Error::ParseError => writeln!(f, "problems with parsing the toml file"),
            Error::KeyError => writeln!(f, "couldn't find the required key"),
            Error::WrongLength => writeln!(f, "array has the wrong length"),
            Error::NoColours => writeln!(f, "there was no colours array"),
            Error::NoRGB => writeln!(f, "there was no 3 colour values given"),
            Error::NoInteger => writeln!(f, "should be an integer..."),
            Error::NoImages => writeln!(f, "there was no images array"),
            Error::NotAnImage => writeln!(f, "thats not an image"),
        }
    }
}

fn parse_toml(path: PathBuf) -> Result<Table, Error> {
    let string = match fs::read_to_string(path) {
        Ok(string) => string,
        Err(_) => return Err(Error::FileReadError),
    };

    match string.parse::<Table>() {
        Ok(table) => Ok(table),
        Err(_) => Err(Error::ParseError),
    }
}

fn insert(html: &mut String, item: &Table, key: &str) -> Result<(), Error> {
    let value = match item.get(key) {
        Some(Value::Array(arr)) => {
            if arr.len() != 2 {
                return Err(Error::WrongLength);
            }
            format!(
                "{} <img src=\"images/icons/cross.svg\" width=\"8px\"> {}",
                arr[0].to_string(),
                arr[1].to_string()
            )
        }
        Some(Value::String(val)) => {
            let res = markdown::to_html(val);
            println!("{}", res);
            res
        }
        Some(val) => val.to_string(),
        None => match key {
            "measurements" => "-".to_string(),
            _ => return Err(Error::KeyError),
        },
    };

    html::insert(html, &[value], key.to_uppercase().as_str()).expect("Should be able to insert");
    Ok(())
}

fn insert_colors(html: &mut String, item: &Table) -> Result<(), Error> {
    let Some(Value::Array(arr)) = item.get("colours") else {
        return Err(Error::NoColours)
    };

    let mut colours: Vec<String> = Vec::with_capacity(arr.len());

    for colour in arr {
        let colour_string = match colour {
            Value::Array(clr) => {
                if clr.len() != 3 {
                    return Err(Error::NoRGB);
                }
                let r = match clr[0] {
                    Value::Integer(i) => i as u8,
                    _ => return Err(Error::NoInteger),
                };
                let g = match clr[1] {
                    Value::Integer(i) => i as u8,
                    _ => return Err(Error::NoInteger),
                };
                let b = match clr[2] {
                    Value::Integer(i) => i as u8,
                    _ => return Err(Error::NoInteger),
                };
                let nb: u32 = r as u32 * 256 * 256 + g as u32 * 256 + b as u32;

                format!("#{:x}", nb)
            }
            Value::String(clr) => clr.clone(),
            _ => return Err(Error::NoRGB),
        };

        let colourgrid_html = data::colordrid();
        let colourgrid_html = colourgrid_html.replace("COLOR", &colour_string);

        colours.push(colourgrid_html);
    }

    html::insert(html, &colours, "COLORGRID").expect("Should be able to add colours");
    Ok(())
}

fn insert_images(html: &mut String, item: &Table) -> Result<(), Error> {
    let Some(Value::Array(arr)) = item.get("images") else {
        return Err(Error::NoImages)
    };

    let sizes = ["320", "640", "960", "1290", "1920", "2560"];
    let mut images: Vec<String> = Vec::with_capacity(arr.len());

    let Some(Value::String(pre)) = item.get("images_dir") else {
        return Err(Error::NoImages)
    };

    for image in arr {
        let Value::String(image_string) = image else {
            return Err(Error::NotAnImage)
        };

        let mut image_html = data::image();

        image_html = image_html.replace(
            "PATH",
            format!("images/{}/resized/{}-2560.webp", pre, image_string).as_str(),
        );

        for size in sizes {
            image_html = image_html.replace(
                format!("{}W", size).as_str(),
                format!("images/{}/resized/{}-{}.webp", pre, image_string, size).as_str(),
            );
        }

        images.push(image_html);
    }

    html::insert(html, &images, "IMAGES").expect("Should be able to add images.");

    Ok(())
}

fn get_item(table: &Table, key: &str) -> Option<Table> {
    let item = table.get(key)?;
    match item {
        Value::Table(tab) => Some(tab.clone()),
        _ => None,
    }
}

pub fn create_site(
    declaration: PathBuf,
    template: PathBuf,
    output: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let table = parse_toml(declaration)?;
    let mut page = html::open_template(template);

    let mut items: Vec<String> = Vec::with_capacity(table.keys().len());

    for key in table.keys() {
        println!("Inserting '{key}'");

        let item = get_item(&table, key).expect("Should be able to get a table for this key...");
        let mut item_html = data::item();

        item_html = item_html.replace("NAME", key);
        insert(&mut item_html, &item, "year")?;
        insert(&mut item_html, &item, "measurements")?;
        insert(&mut item_html, &item, "description")?;
        insert_colors(&mut item_html, &item)?;
        insert_images(&mut item_html, &item)?;

        items.push(item_html.clone());
    }

    html::insert(&mut page, &items.as_slice(), "ITEMS").expect("Should be able to insert");

    html::save(&page, output);
    Ok(())
}
