extern crate sdl2;

use std::thread;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::time::{Duration, Instant};
use sdl2::mouse::MouseButton;


const NUM_PARTICLES: usize = 1000000;



fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let display_mode = video_subsystem.desktop_display_mode(0).unwrap();
    let window = video_subsystem.window("SDL2 Fullscreen", display_mode.w as u32, display_mode.h as u32).fullscreen().build().unwrap();

    // Main event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    


    // main variables
    let mut scene = Scene {
        width: window.size().0,
        height: window.size().1,
        pos_x: 0.0,
        pos_y: 0.0,
        canvas: window.into_canvas().build().unwrap(),
        zoom_factor: 0.0,
        zoom_level: -4.4,
    };
    update_zoom(&mut scene);
    scene.canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let mut mouse = Mouse {
        pos_x: 0.0,
        pos_y: 0.0,
        mouse_state: sdl2::mouse::MouseState::new(&event_pump),
        locked: false,
    };

    let mut particles: Vec<Particle> = Vec::with_capacity(NUM_PARTICLES);

    set_shape(&mut particles, 10.0, 0);



    // lock on particle
    let mut follow_particle: usize = 0;
    let mut follows_particle = false;



    // FPS
    let mut previous_frame_time = Instant::now();
    let frame_duration = Duration::from_secs_f64(1.0 / 60.0);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }

                Event::MouseWheel { y, .. } => {
                    if y > 0 {
                        scene.zoom_level += 0.2;
                    } else {
                        scene.zoom_level -= 0.2;
                    }
                    let start_mouse_position = canvas_to_scene(&scene, (mouse.mouse_state.x(), mouse.mouse_state.y()));
                    update_zoom(&mut scene);
                    let new_mouse_position = canvas_to_scene(&scene, (mouse.mouse_state.x(), mouse.mouse_state.y()));
                    scene.pos_x += start_mouse_position.0 - new_mouse_position.0;
                    scene.pos_y += start_mouse_position.1 - new_mouse_position.1;
                }

                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    set_shape(&mut particles, 10.0, 0)
                }

                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    mouse.locked = !mouse.locked;
                }

                Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                    for particle in &mut particles {
                        particle.vel_x = 0.0;
                        particle.vel_y = 0.0;
                    }
                }
                
                Event::MouseButtonDown { mouse_btn, .. } => {
                    match mouse_btn {
                        MouseButton::Middle => {
                            follows_particle = !follows_particle;
                            if follows_particle {

                                let mut last_distance = f32::INFINITY;
                                let mouse_position = canvas_to_scene(&scene, (mouse.mouse_state.x(), mouse.mouse_state.y()));
                                let mut i: usize = 0;

                                for particle in particles.iter() {
                                    let distance = f32::sqrt((particle.pos_x - mouse_position.0) * (particle.pos_x - mouse_position.0) + (particle.pos_y - mouse_position.1) * (particle.pos_y - mouse_position.1));
                                    if distance < last_distance {
                                        follow_particle = i;
                                        last_distance = distance;
                                    }
                                    i += 1;
                                }
                            }
                        }
                        MouseButton::X1 => {
                            println!("X1");
                        }
                        MouseButton::X2 => {
                            println!("X2");
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }


        // FPS
        let elapsed = previous_frame_time.elapsed();
        if elapsed < frame_duration {
            let sleep_duration = frame_duration - elapsed;
            thread::sleep(sleep_duration);
        }

        previous_frame_time = Instant::now();


        // MOUSE
        let mouse_state = event_pump.mouse_state();
        let mouse_position = canvas_to_scene(&scene, (mouse_state.x(), mouse_state.y()));

        mouse.mouse_state = mouse_state;
        if !mouse.locked {
            mouse.pos_x = mouse_position.0;
            mouse.pos_y = mouse_position.1;
        }




        //
        scene.canvas.set_draw_color(Color::RGB(55, 55, 55));
        scene.canvas.clear();

        apply_force_field(&mut particles, &mouse, 2, 5.0);
        //if mouse.mouse_state.left() || mouse.locked {resistance(&mut particles, 0.95);}
        apply_velocity(&mut particles);
        // follow particle
        if follows_particle {
            scene.pos_x = particles[follow_particle].pos_x;
            scene.pos_y = particles[follow_particle].pos_y;
        }
        //change_color(&mut particles, 0.001);
        render(&mut scene, &particles);

        scene.canvas.present();
    
    }
}



struct Particle {
    pos_x: f32,
    pos_y: f32,
    vel_x: f32,
    vel_y: f32,
    color: (f32, f32, f32),
}


struct Scene {
    canvas: WindowCanvas,
    width: u32,
    height: u32,
    pos_x: f32,
    pos_y: f32,
    zoom_factor: f32,
    zoom_level: f32,
}


struct Mouse {
    pos_x: f32,
    pos_y: f32,
    mouse_state: sdl2::mouse::MouseState,
    locked: bool,
}


