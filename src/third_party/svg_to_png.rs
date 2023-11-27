/**
 *  svg2colored-png, an svg to png converter
 *  Copyright (C) 2023 MCorange<mcorangecodes@gmail.com>
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *  
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *  
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
/*
 *  Massive thanks to this guy. You can find their SVG to PNG converter
 *  which I modified, at: <https://github.com/MCorange99/svg2colored-png>
 *  - Mrmayman
*/
use std::path::Path;

use usvg_text_layout::TreeTextToPath;

pub fn render(
    input: &Path,
    output: &Path,
    fontdb: &usvg_text_layout::fontdb::Database,
) -> Result<(), String> {
    // Check if the file is actually an SVG.
    match input.extension() {
        Some(e) if e.to_str() == Some("svg") => {}
        Some(_) | None => return Err(format!("Filer {:?} is not of type SVG", input)),
    };

    // Check if output PNG already exists.
    let fo: &Path = &output;
    if fo.exists() {
        return Err("File {fo:?} exists".to_string());
    }

    // Load SVG Data.
    let svg = match std::fs::read_to_string(input) {
        Ok(d) => d,
        Err(_) => return Err("File {fi:?} does not exist".to_string()),
    };

    // Setup USVG Options.
    let mut opt = usvg::Options::default();
    // Get file's absolute directory.
    opt.resources_dir = std::fs::canonicalize(input)
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));

    // Build SVG Tree.
    let mut tree = match usvg::Tree::from_data(svg.as_bytes(), &opt) {
        Ok(v) => v,
        Err(_) => return Err("Failed to parse {fi:?}".to_string()),
    };
    // Render text if needed.
    tree.convert_text(&fontdb);

    // Create Pixel Map to draw SVG to.
    let mut pixmap = tiny_skia::Pixmap::new(200, 200).unwrap();
    // Draw to Pixel Map.
    resvg::render(
        &tree,
        usvg::FitTo::Size(200, 200),
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .expect("Could not render image from svg");
    // Save Pixel Map to PNG.
    pixmap.save_png(fo).expect("Could not save converted PNG");

    Ok(())
}
