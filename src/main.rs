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
mod interpreter;
mod sprite;
mod thread;

mod project {
    pub mod base;
    pub mod loader;
    pub mod register_allocator;
    pub mod state;
}

mod compile {
    pub mod control;
    pub mod motion;
    pub mod operators;
    pub mod variables;
}

mod third_party {
    pub mod svg_to_png;
}

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

    let mut project = project::base::Project::new(get_project_file_path(), &texture_creator)
        .expect("Could not load project");

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_frame_time = std::time::Instant::now();

    // let _image_context = sdl2::image::init(sdl2::image::InitFlag::)

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.clear();
        project.run();
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 255));
        project.draw(&mut canvas);
        canvas.present();

        let elapsed = last_frame_time.elapsed();
        last_frame_time = std::time::Instant::now();
        let frame_time = std::time::Duration::from_secs_f64(1.0 / 30.0);

        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }
    }

    /*let starting_time = std::time::Instant::now();
    project.run();
    println!(
        "\nRASH:\n{} seconds elapsed, 1 million iterations",
        starting_time.elapsed().as_secs_f64(),
    );

    let starting_time = std::time::Instant::now();
    let pi = _calculate_pi();
    println!(
        "RUST:\n{} seconds elapsed, 1 million iterations\n\npi = {pi}",
        starting_time.elapsed().as_secs_f64(),
    );*/
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
