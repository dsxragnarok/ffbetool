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

    // Validate animation file if specified (for single animation mode)
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
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_validate_input_args_zero_uid() {
        let result = validate_input_args(0, "test_data", Some("atk"));
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Unit ID must be greater than 0")
        );
    }

    #[test]
    fn test_validate_input_args_nonexistent_dir() {
        let result = validate_input_args(12345, "nonexistent_dir", Some("atk"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Input directory"));
    }

    #[test]
    fn test_validate_input_args_missing_atlas() {
        let result = validate_input_args(99999, "test_data", Some("atk"));
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("unit_anime_99999.png")
        );
    }

    #[test]
    fn test_validate_input_args_missing_cgg() {
        // Create a temporary directory with just the atlas file
        let temp_dir = "temp_test_dir";
        fs::create_dir_all(temp_dir).unwrap();

        // Create a dummy atlas file
        fs::write(format!("{}/unit_anime_12345.png", temp_dir), b"dummy").unwrap();

        let result = validate_input_args(12345, temp_dir, Some("atk"));
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("unit_cgg_12345.csv")
        );

        // Clean up
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_validate_input_args_missing_cgs() {
        // Create a temporary directory with atlas and cgg files
        let temp_dir = "temp_test_dir2";
        fs::create_dir_all(temp_dir).unwrap();

        fs::write(format!("{}/unit_anime_12345.png", temp_dir), b"dummy").unwrap();
        fs::write(format!("{}/unit_cgg_12345.csv", temp_dir), b"dummy").unwrap();

        let result = validate_input_args(12345, temp_dir, Some("nonexistent_anim"));
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("unit_nonexistent_anim_cgs_12345.csv")
        );

        // Clean up
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_validate_input_args_valid() {
        let result = validate_input_args(204000103, "test_data", Some("atk"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_input_args_no_anim() {
        let result = validate_input_args(204000103, "test_data", None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_output_dir_existing() {
        let result = validate_output_dir("test_data");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_output_dir_create_new() {
        let temp_dir = "temp_output_dir";

        // Make sure it doesn't exist first
        let _ = fs::remove_dir_all(temp_dir);

        let result = validate_output_dir(temp_dir);
        assert!(result.is_ok());
        assert!(Path::new(temp_dir).exists());
        assert!(Path::new(temp_dir).is_dir());

        // Clean up
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_validate_output_dir_file_exists() {
        let temp_file = "temp_file_not_dir";

        // Create a file with the same name
        fs::write(temp_file, b"dummy").unwrap();

        let result = validate_output_dir(temp_file);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("is not a directory")
        );

        // Clean up
        fs::remove_file(temp_file).unwrap();
    }
}
