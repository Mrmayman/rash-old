use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
struct Svg {
    #[serde(rename = "width")]
    width: String,

    #[serde(rename = "height")]
    height: String,
    // Add other fields as needed
}

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
    let fo: &Path = output;

    // Load SVG Data.
    let svg = match std::fs::read_to_string(input) {
        Ok(d) => d,
        Err(_) => return Err("File {fi:?} does not exist".to_string()),
    };

    let parsed: Svg = serde_xml_rs::from_str(&svg).unwrap();
    if parsed.width == "0" && parsed.height == "0" {
        let blank_image = image::DynamicImage::new_rgba8(1, 1);
        blank_image.save(output).unwrap();
        return Ok(());
    }

    // Setup USVG Options.
    let opt = usvg::Options {
        // Get file's absolute directory.
        resources_dir: std::fs::canonicalize(input)
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf())),
        ..Default::default()
    };

    // Build SVG Tree.
    let mut tree = match usvg::Tree::from_data(svg.as_bytes(), &opt) {
        Ok(v) => v,
        Err(_) => return Err(format!("Failed to parse {} {svg}", input.to_string_lossy())),
    };
    // Render text if needed.
    tree.convert_text(fontdb);

    // Create Pixel Map to draw SVG to.
    let mut pixmap =
        tiny_skia::Pixmap::new(tree.size.width() as u32, tree.size.height() as u32).unwrap();
    // Draw to Pixel Map.
    resvg::render(
        &tree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .expect("Could not render image from svg");
    // Save Pixel Map to PNG.
    pixmap.save_png(fo).expect("Could not save converted PNG");

    Ok(())
}
