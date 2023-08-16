/*!
* Read the `TOML` declaration file and use html module to create the site.
*/

use std::{fs, path::PathBuf};

use tauri::AppHandle;
use toml::{self, Table, Value};

use crate::{
    data::{self, Data},
    error::PorterError,
    html,
};

fn parse_toml(path: PathBuf) -> Result<Table, PorterError> {
    let string = match fs::read_to_string(path.clone()) {
        Ok(string) => string,
        Err(_) => {
            return Err(PorterError::FileReadError(
                path.to_str().unwrap().to_string(),
            ))
        }
    };

    match string.parse::<Table>() {
        Ok(table) => Ok(table),
        Err(_) => Err(PorterError::ParseError),
    }
}

fn insert(html: &mut String, item: &Table, key: &str) -> Result<(), PorterError> {
    let value = match item.get(key) {
        Some(Value::Array(arr)) => {
            if arr.len() != 2 {
                return Err(PorterError::WrongLength);
            }
            format!(
                "{} <img src=\"images/icons/cross.svg\" width=\"8px\"> {}",
                arr[0].to_string(),
                arr[1].to_string()
            )
        }
        Some(Value::String(val)) => {
            let res = markdown::to_html(val);
            res
        }
        Some(val) => val.to_string(),
        None => match key {
            "measurements" => "-".to_string(),
            _ => return Err(PorterError::KeyError(key.to_string())),
        },
    };

    html::insert(html, &[value], key.to_uppercase().as_str()).expect("Should be able to insert");
    Ok(())
}

fn insert_colors(html: &mut String, item: &Table, data: &Data) -> Result<(), PorterError> {
    let Some(Value::Array(arr)) = item.get("colours") else {
        return Err(PorterError::NoColours)
    };

    let mut colours: Vec<String> = Vec::with_capacity(arr.len());

    for colour in arr {
        let colour_string = match colour {
            Value::Array(clr) => {
                if clr.len() != 3 {
                    return Err(PorterError::NoRGB);
                }
                let r = match clr[0] {
                    Value::Integer(i) => i as u8,
                    _ => return Err(PorterError::NoInteger),
                };
                let g = match clr[1] {
                    Value::Integer(i) => i as u8,
                    _ => return Err(PorterError::NoInteger),
                };
                let b = match clr[2] {
                    Value::Integer(i) => i as u8,
                    _ => return Err(PorterError::NoInteger),
                };
                let nb: u32 = r as u32 * 256 * 256 + g as u32 * 256 + b as u32;

                format!("#{:x}", nb)
            }
            Value::String(clr) => clr.clone(),
            _ => return Err(PorterError::NoRGB),
        };

        let Ok(colourgrid_html) = data.resource("colorgrid.html") else {
            return Err(PorterError::FileReadError("colorgrid.html".to_string()))
        };
        let colourgrid_html = colourgrid_html.replace("COLOR", &colour_string);

        colours.push(colourgrid_html);
    }

    html::insert(html, &colours, "COLORGRID").expect("Should be able to add colours");
    Ok(())
}

fn insert_images(html: &mut String, item: &Table, data: &Data) -> Result<(), PorterError> {
    let Some(Value::Array(arr)) = item.get("images") else {
        return Ok(())
    };

    let sizes = ["320", "640", "960", "1290", "1920", "2560"];
    let mut images: Vec<String> = Vec::with_capacity(arr.len());

    let Some(Value::String(pre)) = item.get("images_dir") else {
        return Err(PorterError::NoImagesDir)
    };

    for image in arr {
        let Value::String(image_string) = image else {
            return Err(PorterError::WrongType("image".to_string(), "string".to_string()))
        };

        let Ok(mut image_html) = data.resource("image.html") else {
            return Err(PorterError::FileReadError("image.html".to_string()))
        };

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
    handle: AppHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    let table = parse_toml(declaration)?;
    let mut page = html::open_template(template);

    let mut items: Vec<String> = Vec::with_capacity(table.keys().len());

    let data_path = handle.path_resolver().resolve_resource("data").unwrap();
    println!("Data path: {data_path:?}");

    let data_handler = data::Data::new(data_path);

    for key in table.keys() {
        let item = get_item(&table, key).expect("Should be able to get a table for this key...");
        let mut item_html = data_handler.resource("item.html")?;

        item_html = item_html.replace("NAME", key);
        insert(&mut item_html, &item, "year")?;
        insert(&mut item_html, &item, "measurements")?;
        insert(&mut item_html, &item, "description")?;
        insert_colors(&mut item_html, &item, &data_handler)?;
        insert_images(&mut item_html, &item, &data_handler)?;

        items.push(item_html.clone());
    }

    let Ok(_) = html::insert(&mut page, &items.as_slice(), "ITEMS") else {
        return Err(Box::new(PorterError::NoItems))
    };

    html::save(&page, output);
    Ok(())
}
