pub struct PenCanvases<'a> {
    pub pen_canvas: sdl2::render::Texture<'a>,
    pub pen_line_canvas: sdl2::render::Texture<'a>,
}

impl<'a> PenCanvases<'a> {
    pub fn new(
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> PenCanvases<'a> {
        PenCanvases {
            pen_canvas: PenCanvases::create_writable_canvas(texture_creator, canvas),
            pen_line_canvas: PenCanvases::create_writable_canvas(texture_creator, canvas),
        }
    }

    fn create_writable_canvas(
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> sdl2::render::Texture<'a> {
        // Create canvas for pen.
        let mut pen_canvas = texture_creator
            .create_texture_target(None, 800, 600)
            .unwrap();
        pen_canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        // Clear the pen canvas, otherwise it will be black.
        canvas
            .with_texture_canvas(&mut pen_canvas, |texture_canvas| {
                texture_canvas.clear();
            })
            .unwrap();
        pen_canvas
    }
}
