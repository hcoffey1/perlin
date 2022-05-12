//use perlin_noise::PerlinNoise;
extern crate sdl2;
use perlin_noise::PerlinNoise;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::video::Window;
use std::time::Duration;

static WIDTH: i32 = 400;
static HEIGHT: i32 = 400;
static SHAPE: (i32, i32) = (WIDTH, HEIGHT);

fn update_canvas_moving(
    canvas: &mut sdl2::render::Canvas<Window>,
    perlin: &PerlinNoise,
    center_vec: &[Center],
    shape: &(i32, i32),
) {
    for i in 0..shape.0 {
        for j in 0..shape.1 {
            let distance_vec = get_distances((i, j), &center_vec);
            let z = 10;
            let x = ((center_vec[distance_vec[0].1].pos.0 - i).abs() as f64) * 0.02;
            let y = ((center_vec[distance_vec[0].1].pos.1 - j).abs() as f64) * 0.02;

            let rgb = get_1d_rgb(distance_vec.as_slice(), perlin);
            canvas.set_draw_color(Color::RGB(rgb.0, rgb.1, rgb.2));

            canvas
                .draw_point(Point::new(i, j))
                .expect("Failed to draw pixel!");
        }
    }
}
fn get_2d_rgb(distance_vec: &[(f64, usize)], perlin: &PerlinNoise) -> (u8, u8, u8) {
    let scale = 0.01;

    let val_r = perlin.get2d([distance_vec[0].0 * scale, distance_vec[0].0 * scale]);
    let val_r = val_r * 256.0;

    let val_g = perlin.get2d([distance_vec[1].0 * scale, distance_vec[1].0 * scale]);
    let val_g = val_g * 256.0;

    let val_b = perlin.get2d([distance_vec[0].0 * scale, distance_vec[0].0 * scale]);
    let val_b = val_b * 256.0;

    (val_r as u8, val_g as u8, val_b as u8)
}

fn get_1d_rgb(distance_vec: &[(f64, usize)], perlin: &PerlinNoise) -> (u8, u8, u8) {
    let scale = 0.001;

    let val_r = perlin.get(distance_vec[0].0 * scale);
    let val_r = val_r * 256.0;

    let val_g = perlin.get(distance_vec[3].0 * scale);
    let val_g = val_g * 256.0;

    let val_b = perlin.get(distance_vec[2].0 * scale);
    let val_b = val_b * 256.0;

    (val_r as u8, val_g as u8, val_b as u8)
}

fn distance(point: &(i32, i32), center: &(i32, i32)) -> f64 {
    (((point.0 - center.0).pow(2) + (point.1 - center.1).pow(2)) as f64).powf(0.5)
}

fn get_distances(point: (i32, i32), centers: &[Center]) -> Vec<(f64, usize)> {
    let mut distance_vec = Vec::<(f64, usize)>::new();
    let mut j: usize = 0;
    for i in centers {
        let d = distance(&point, &i.pos);
        distance_vec.push((d, j));
        j += 1;
    }

    distance_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    distance_vec
}

struct Center {
    pub pos: (i32, i32),
    canvas_shape: (i32, i32),
    vel: (i32, i32),
}

impl Center {
    pub fn step(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;

        if self.pos.0 >= self.canvas_shape.0 {
            self.pos.0 = self.canvas_shape.0 - 1;
            self.vel.0 *= -1;
        } else if self.pos.0 < 0 {
            self.pos.0 = 0;
            self.vel.0 *= -1;
        }

        if self.pos.1 >= self.canvas_shape.1 {
            self.pos.1 = self.canvas_shape.1 - 1;
            self.vel.1 *= -1;
        } else if self.pos.1 < 0 {
            self.pos.1 = 0;
            self.vel.1 *= -1;
        }
    }

    pub fn new(center: (i32, i32), velocity: (i32, i32), canvas_shape: (i32, i32)) -> Center {
        Center {
            pos: center,
            canvas_shape: canvas_shape,
            vel: velocity,
        }
    }
}

fn render_moving_points(canvas: &mut sdl2::render::Canvas<Window>, sdl_context: &sdl2::Sdl) {
    //For reading keypresses
    let mut event_pump = sdl_context.event_pump().unwrap();

    //Perlin and random numbers
    let mut rng = rand::thread_rng();
    let perlin = PerlinNoise::new();

    //Initialize moving centers
    let mut center_vec = Vec::<Center>::new();
    for i in 1..5 {
        let center = Center::new(
            (rng.gen_range(0..WIDTH), rng.gen_range(0..HEIGHT)),
            (5 * (-1i32).pow(i as u32), 5 * (-1i32).pow(i as u32)),
            SHAPE,
        );
        center_vec.push(center);
    }

    //Render loop
    'running: loop {
        //Draw canvas
        update_canvas_moving(canvas, &perlin, center_vec.as_slice(), &SHAPE);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.present();

        //Update center positions
        for c in center_vec.iter_mut() {
            c.step();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn render_demo2(canvas: &mut sdl2::render::Canvas<Window>, sdl_context: &sdl2::Sdl) {
    //For reading keypresses
    let mut event_pump = sdl_context.event_pump().unwrap();

    //Perlin
    let perlin = PerlinNoise::new();

    //Render loop
    'running: loop {
        for scale in (1..300) {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            for i in 0..SHAPE.0 {
                for j in 0..SHAPE.1 {
                    let i = i + scale;
                    let j = j + scale;

                    let scale = scale as f64 / 10000.0;
                    let r = perlin.get2d([i as f64 * scale, j as f64 * scale]);
                    let g = perlin.get2d([j as f64 * scale, i as f64 * scale]);
                    let b = perlin.get2d([j as f64 * scale, i as f64 * scale]);

                    let r = perlin.get2d([r, r]);
                    let g = perlin.get2d([g, g]);
                    let b = perlin.get2d([b, b]);

                    let r = (r * 200.0) as u8;
                    let g = (g * 130.0) as u8;
                    let b = (b * 130.0) as u8;

                    //let rgb = get_1d_rgb(distance_vec.as_slice(), &perlin);
                    canvas.set_draw_color(Color::RGB(r, g, b));

                    canvas
                        .draw_point(Point::new(i, j))
                        .expect("Failed to draw pixel!");
                }
            }
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

fn main() {
    let sdl_context = match sdl2::init() {
        Ok(sdl) => sdl,
        Err(_) => panic!("Failed to init sdl2 context!"),
    };

    let video_subsystem = match sdl_context.video() {
        Ok(sdl) => sdl,
        Err(_) => panic!("Failed to init VideoSubsystem!"),
    };

    let window = video_subsystem
        .window("Perlin", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    //render_moving_points(&mut canvas, &sdl_context);

    render_demo2(&mut canvas, &sdl_context);
}
