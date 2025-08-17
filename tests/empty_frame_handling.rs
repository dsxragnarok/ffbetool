use std::process::Command;
use std::fs;
use tempfile::TempDir;

const REISZ_UNIT_ID: u32 = 401012417;
const FIXTURES_DIR: &str = "tests/fixtures";

/// Test that animations without empty frames work the same regardless of --empty flag
#[test]
fn test_animation_without_empty_frames() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_str().unwrap();
    
    // Test without --empty flag
    let output1 = Command::new("./target/release/ffbetool")
        .args([
            &REISZ_UNIT_ID.to_string(),
            "-a", "limit_atk",
            "-i", FIXTURES_DIR,
            "-o", temp_path,
            "--gif"
        ])
        .output()
        .expect("Failed to execute ffbetool");
    
    assert!(output1.status.success(), "Command failed: {}", String::from_utf8_lossy(&output1.stderr));
    
    let gif1_path = format!("{}/{}-limit_atk-anim.gif", temp_path, REISZ_UNIT_ID);
    assert!(fs::metadata(&gif1_path).is_ok(), "GIF file not created");
    let gif1_size = fs::metadata(&gif1_path).unwrap().len();
    
    // Remove the file for second test
    fs::remove_file(&gif1_path).unwrap();
    
    // Test with --empty flag
    let output2 = Command::new("./target/release/ffbetool")
        .args([
            &REISZ_UNIT_ID.to_string(),
            "-a", "limit_atk", 
            "-i", FIXTURES_DIR,
            "-o", temp_path,
            "--gif",
            "--empty"
        ])
        .output()
        .expect("Failed to execute ffbetool");
    
    assert!(output2.status.success(), "Command failed: {}", String::from_utf8_lossy(&output2.stderr));
    
    let gif2_path = format!("{}/{}-limit_atk-anim.gif", temp_path, REISZ_UNIT_ID);
    assert!(fs::metadata(&gif2_path).is_ok(), "GIF file not created");
    let gif2_size = fs::metadata(&gif2_path).unwrap().len();
    
    // Both should be identical since limit_atk doesn't reference the empty frame 108
    assert_eq!(gif1_size, gif2_size, "GIF files should be identical when no empty frames are referenced");
}

/// Test that animations with empty frames behave differently with --empty flag
#[test]
fn test_animation_with_empty_frames() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_str().unwrap();
    
    // Test without --empty flag (should exclude empty frame)
    let output1 = Command::new("./target/release/ffbetool")
        .args([
            &REISZ_UNIT_ID.to_string(),
            "-a", "limit_atk_with_empty",
            "-i", FIXTURES_DIR,
            "-o", temp_path,
            "--gif"
        ])
        .output()
        .expect("Failed to execute ffbetool");
    
    assert!(output1.status.success(), "Command failed: {}", String::from_utf8_lossy(&output1.stderr));
    
    let gif1_path = format!("{}/{}-limit_atk_with_empty-anim.gif", temp_path, REISZ_UNIT_ID);
    assert!(fs::metadata(&gif1_path).is_ok(), "GIF file not created");
    let gif1_size = fs::metadata(&gif1_path).unwrap().len();
    
    // Count frames in first GIF
    let identify1 = Command::new("identify")
        .arg(&gif1_path)
        .output()
        .expect("Failed to run identify command");
    let frame_count1 = String::from_utf8_lossy(&identify1.stdout).lines().count();
    
    // Remove the file for second test
    fs::remove_file(&gif1_path).unwrap();
    
    // Test with --empty flag (should include empty frame)
    let output2 = Command::new("./target/release/ffbetool")
        .args([
            &REISZ_UNIT_ID.to_string(),
            "-a", "limit_atk_with_empty",
            "-i", FIXTURES_DIR, 
            "-o", temp_path,
            "--gif",
            "--empty"
        ])
        .output()
        .expect("Failed to execute ffbetool");
    
    assert!(output2.status.success(), "Command failed: {}", String::from_utf8_lossy(&output2.stderr));
    
    let gif2_path = format!("{}/{}-limit_atk_with_empty-anim.gif", temp_path, REISZ_UNIT_ID);
    assert!(fs::metadata(&gif2_path).is_ok(), "GIF file not created");
    let gif2_size = fs::metadata(&gif2_path).unwrap().len();
    
    // Count frames in second GIF
    let identify2 = Command::new("identify")
        .arg(&gif2_path)
        .output()
        .expect("Failed to run identify command");
    let frame_count2 = String::from_utf8_lossy(&identify2.stdout).lines().count();
    
    // With --empty should have one more frame
    assert_eq!(frame_count2, frame_count1 + 1, "Animation with --empty should have one more frame");
    
    // With --empty should have larger file size (due to additional frame)
    assert!(gif2_size > gif1_size, "Animation with --empty should have larger file size");
}

