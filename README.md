# FFBETOOL
Tool to assemble **Final Fantasy Brave Exvius** sprite sheets.

Takes the master sprite atlas `png` file and uses information from `cvs` files
to assemble the spritesheet.

## Usage
```bash
Usage: ffbetool [OPTIONS] <UID>

Arguments:
    <UID>  The unit id

Options:
    -a, --anim <ANIM>          The animation name (if not specified, all animations will be processed)
    -c, --columns <COLUMNS>    The number of columns [default: 0]
    -e, --empty                Include empty frames
    -v, --verbose              Verbose logs
    -j, --json                 Save json file
        --gif                  Save animated gif
        --apng                 Save animated png (APNG)
    -i, --input <INPUT_DIR>    The source input directory [default: .]
    -o, --output <OUTPUT_DIR>  The output directory [default: .]
    -h, --help                 Print help
```

## Examples

### Process all animations for a unit
```bash
# Auto-discovers and processes all CGS animation files for unit 401012417
ffbetool 401012417 -i input/ -o output/
```

### Process a specific animation
```bash
# Process only the "atk" animation for unit 401012417
ffbetool 401012417 -a atk -i input/ -o output/
```

### Generate animated outputs
```bash
# Generate animated GIFs for all animations
ffbetool 401012417 -i input/ -o output/ --gif

# Generate animated PNGs (APNG) for all animations
ffbetool 401012417 -i input/ -o output/ --apng
```

### Generate JSON metadata
```bash
# Generate JSON metadata files for all animations
ffbetool 401012417 -i input/ -o output/ --json

# Generate JSON metadata for specific animation
ffbetool 401012417 -a limit_atk -i input/ -o output/ --json

# Combine JSON with animated outputs
ffbetool 401012417 -i input/ -o output/ --json --gif
```

### Include empty frames in animations
```bash
# Include empty frames in the output (useful for maintaining timing)
ffbetool 401012417 -a limit_atk -i input/ -o output/ --empty --gif
```

## Tasks
- [x] Implement robust cmdline argument parsing.
- [x] Handle empty frames.
- [x] User-defined input / output directory.
- [x] Process all cgs files for a given directory.
- [x] Save JSON file.
- [ ] Reference the `data.json` for mapping between character name and ID.

## Defects
- [x] Improper rotation handling of -270 and -180. [ref: 100004403_limit_atk](https://github.com/dsxragnarok/ffbe_asset_dump/blob/main/animated_gifs/unit_100004403_limitatk_opac.gif)
