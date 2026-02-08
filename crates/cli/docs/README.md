# HKXC-ANNO-CLI

## HKX Animation Annotation Tool

## OVERVIEW

`hkxc-anno-cli` is a command-line tool for extracting and updating animation annotations in Havok HKX files used by LE and SSE Skyrim. It wraps the `hkxc.exe` converter to provide a streamlined workflow for managing animation event annotations.

Animation annotations are time-stamped events embedded in HKX files that trigger game behaviors like weapon swings, hit detection, sound effects, and state changes during animation playback.

## REQUIREMENTS

`hkxc.exe` must be installed and available in your system PATH

## COMMANDS

### 1. DUMP - Extract annotations from an HKX file to a text file

**Usage:**

```bash
hkxc-anno-cli dump -i <INPUT_FILE> [-o <OUTPUT>]
```

**Options:**

- `-i, --input <INPUT>` - Input HKX file (single file only)
- `-o, --output <OUTPUT>` - Output annotation file path (optional)
  - Default: same directory as input, with `.txt` extension

**Examples:**

```bash
# Dump to same directory as input (creates attack.txt next to attack.hkx)
hkxc-anno-cli dump -i attack.hkx

# Dump to specific output file
hkxc-anno-cli dump -i attack.hkx -o my_annotations.txt
```

### 2. UPDATE - Apply annotations from a text file to an HKX file

**Usage:**

```bash
hkxc-anno-cli update -i <INPUT_FILE> [-a <ANNO_FILE>] [-v <FORMAT>] [-o <OUTPUT>]
```

**Options:**

- `-i, --input <INPUT>` - Input HKX file (single file only)
- `-a, --anno <ANNO>` - Annotation file to apply (optional)
  - Default: same directory as input, with `.txt` extension
- `-v, --format <FORMAT>` - Output format: `amd64` or `win32` (default: `amd64`)
- `-o, --output <OUTPUT>` - Output HKX file path (optional, overwrites input if omitted)

**Examples:**

```bash
# Update using annotation file in same directory (attack.txt)
hkxc-anno-cli update -i attack.hkx

# Update with specific annotation file
hkxc-anno-cli update -i attack.hkx -a custom_annos.txt

# Update and output to different file
hkxc-anno-cli update -i attack.hkx -o modified_attack.hkx

# Output in 32-bit format for Skyrim Legendary Edition
hkxc-anno-cli update -i attack.hkx -v win32
```

## ANNOTATION FILE FORMAT

Annotation text files use a simple format:

```shell
# Comment lines start with #
# Metadata is included as comments:
# duration: 3.166667
# numberOfTransformTracks: 99
# annotations numelements: 20
#
0.000000 tailCombatState
0.033333 CastOkStart
0.400000 WeaponSwing
0.466667 Collision_Add.node(WEAPON)
0.500000 preHitFrame
0.533333 HitFrame
0.600000 Collision_Remove.node(WEAPON)
1.000000 preHitFrame
3.166667 attackStop
```

Each non-comment line contains:

```txt
<time_in_seconds> <annotation_text>
```

## WORKFLOW EXAMPLE

Edit an animation's annotations:

1. **Extract annotations** from HKX file (creates `attack.txt` in same folder):

   ```bash
   hkxc-anno-cli dump -i attack.hkx
   ```

2. **Edit the annotation file** (`attack.txt`) with a text editor:
   - Modify timing values
   - Add or remove events
   - Copy events from other animation files

3. **Apply the modified annotations** back to the HKX:

   ```bash
   hkxc-anno-cli update -i attack.hkx -a attack.txt
   ```

## OUTPUT FORMATS

- `-v amd64` - 64-bit format for Skyrim Special Edition / Anniversary Edition (default)
- `-v win32` - 32-bit format for Skyrim Legendary Edition

## TROUBLESHOOTING

- **"Failed to execute hkxc"** - Ensure `hkxc.exe` is in your system PATH
- **"Annotation file not found"** - Check that `.txt` file exists in the same directory as the input HKX file
- **"XML file not found"** - The HKX file may be corrupted or not a valid animation file
