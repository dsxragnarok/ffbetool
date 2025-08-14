use crate::{FfbeError, Result};
use std::path::Path;

pub fn validate_input_args(uid: u32, input_dir: &str, anim_name: Option<&str>) -> Result<()> {
    // Validate unit ID
    if uid == 0 {
        return Err(FfbeError::InvalidInput(
            "Unit ID must be greater than 0".to_string(),
        ));
    }

    // Validate input directory exists
    if !Path::new(input_dir).exists() {
        return Err(FfbeError::FileNotFound(format!(
            "Input directory '{}'",
            input_dir
        )));
    }

    // Validate required files exist
    let atlas_file = format!("{}/unit_anime_{}.png", input_dir, uid);
    if !Path::new(&atlas_file).exists() {
        return Err(FfbeError::FileNotFound(atlas_file));
    }

    let cgg_file = format!("{}/unit_cgg_{}.csv", input_dir, uid);
    if !Path::new(&cgg_file).exists() {
        return Err(FfbeError::FileNotFound(cgg_file));
    }

    // Validate animation file if specified
    if let Some(anim) = anim_name {
        let cgs_file = format!("{}/unit_{}_cgs_{}.csv", input_dir, anim, uid);
        if !Path::new(&cgs_file).exists() {
            return Err(FfbeError::FileNotFound(cgs_file));
        }
    }

    Ok(())
}

pub fn validate_output_dir(output_dir: &str) -> Result<()> {
    let path = Path::new(output_dir);

    if !path.exists() {
        std::fs::create_dir_all(path).map_err(FfbeError::IoError)?;
    }

    if !path.is_dir() {
        return Err(FfbeError::InvalidInput(format!(
            "Output path '{}' is not a directory",
            output_dir
        )));
    }

    Ok(())
}
