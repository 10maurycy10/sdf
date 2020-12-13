mod line_noise {
    pub use core::time::Duration;
    
    pub extern crate nalgebra as na;
    pub use na::{Vector3,Vector2};
    pub use nalgebra::Rotation3;
    pub use nalgebra::Isometry3;
    pub use nalgebra::Point;
        
    pub use sdl2::rect::Rect as SRect;
    pub use sdl2::keyboard::PressedScancodeIterator;
    pub use sdl2::keyboard::Scancode;
    pub use sdl2::rect::Point as SPoint;
    pub use sdl2::video::Window;
    pub use sdl2::pixels::Color;
    pub use sdl2::render::Canvas;
    pub use sdl2::event::Event;
}

use self::line_noise::*;

mod sdf;
use sdf::*;

#[inline]
fn get_dist_col(pos: &Vector3<f32>) -> (f32,Vector3<f32>) {
    op_union(
        pos,
        |p| op_move(
            p,
            &Vector3::new(0.0,0.0,0.0),
            |p| sdf_torus(p,&Vector2::new(0.5,0.25))
        ),
        |p| op_move(
            p,
            &Vector3::new(0.0,0.0,2.0),
            |p| op_revolve(
                p,
                1.0,
                |x| ((x.norm()-0.5).abs(),Vector3::new(0.0,1.0,0.0))
            )
        )
    )
}

const SCALE: u32 = 3;
const PLAIN: f32 = 1.0;
const SKY: f32 = 30.0;
const HIT_TOLERANCE: f32 = 0.0001;
const CAMERA_SPEED: f32 = 2.0; //u per second
const GLOW_END: f32 = 0.3;

#[inline]
fn compute_sky_glow(mindist: f32) -> (f32,Vector3<f32>) {
    let glow_intensity: f32 = 0.0_f32.max(((GLOW_END-mindist))/GLOW_END);
    (glow_intensity,Vector3::new(0.0,0.0,0.2))
}

#[inline]
fn raymarch(dir: Vector2<f32>,is: Isometry3<f32>) -> Color {
    let v = Vector3::new(dir.x,PLAIN,dir.y).normalize();
    let mut pos = Point::from(Vector3::new(0.0,0.0,0.0));
    let mut step_count:f32 = 1.0;
    let mut mindist:f32 = f32::MAX;
    loop {
        let sp = (is*pos).coords;
        let dist = get_dist_col(&sp).0;
        mindist = mindist.min(dist);
        let mag = pos.coords.norm();
        if mag > SKY {
            let x = compute_sky_glow(mindist);
            let y = x.1*x.0;
            return Color::RGB(
                (y.x*255.0) as u8,
                (y.y*255.0) as u8,
                (y.z*255.0) as u8
            );
        }
        if dist < HIT_TOLERANCE {
            let shaded = get_dist_col(&sp).1*((1.0/step_count*3.0)+0.5).clamp(0.0,1.0);
            let glow = compute_sky_glow(mindist);
            let glowed = shaded+((glow.1)*glow.0/1.0);
            return Color::RGB(
                (glowed.x*255.0) as u8,
                (glowed.y*255.0) as u8,
                (glowed.z*255.0) as u8
            );
        }
        step_count += 1.0;
        pos += v * dist;
    }
}

fn draw(dest: &mut Canvas<Window>,is: Isometry3<f32>) {
    let size = dest.output_size().unwrap();
    let end: Vector2<f32> = Vector2::new(size.0 as f32,size.1 as f32);
    let center = end/2.0;
    for x in 0..(size.0/SCALE) {
        for y in 0..(size.1/SCALE) {
            //normalize pos to -1 to 1
            let pos = (Vector2::new(x as f32, y as f32)-center/(SCALE as f32)).component_div(&(end/(SCALE as f32)));
            dest.set_draw_color(raymarch(pos,is));
            let r = SRect::new((x*SCALE) as i32,(y*SCALE) as i32,SCALE,SCALE);
            dest.fill_rect(r).unwrap();
        }
    }
}

fn move_camera(camera: Isometry3<f32>, timestep :f32, keys: PressedScancodeIterator) -> Isometry3<f32> {
    let mut movement = Vector3::new(0.0,0.0,0.0);
    let mut rot:f32 = 0.0;
    let mut rot2:f32 = 0.0;
    for key in keys {
        match key {
            Scancode::W => movement.y += CAMERA_SPEED,
            Scancode::S => movement.y -= CAMERA_SPEED,
            Scancode::A => movement.x -= CAMERA_SPEED,
            Scancode::D => movement.x += CAMERA_SPEED,
            Scancode::Q => movement.z -= CAMERA_SPEED,
            Scancode::E => movement.z += CAMERA_SPEED,
 
            Scancode::K => rot2 += 0.3,
            Scancode::I => rot2 -= 0.3,
            Scancode::J => rot += 0.3,
            Scancode::L => rot -= 0.3,
            _ => (),
        }
    }
    camera*Isometry3::new(
        movement*timestep,
        Vector3::new(rot2, 0.0 , rot)*timestep
    )
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
 
    let window = video_subsystem.window("sdf", 900, 900)
        .position_centered()
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas = window.into_canvas() //make software renderer
        //.accelerated()
        .software()
        .build()
        .unwrap();
    let mut is = Isometry3::new(
        Vector3::new(2.0,-16.0,2.0),
        Vector3::new(0.0,0.0,0.0)
    );
    let mut last_start_time = timer.ticks();
    'running: loop {
        let change = timer.ticks() - last_start_time;
        last_start_time = timer.ticks();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {timestamp: _} => {
                    break 'running
                },
                _ => {}
            }
        }
        is = move_camera(is,
            (change as f32)/1000.0,
            event_pump.keyboard_state().pressed_scancodes()
        );
        draw(&mut canvas,is);
        canvas.present();
    }
}