/// Test that spritesheets reflect empty frame handling correctly
#[test]
fn test_spritesheet_empty_frame_handling() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_str().unwrap();
    
    // Test without --empty flag
    let output1 = Command::new("./target/release/ffbetool")
        .args([
            &REISZ_UNIT_ID.to_string(),
            "-a", "limit_atk_with_empty",
            "-i", FIXTURES_DIR,
            "-o", temp_path
        ])
        .output()
        .expect("Failed to execute ffbetool");
    
    assert!(output1.status.success(), "Command failed: {}", String::from_utf8_lossy(&output1.stderr));
    
    let sheet1_path = format!("{}/{}-limit_atk_with_empty.png", temp_path, REISZ_UNIT_ID);
    assert!(fs::metadata(&sheet1_path).is_ok(), "Spritesheet not created");
    
    // Get dimensions of first spritesheet
    let identify1 = Command::new("identify")
        .arg(&sheet1_path)
        .output()
        .expect("Failed to run identify command");
    let dimensions1 = String::from_utf8_lossy(&identify1.stdout);
    let width1 = extract_width_from_identify(&dimensions1);
    
    // Remove the file for second test
    fs::remove_file(&sheet1_path).unwrap();
    
    // Test with --empty flag
    let output2 = Command::new("./target/release/ffbetool")
        .args([
            &REISZ_UNIT_ID.to_string(),
            "-a", "limit_atk_with_empty",
            "-i", FIXTURES_DIR,
            "-o", temp_path,
            "--empty"
        ])
        .output()
        .expect("Failed to execute ffbetool");
    
    assert!(output2.status.success(), "Command failed: {}", String::from_utf8_lossy(&output2.stderr));
    
    let sheet2_path = format!("{}/{}-limit_atk_with_empty.png", temp_path, REISZ_UNIT_ID);
    assert!(fs::metadata(&sheet2_path).is_ok(), "Spritesheet not created");
    
    // Get dimensions of second spritesheet
    let identify2 = Command::new("identify")
        .arg(&sheet2_path)
        .output()
        .expect("Failed to run identify command");
    let dimensions2 = String::from_utf8_lossy(&identify2.stdout);
    let width2 = extract_width_from_identify(&dimensions2);
    
    // Spritesheet with --empty should be wider (one more frame)
    assert!(width2 > width1, "Spritesheet with --empty should be wider due to additional frame");
}

/// Helper function to extract width from identify command output
fn extract_width_from_identify(output: &str) -> u32 {
    // identify output format: "filename PNG 1234x567 8-bit/color RGBA, non-interlaced"
    for line in output.lines() {
        if let Some(dimensions_part) = line.split_whitespace().nth(2) {
            if let Some(width_str) = dimensions_part.split('x').next() {
                if let Ok(width) = width_str.parse::<u32>() {
                    return width;
                }
            }
        }
    }
    panic!("Could not extract width from identify output: {}", output);
}

/// Test that verifies frame 108 is actually empty in the CGG data
#[test]
fn test_frame_108_is_empty() {
    let cgg_content = fs::read_to_string("tests/fixtures/unit_cgg_401012417.csv")
        .expect("Failed to read CGG file");
    
    let lines: Vec<&str> = cgg_content.lines().collect();
    
    // Frame 108 should be at line 109 (0-indexed + 1)
    assert!(lines.len() > 108, "CGG file should have more than 108 lines");
    
    let frame_108_line = lines[108];
    
    // Frame 108 should be empty (format: "0,0,")
    assert_eq!(frame_108_line.trim(), "0,0,", "Frame 108 should be empty (0,0,)");
}

/// Test that verifies the modified animation includes frame 108
#[test]
fn test_modified_animation_includes_empty_frame() {
    let cgs_content = fs::read_to_string("tests/fixtures/unit_limit_atk_with_empty_cgs_401012417.csv")
        .expect("Failed to read modified CGS file");
    
    let frame_108_referenced = cgs_content.lines()
        .any(|line| line.trim().starts_with("108,"));
    
    assert!(frame_108_referenced, "Modified animation should reference frame 108");
}
