use crate::{FfbeError, Result};
use std::fs;
use std::path::Path;

/// Represents a discovered animation for a unit
#[derive(Debug, Clone)]
pub struct DiscoveredAnimation {
    pub name: String,
    pub file_path: String,
}

/// Discovers all CGS animation files for a given unit in the specified directory
pub fn discover_animations(unit_id: u32, input_dir: &str) -> Result<Vec<DiscoveredAnimation>> {
    let input_path = Path::new(input_dir);

    if !input_path.exists() {
        return Err(FfbeError::FileNotFound(format!(
            "Input directory '{}'",
            input_dir
        )));
    }

    let mut animations = Vec::new();

    let entries = fs::read_dir(input_path).map_err(|err| FfbeError::IoError(err))?;

    for entry in entries {
        let entry = entry.map_err(|err| FfbeError::IoError(err))?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        // Look for files matching pattern: unit_{animation_name}_cgs_{unit_id}.csv
        if let Some(animation_name) = extract_animation_name(&file_name, unit_id) {
            animations.push(DiscoveredAnimation {
                name: animation_name,
                file_path: entry.path().to_string_lossy().to_string(),
            });
        }
    }

    // Sort animations by name for consistent output
    animations.sort_by(|a, b| a.name.cmp(&b.name));

    if animations.is_empty() {
        return Err(FfbeError::FileNotFound(format!(
            "No CGS animation files found for unit {} in directory '{}'",
            unit_id, input_dir
        )));
    }

    Ok(animations)
}

/// Extracts animation name from CGS filename
/// Expected format: unit_{animation_name}_cgs_{unit_id}.csv
fn extract_animation_name(filename: &str, unit_id: u32) -> Option<String> {
    if !filename.ends_with(".csv") {
        return None;
    }

    let expected_prefix = "unit_";
    let expected_suffix = format!("_cgs_{}.csv", unit_id);

    if filename.starts_with(expected_prefix) && filename.ends_with(&expected_suffix) {
        // Extract the middle part (animation name)
        let start = expected_prefix.len();
        let end = filename.len() - expected_suffix.len();

        if end > start {
            let animation_name = &filename[start..end];
            return Some(animation_name.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_extract_animation_name() {
        // Valid cases
        assert_eq!(
            extract_animation_name("unit_atk_cgs_123.csv", 123),
            Some("atk".to_string())
        );
        assert_eq!(
            extract_animation_name("unit_limit_atk_cgs_456.csv", 456),
            Some("limit_atk".to_string())
        );
        assert_eq!(
            extract_animation_name("unit_magic_standby_cgs_789.csv", 789),
            Some("magic_standby".to_string())
        );

        // Invalid cases
        assert_eq!(extract_animation_name("unit_atk_cgs_123.csv", 456), None); // Wrong unit ID
        assert_eq!(extract_animation_name("unit_atk_cgs_123.txt", 123), None); // Wrong extension
        assert_eq!(extract_animation_name("other_atk_cgs_123.csv", 123), None); // Wrong prefix
        assert_eq!(extract_animation_name("unit_atk_other_123.csv", 123), None); // Wrong pattern
        assert_eq!(extract_animation_name("unit__cgs_123.csv", 123), None); // Empty animation name
    }

    #[test]
    fn test_discover_animations_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().to_str().unwrap();

        let result = discover_animations(123, temp_path);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No CGS animation files found")
        );
    }

    #[test]
    fn test_discover_animations_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path().to_str().unwrap();

        // Create test CGS files
        fs::write(
            format!("{}/unit_atk_cgs_123.csv", temp_path),
            "test content",
        )
        .unwrap();
        fs::write(
            format!("{}/unit_idle_cgs_123.csv", temp_path),
            "test content",
        )
        .unwrap();
        fs::write(
            format!("{}/unit_limit_atk_cgs_123.csv", temp_path),
            "test content",
        )
        .unwrap();

        // Create some files that should be ignored
        fs::write(format!("{}/unit_atk_cgs_456.csv", temp_path), "wrong unit").unwrap();
        fs::write(format!("{}/other_file.txt", temp_path), "not a cgs file").unwrap();
        fs::write(format!("{}/unit_cgg_123.csv", temp_path), "cgg file").unwrap();

        let result = discover_animations(123, temp_path).unwrap();

        assert_eq!(result.len(), 3);

        // Should be sorted alphabetically
        assert_eq!(result[0].name, "atk");
        assert_eq!(result[1].name, "idle");
        assert_eq!(result[2].name, "limit_atk");

        // Check file paths
        assert!(result[0].file_path.contains("unit_atk_cgs_123.csv"));
        assert!(result[1].file_path.contains("unit_idle_cgs_123.csv"));
        assert!(result[2].file_path.contains("unit_limit_atk_cgs_123.csv"));
    }

    #[test]
    fn test_discover_animations_nonexistent_directory() {
        let result = discover_animations(123, "nonexistent_directory");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Input directory"));
    }
}