fn set_shape(particles: &mut Vec<Particle>, range: f32, shape_type: u8) {

    particles.clear();
    
    match shape_type {
        1 => {
            
        }

        2 => {
            
        }

        3 => {
            
        }

        // as 0
        _ => {
            let side_length = f32::floor(f32::sqrt(NUM_PARTICLES as f32)) as usize;
            let mut created_particles: usize = 0;

            for y in 0..side_length {
                for x in 0..side_length {

                    created_particles += 1;

                    if created_particles <= NUM_PARTICLES {
                        particles.push(Particle {
                            pos_x: ((x as f32) / (side_length - 1) as f32 * 2.0 - 1.0) * range,
                            pos_y: ((y as f32) / (side_length - 1) as f32 * 2.0 - 1.0) * range,
                            vel_x: 0.0,
                            vel_y: 0.0,
                            color: (0.0, 0.0, 0.0),
                        });
                    }

                }
            }
        }
    }

}


fn update_zoom(scene: &mut Scene) {
    scene.zoom_factor = f32::powf(2.0, scene.zoom_level);
}


fn scene_to_canvas(scene: &Scene, pos: (f32, f32)) -> (i32, i32) {
    let shorter_side = u32::min(scene.width, scene.height) as f32;
    return (
        ((pos.0 - scene.pos_x) * shorter_side * scene.zoom_factor + scene.width as f32 / 2.0) as i32,
        (-(pos.1 - scene.pos_y) * shorter_side * scene.zoom_factor + scene.height as f32 / 2.0) as i32,
    );
}


fn canvas_to_scene(scene: &Scene, pos: (i32, i32)) -> (f32, f32) {
    let shorter_side = u32::min(scene.width, scene.height) as f32;
    return (
        (pos.0 as f32 - scene.width as f32 / 2.0) / shorter_side / scene.zoom_factor + scene.pos_x,
        -(pos.1 as f32 - scene.height as f32 / 2.0) / shorter_side / scene.zoom_factor + scene.pos_y,
    );
}


fn render(scene: &mut Scene, particles: &Vec<Particle>) {

    scene.canvas.set_draw_color(Color::RGBA(255, 255, 255, 40));

    for particle in particles {
        /* scene.canvas.set_draw_color(Color::RGB(
            particle.color.0 as u8,
            particle.color.1 as u8,
            particle.color.2 as u8,
        )); */
        scene.canvas.draw_point(scene_to_canvas(&scene, (particle.pos_x, particle.pos_y))).unwrap();
    }
    
}


fn apply_velocity(particles: &mut Vec<Particle>) {
    for particle in particles {
        particle.pos_x += particle.vel_x;
        particle.pos_y += particle.vel_y;
    }
}

/*

    0: realsitic
    1: bouncy
    2: wave
    3: 
    4: 
    5: 
    6: 
    7: 

*/
fn apply_force_field(particles: &mut Vec<Particle>, mouse: &Mouse, field_type: u8, strength: f32) {

    if mouse.mouse_state.left() || mouse.locked {
        match field_type {
            1 => {
                for particle in particles {
                    let delta_x = particle.pos_x - mouse.pos_x;
                    let delta_y = particle.pos_y - mouse.pos_y;
                    let dist = f32::sqrt((delta_x) * (delta_x) + (delta_y) * (delta_y));
                    let norm_x = delta_x / dist;
                    let norm_y = delta_y / dist;

                    let force = strength / (dist + 20.0);

                    particle.vel_x -= norm_x * force;
                    particle.vel_y -= norm_y * force;
                }
            }

            2 => {
                for particle in particles {
                    let delta_x = particle.pos_x - mouse.pos_x;
                    let delta_y = particle.pos_y - mouse.pos_y;
                    let dist = f32::sqrt((delta_x) * (delta_x) + (delta_y) * (delta_y));
                    let norm_x = delta_x / dist;
                    let norm_y = delta_y / dist;

                    let force = strength * (f32::cos(dist) / (dist * dist + 100.0) * 300.0 + 0.2) * 0.1;

                    particle.vel_x -= norm_x * force;
                    particle.vel_y -= norm_y * force;
                }
            }

            // as 0
            _ => {
                for particle in particles {
                    let delta_x = particle.pos_x - mouse.pos_x;
                    let delta_y = particle.pos_y - mouse.pos_y;
                    let dist = f32::sqrt((delta_x) * (delta_x) + (delta_y) * (delta_y));
                    let norm_x = delta_x / dist;
                    let norm_y = delta_y / dist;

                    let force = strength / (dist * dist);

                    particle.vel_x -= norm_x * force;
                    particle.vel_y -= norm_y * force;
                }
            }
        }
    }
}


fn change_color(particles: &mut Vec<Particle>, speed: f32) {
    for particle in particles {

        /* let target_color = (particle.pos_x * 20.0, particle.pos_y * 20.0, -particle.pos_x * 20.0);

        particle.color.0 += (target_color.0 - particle.color.0) * speed;
        particle.color.1 += (target_color.1 - particle.color.1) * speed;
        particle.color.2 += (target_color.2 - particle.color.2) * speed; */

        let vel = particle.vel_x * particle.vel_x + particle.vel_y * particle.vel_y * 40.0;
        particle.color.2 = vel;
    }
}


fn resistance(particles: &mut Vec<Particle>, factor: f32) {
    for particle in particles {
        particle.vel_x *= factor;
        particle.vel_y *= factor;
    }
}

