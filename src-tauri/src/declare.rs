/*!
* Read the `TOML` declaration file and use html module to create the site.
*/

use std::{fs, path::PathBuf};

use serde::Deserialize;
use tauri::AppHandle;
use toml::{self, Value};

use crate::{
    data::{self, Data},
    error::PorterError,
    html,
};

#[derive(Deserialize, Debug)]
struct Item {
    name: String,
    year: u32,
    measurements: Option<(u32, u32)>,
    images_dir: String,
    images: Vec<String>,
    colours: Vec<String>,
    description: String,
}

impl Item {
    fn to_html(&self, data_handler: &Data) -> Result<String, PorterError> {
        let Ok(mut my_html) = data_handler.resource("item.html") else {
            return Err(PorterError::FileReadError("item.html".to_string()))
        };

        let md = markdown::to_html(&self.description);
        println!("md: {md}");

        my_html = my_html.replace("NAME", &self.name);
        my_html = my_html.replace("MEASUREMENTS", &self.html_measurements());
        my_html = my_html.replace("YEAR", &self.year.to_string());
        my_html = my_html.replace("DESCRIPTION", &md);
        html::insert(&mut my_html, &self.html_images(data_handler)?, "IMAGES")?;
        html::insert(&mut my_html, &self.html_colours(data_handler)?, "COLORGRID")?;
        Ok(my_html)
    }

    fn html_measurements(&self) -> String {
        match self.measurements {
            Some(meas) => {
                format!(
                    "{} <img src=\"images/icons/cross.svg\" width=\"8px\"> {}",
                    meas.0.to_string(),
                    meas.1.to_string()
                )
            }
            None => "-".to_string(),
        }
    }

    fn html_images(&self, data_handler: &Data) -> Result<Vec<String>, PorterError> {
        let sizes = ["320", "640", "960", "1290", "1920", "2560"];
        let mut image_htmls: Vec<String> = Vec::with_capacity(self.images.len());

        for image in self.images.as_slice() {
            let Ok(mut image_html) = data_handler.resource("image.html") else {
                return Err(PorterError::FileReadError("image.html".to_string()))
            };

            // replace default image size
            image_html = image_html.replace(
                "PATH",
                format!("images/{}/resized/{}-2560.webp", self.images_dir, image).as_str(),
            );

            // fill in all of the different sizes
            for size in sizes {
                image_html = image_html.replace(
                    format!("{}W", size).as_str(),
                    format!("images/{}/resized/{}-{}.webp", self.images_dir, image, size).as_str(),
                );
            }

            image_htmls.push(image_html);
        }

        Ok(image_htmls)
    }

    fn html_colours(&self, data_handler: &Data) -> Result<Vec<String>, PorterError> {
        let mut colour_htmls: Vec<String> = Vec::with_capacity(self.colours.len());

        for colour in self.colours.as_slice() {
            let Ok(colourgrid_html) = data_handler.resource("colorgrid.html") else {
                return Err(PorterError::FileReadError("colorgrid.html".to_string()))
            };

            let colourgrid_html = colourgrid_html.replace("COLOR", &colour);

            colour_htmls.push(colourgrid_html);
        }

        Ok(colour_htmls)
    }
}

fn parse_toml(path: PathBuf) -> Result<Vec<Item>, PorterError> {
    let string = match fs::read_to_string(path.clone()) {
        Ok(string) => string,
        Err(_) => {
            return Err(PorterError::FileReadError(
                path.to_str().unwrap().to_string(),
            ))
        }
    };

    let parsed: Result<Value, _> = toml::from_str(&string);
    let parsed: Value = match parsed {
        Err(err) => return Err(PorterError::ParseError(format!("{err}"))),
        Ok(value) => value,
    };

    let Some(table) = parsed.as_table() else {
        return Err(PorterError::ParseError("Couldn't create TOML Table".to_string()))
    };

    let mut error: String = "".to_string();

    let items: Vec<Item> = table
        .values()
        .filter_map(|item| {
            let item: Result<Item, _> = item.clone().try_into();
            match item {
                Err(err) => {
                    error = format!("{err}");
                    None
                }
                Ok(itm) => Some(itm),
            }
        })
        .collect();

    if !error.is_empty() {
        return Err(PorterError::ParseError(format!("{error}")));
    }

    Ok(items)
}

pub fn create_site(
    declaration: PathBuf,
    template: PathBuf,
    output: PathBuf,
    handle: AppHandle,
) -> Result<(), Box<dyn std::error::Error>> {
    let items = parse_toml(declaration)?;
    let mut page = html::open_template(template);

    let mut item_htmls: Vec<String> = Vec::with_capacity(items.len());

    let data_path = handle.path_resolver().resolve_resource("data").unwrap();
    println!("Data path: {data_path:?}");

    let data_handler = data::Data::new(data_path);

    for item in items {
        let item_html = match item.to_html(&data_handler) {
            Ok(result) => result,
            Err(err) => return Err(Box::new(err)),
        };

        item_htmls.push(item_html.clone());
    }

    let Ok(_) = html::insert(&mut page, &item_htmls.as_slice(), "ITEMS") else {
        return Err(Box::new(PorterError::NoItems))
    };

    html::save(&page, output);
    Ok(())
}
