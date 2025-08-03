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
    -a, --anim <ANIM>          The animation name
    -c, --columns <COLUMNS>    The number of columns [default: 0]
    -e, --empty                Include empty frames
    -v, --verbose              Verbose logs
    -j, --json                 Save json file
        --gif                  Save animated gif
        --apng                 Save animated png (APNG)
    -i, --input <INPUT_DIR>    The source input directory [default: .]
    -o, --output <OUTPUT_DIR>  The output directory [default: output]
    -h, --help                 Print help
```


## Tasks
- [x] Implement robust cmdline argument parsing.
- [ ] Handle empty frames.
- [ ] Save JSON file.
- [ ] User-defined input / output directory.
- [ ] Process all cgs files for a given directory.
- [ ] Reference the `data.json` for mapping between character name and ID.
