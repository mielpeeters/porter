/*!
This Module is used to convert images to the desired sizes and WEBP format!
*/

use std::{error::Error, fs::read_dir, path::PathBuf, sync::mpsc, thread};

use tauri::Window;
use walkdir::WalkDir;

use crate::{error::PorterError, imgutil, webper};

fn output_name(dest_dir: PathBuf, dir_name: &str, src_name: &str, width: u32) -> PathBuf {
    let dest = dest_dir.join(dir_name);
    let mut dest = dest.join("resized");

    // get rid of file extension
    let src_name = src_name.split('.').next().unwrap();

    // add the width to the image name, as well as the new file extension
    let image_name = format!("{}-{}.webp", src_name, width);

    dest.push(image_name);

    dest
}

pub fn handle_images(
    src_dir: &PathBuf,
    dest_dir: PathBuf,
    window: Window,
) -> Result<(), Box<dyn Error>> {
    // the widths to convert images to
    let widths: [u32; 6] = [320, 640, 960, 1290, 1920, 2560];

    // the name of the source image directory
    let Some(src_dir_name) = src_dir.file_name() else {
        return Err(Box::new(PorterError::PathConvert("source dir".to_string())))
    };
    let Some(src_dir_name) = src_dir_name.to_str() else {
        return Err(Box::new(PorterError::PathConvert("source dir".to_string())))
    };
    let src_dir_name = src_dir_name.to_string();

    let (tx, rx) = mpsc::channel();

    let total_work = read_dir(src_dir)?.count() * widths.len();

    let mut handles = Vec::new();

    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        let src = entry.path().to_path_buf();
        if entry.depth() != 0 {
            // there is an actual file being handled here (not the directory itself)
            // create a new thread that handles this input image

            // create local copies of strings
            let dest_local = dest_dir.clone();
            let src_local = src_dir_name.clone();
            let local_tx = tx.clone();

            // spawn thread
            let handle = thread::spawn(move || {
                // open the input image
                let img = imgutil::open_image(src.clone()).unwrap();

                // for all widths, resize the image and convert to webp
                for width in widths {
                    let name = output_name(
                        dest_local.clone(),
                        src_local.as_str(),
                        src.file_name().unwrap().to_str().unwrap(),
                        width,
                    );

                    let img_res = imgutil::resize(&img, width).unwrap();

                    let mut resp = true;

                    if !name.exists() {
                        webper::save_to_webp(&img_res, name, 80.0).unwrap();
                        resp = false;
                    }

                    // update receiver about one unit of work being complete
                    let Ok(_) = local_tx.send(resp) else {
                        break;
                    };
                }
            });

            handles.push(handle);
        }
    }

    let mut count = 0;
    for existed in rx {
        count += 1;
        if existed {
            window.emit("skip", count).unwrap();
        } else {
            window.emit("work", (count, total_work)).unwrap();
        }

        if count == total_work {
            break;
        }
    }

    Ok(())
}
