# Integration Tests

This directory contains integration tests that verify the end-to-end functionality of ffbetool using real game data.

## Test Files

### `empty_frame_handling.rs`
Tests the empty frame handling functionality using real data from Reisz BS (unit 401012417).

**Key Tests:**
- `test_animation_without_empty_frames()` - Verifies that animations without empty frames produce identical results regardless of the `--empty` flag
- `test_animation_with_empty_frames()` - Verifies that animations with empty frames behave differently with/without the `--empty` flag
- `test_spritesheet_empty_frame_handling()` - Verifies that spritesheets reflect empty frame handling correctly
- `test_frame_108_is_empty()` - Verifies that frame 108 in the CGG data is actually empty
- `test_modified_animation_includes_empty_frame()` - Verifies that the modified animation includes the empty frame

**Test Data:**
- Uses unit 401012417 (Reisz BS) which has a confirmed empty frame at index 108
- Tests both the original `limit_atk` animation (which skips frame 108) and a modified version that includes frame 108

## Test Fixtures

### `fixtures/`
Contains the test data files needed for integration tests:

- `unit_cgg_401012417.csv` - CGG frame data for Reisz BS unit
- `unit_anime_401012417.png` - Source sprite atlas for Reisz BS unit  
- `unit_limit_atk_cgs_401012417.csv` - Original limit attack animation (skips empty frame 108)
- `unit_limit_atk_with_empty_cgs_401012417.csv` - Modified limit attack animation (includes empty frame 108)

## Running Integration Tests

```bash
# Run all integration tests
cargo test

# Run only empty frame handling tests
cargo test --test empty_frame_handling

# Run a specific test
cargo test --test empty_frame_handling test_animation_with_empty_frames
```

## Requirements

Integration tests require:
- The `ffbetool` binary to be built (`cargo build --release`)
- The `identify` command from ImageMagick (for frame counting and dimension checking)

## Test Verification

The integration tests verify that:

1. **Empty frame detection works correctly** - Frame 108 is properly identified as empty
2. **Filtering works correctly** - Empty frames are excluded when `--empty` flag is not used
3. **Inclusion works correctly** - Empty frames are included and properly sized when `--empty` flag is used
4. **Output differences are measurable** - File sizes and frame counts differ appropriately
5. **Spritesheet dimensions reflect frame count** - Spritesheets are wider when empty frames are included
