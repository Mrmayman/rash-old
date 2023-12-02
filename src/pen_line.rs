use sdl2::rect::{Point, Rect};

use crate::{
    project::base::get_scaled_point, project_state::ProjectState, sprite::GraphicalProperties,
};

pub fn draw(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    pen_canvas: &mut ProjectState<'_>,
    properties: &mut GraphicalProperties,
    new_x: f64,
    new_y: f64,
) {
    if !properties.pen_down {
        return;
    }
    let query = canvas.output_size().unwrap();
    canvas
        .with_texture_canvas(&mut pen_canvas.main_canvas, |texture_canvas| {
            let start_position = get_scaled_point((properties.x, properties.y), query);
            let end_position = get_scaled_point((new_x, new_y), query);

            let distance = (((end_position.0 - start_position.0).pow(2)
                + (end_position.1 - start_position.1).pow(2)) as f64)
                .sqrt();

            if properties.pen_radius > 2 {
                draw_circle_edges(properties, texture_canvas, start_position, end_position);
            }

            if distance > properties.pen_radius as f64 {
                let angle = calculate_direction(new_x, properties, new_y);

                let line = get_rect(start_position, properties, distance);

                texture_canvas
                    .copy_ex(
                        &pen_canvas.pen_line_canvas,
                        None,
                        line,
                        angle,
                        Point::new(properties.pen_radius / 2, 0),
                        false,
                        false,
                    )
                    .unwrap();
            }
        })
        .unwrap();
}

fn get_rect(
    start_position: (i32, i32),
    properties: &mut GraphicalProperties,
    distance: f64,
) -> Rect {
    let line_start_x = start_position.0 as i32 - (properties.pen_radius / 2);
    let line_start_y = start_position.1 as i32;

    let line = Rect::new(
        line_start_x,
        line_start_y,
        properties.pen_radius as u32,
        distance as u32,
    );
    line
}

fn calculate_direction(new_x: f64, properties: &mut GraphicalProperties, new_y: f64) -> f64 {
    let mut angle = 180.0
        + (((new_x - properties.x) / (new_y - properties.y)).atan()
            * (180.0 / std::f64::consts::PI));
    if new_y - properties.y < 0.0 {
        angle -= 180.0
    }
    angle = angle.rem_euclid(360.0);
    angle
}

fn draw_circle_edges(
    properties: &mut GraphicalProperties,
    texture_canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    sprite_rect: (i32, i32),
    new_scaled_rect: (i32, i32),
) {
    for x2 in -(properties.pen_radius / 2)..(properties.pen_radius / 2) {
        for y2 in -(properties.pen_radius / 2)..(properties.pen_radius / 2) {
            if (x2 * x2 + y2 * y2) < ((properties.pen_radius * properties.pen_radius) / 4) {
                texture_canvas.set_draw_color(properties.pen_color);
                texture_canvas
                    .draw_point(Point::new(sprite_rect.0 + x2, sprite_rect.1 + y2))
                    .unwrap();
                texture_canvas
                    .draw_point(Point::new(new_scaled_rect.0 + x2, new_scaled_rect.1 + y2))
                    .unwrap();
            }
        }
    }
}
