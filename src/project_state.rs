use sdl2::pixels::{Color, PixelFormatEnum};

pub struct ProjectState<'a> {
    pub main_canvas: sdl2::render::Texture<'a>,
    pub pen_line_canvas: sdl2::render::Texture<'a>,
    pub scratch_timer: std::time::Instant,
}

impl<'a> ProjectState<'a> {
    pub fn new(
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> ProjectState<'a> {
        let mut this = ProjectState {
            main_canvas: ProjectState::create_writable_canvas(texture_creator, canvas),
            pen_line_canvas: ProjectState::create_writable_canvas(texture_creator, canvas),
            scratch_timer: std::time::Instant::now(),
        };
        this.update_pen_line_properties(canvas);
        this
    }

    pub fn update_pen_line_properties(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) {
        canvas
            .with_texture_canvas(&mut self.pen_line_canvas, |texture_canvas| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 255));
                texture_canvas.clear();
            })
            .unwrap();
    }

    fn create_writable_canvas(
        texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> sdl2::render::Texture<'a> {
        // Create canvas for pen.
        let mut pen_canvas = texture_creator
            .create_texture_target(PixelFormatEnum::RGBA8888, 800, 600)
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
