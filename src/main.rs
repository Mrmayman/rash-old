use project_state::Renderer;

/**
 *  Rash, a Scratch interpreter written in Rust
 *  Copyright (C) 2023 Mrmayman<navneetkrishna22@gmail.com>
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
mod ansi_codes;
mod costume_loader;
mod interpreter;
mod pen_line;
mod project_state;
mod sprite;
mod thread;

mod project {
    pub mod project_file_loader;
    pub mod project_main;
}

mod bc_compiler {
    pub mod bc_comp_main;
    pub mod bc_comp_optimizer;
    pub mod bc_comp_register_allocator;
    pub mod bc_comp_variable_manager;
}

mod blocks {
    pub mod block_control;
    pub mod block_looks;
    pub mod block_motion;
    pub mod block_operators;
    pub mod block_pen;
    pub mod block_sensing;
    pub mod block_variables;
}

mod third_party {
    pub mod svg_to_png;
}

const FRAME_RATE: f64 = 30.0;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rash", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut last_frame_time = std::time::Instant::now();

    let mut renderer = Renderer::new(&texture_creator, &mut canvas);

    let mut project =
        project::project_main::Project::new(get_project_file_path(), &texture_creator)
            .expect("Could not load project");

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => { /* TODO */ }
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.clear();

        project.run(&mut canvas, &mut renderer);
        project.draw(&mut canvas, &mut renderer);
        canvas.present();

        let elapsed = last_frame_time.elapsed();
        last_frame_time = std::time::Instant::now();
        let frame_time = std::time::Duration::from_secs_f64(1.0 / FRAME_RATE);

        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }
    }
}

fn get_project_file_path() -> String {
    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if an argument (file path) is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // Extract the file path from the arguments
    let file_path = &args[1];
    file_path.clone()
}

fn _calculate_pi() -> f64 {
    let mut pi = 0.0;
    let n = 4.0;
    let mut d = 1.0;
    for i in 0..10_000_000 {
        let a = (2 * (i % 2) - 1) as f64;
        pi += a * n / d;
        d += 2.0;
    }
    pi
}
