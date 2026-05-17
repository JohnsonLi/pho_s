# photo_sort

A keyboard-driven photo culling tool. Open a folder of photos, browse them with
arrow keys, and use single-letter shortcuts to move each one into a destination
folder. Sidecar `.ARW` (only Sony for now) raw files are moved alongside their JPG automatically.

Supports JPG, JPEG, PNG, WEBP. Displays EXIF metadata (camera, lens, exposure,
ISO, focal length).

## Usage

1. **Open** → **Open Folder** to load a directory of images.
2. Arrow keys navigate (left / right, wraps).
3. **Space** resets zoom and pan.
4. **Scroll wheel** zooms toward the cursor; **click + drag** pans.
5. In the **Destination Folders** panel at the bottom, click **+** to add a
   target folder, then type a single letter in its key field.
6. Press that letter while viewing an image to move it (and its `.ARW` sidecar,
   if present) into that folder.
7. Toggle light/dark with the sun/moon button in the menu bar.

## Installation

Requires [Rust](https://www.rust-lang.org/tools/install) (stable).

```
git clone <repo-url>
cd pho_s
cargo build --release
```

The binary will be at `target/release/phos` (or `phos.exe` on Windows).

## Tests

```
cargo test
```
