# FFBETOOL
Tool to assemble **Final Fantasy Brave Exvius** sprite sheets.

Takes the master sprite atlas `png` file and uses information from `csv` files
to assemble the spritesheet.

## Features
- **Unit ID Support**: Use traditional numeric unit IDs for precise character selection
- **Character Name Support**: Use character names instead of unit IDs for convenience
- **Auto-discovery**: Automatically finds and processes all animations for a unit
- **Multiple Output Formats**: Generate spritesheets, animated GIFs, APNGs, and JSON metadata
- **Flexible Layout**: Control spritesheet columns and include empty frames
- **Smart Matching**: Case-insensitive character name lookup with partial matching

## Download and Installation
Go to the [Releases Page](https://github.com/dsxragnarok/ffbetool/releases) and download the package for your platform. Extract the binary to your preferred location. Then run the executable from the terminal.

## Usage
```bash
Usage: ffbetool [OPTIONS] <UID>

Arguments:
    <UID>  The unit id or character name

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

## Setup

### Character Database
To use character names, place a `character_data.json` file in your working directory. The file should contain character mappings in this format:

```json
{
    "100000102": {"type": "story", "name": "Rain", "rarity": ""},
    "100000202": {"type": "story", "name": "Lasswell", "rarity": ""},
    "100000302": {"type": "story", "name": "Fina", "rarity": ""}
}
```

## Examples

### Using Character Names
```bash
# Process all animations for Rain
ffbetool "Rain" -i input/ -o output/

# Process specific animation using character name
ffbetool "Lightning" -a atk -i input/ -o output/

# Case-insensitive matching works too
ffbetool "CECIL" -i input/ -o output/ --gif
```

### Using Unit IDs (Traditional)
```bash
# Auto-discovers and processes all CGS animation files for unit 401012417
ffbetool 401012417 -i input/ -o output/

# Process only the "atk" animation for unit 401012417
ffbetool 401012417 -a atk -i input/ -o output/
```

### Generate animated outputs
```bash
# Generate animated GIFs for all animations
ffbetool "Rain" -i input/ -o output/ --gif

# Generate animated PNGs (APNG) for all animations
ffbetool 401012417 -i input/ -o output/ --apng
```

### Generate JSON metadata
```bash
# Generate JSON metadata files for all animations
ffbetool "Lightning" -i input/ -o output/ --json

# Generate JSON metadata for specific animation
ffbetool 401012417 -a limit_atk -i input/ -o output/ --json

# Combine JSON with animated outputs
ffbetool "Cecil" -i input/ -o output/ --json --gif
```

### Include empty frames in animations
```bash
# Include empty frames in the output (useful for maintaining timing)
ffbetool "Rain" -a limit_atk -i input/ -o output/ --empty --gif
```

### Handling Multiple Matches
When a character name matches multiple characters, ffbetool will show suggestions:

```bash
$ ffbetool "Light"
Did you mean one of the following? Try again with the associated uid.

    213000105 -> Lightning
    213001005 -> Radiant Lightning
    250000205 -> Lightning (FFXIII-2)
    258000207 -> Savior of Souls Lightning
```

## Character Name Matching

The tool supports flexible character name matching:

- **Exact Match**: `"Rain"` matches "Rain"
- **Case-Insensitive**: `"RAIN"` matches "Rain"
- **Partial Match**: `"Light"` matches "Lightning", "Radiant Lightning", etc.

When multiple matches are found, the tool will display all options with their unit IDs for disambiguation.

## Output Files

The tool generates various output files based on the options specified:

- **Spritesheet**: `{unit_id}-{animation}.png` - The main spritesheet image
- **Animated GIF**: `{unit_id}-{animation}-anim.gif` - Animated version (with `--gif`)
- **Animated PNG**: `{unit_id}-{animation}-anim.png` - APNG format (with `--apng`)
- **JSON Metadata**: `{unit_id}-{animation}.json` - Frame timing and layout data (with `--json`)

## Tasks
- [x] Implement robust cmdline argument parsing.
- [x] Handle empty frames.
- [x] User-defined input / output directory.
- [x] Process all cgs files for a given directory.
- [x] Save JSON file.
- [x] Reference the `character_data.json` for mapping between character name and ID.
- [x] Character name lookup with case-insensitive and partial matching.
- [x] Multiple match handling with user-friendly suggestions.

## Defects
- [x] Improper rotation handling of -270 and -180. [ref: 100004403_limit_atk](https://github.com/dsxragnarok/ffbe_asset_dump/blob/main/animated_gifs/unit_100004403_limitatk_opac.gif)
