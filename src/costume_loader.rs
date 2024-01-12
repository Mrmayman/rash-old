use sdl2::image::LoadTexture;

use crate::project::project_main::Project;

pub fn load_png<'a>(
    texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    temp_project: &Project<'_>,
    costume: &serde_json::Value,
) -> Result<sdl2::render::Texture<'a>, String> {
    texture_creator.load_texture(
        temp_project
            .path
            .join(costume["assetId"].as_str().unwrap().to_string() + ".png"),
    )
}

pub fn convert_svg_to_png(
    costume_json: &serde_json::Value,
    project: &Project<'_>,
    font_database: &usvg_text_layout::fontdb::Database,
) -> Result<(), String> {
    crate::third_party::svg_to_png::render(
        project
            .path
            .join(costume_json["md5ext"].as_str().unwrap())
            .as_ref(),
        project
            .path
            .join(costume_json["assetId"].as_str().unwrap().to_string() + ".png")
            .as_ref(),
        font_database,
    )
}
