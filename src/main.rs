use bevy::{
    app::Events,
    prelude::*,
    window::{WindowResizeConstraints, WindowResized},
};
use bevy_pixels::prelude::*;

// TODO: These variables have to react to change of window size!
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "plothole".to_owned(),
            width: WIDTH as f32,
            height: HEIGHT as f32,
            resize_constraints: WindowResizeConstraints {
                min_width: WIDTH as f32,
                min_height: HEIGHT as f32,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_resource(PixelsOptions {
            width: WIDTH,
            height: HEIGHT,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin)
        .add_system(resize_notificator)
        .add_system(draw)
        .run();
}

struct FunctionValueProvider<T>
where T: Fn(i64) -> i64
{
    function: T,
}

impl<T> FunctionValueProvider<T>
where T: Fn(i64) -> i64
{
    fn new(function: T) -> FunctionValueProvider<T> {
        Self {
            function
        }
    }

    fn get(&self, x: i64) -> i64 {
        HEIGHT as i64 - (self.function)(x)
    }
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    fn to_u8_slice(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

fn resize_notificator(
    resize_event: Res<Events<WindowResized>>,
    mut pixels_ressource: ResMut<PixelsResource>,
) {
    let mut reader = resize_event.get_reader();
    let event = reader.iter(&resize_event).last();
    if let Some(e) = event {
        pixels_ressource
            .pixels
            .resize_buffer(e.width as u32, e.height as u32);
        pixels_ressource
            .pixels
            .resize_surface(e.width as u32, e.height as u32);
    }
}

fn draw(mut pixels_ressource: ResMut<PixelsResource>) {
    let frame: &mut [u8] = pixels_ressource.pixels.get_frame();
    let buffer = &Color::rgb(255, 255, 255)
        .to_u8_slice()
        .repeat(frame.len() / 4);
    frame.copy_from_slice(buffer);
    draw_pixel(0, 0, Color::rgb(255, 0, 0), frame);
    let f = FunctionValueProvider::new(|x| x*x);
    for i in 0..WIDTH - 1 {
        draw_pixel(i, f.get(i.into()).try_into().unwrap_or(HEIGHT), Color::rgb(255, 0, 0), frame);
    }
}

fn draw_pixel(x: u32, y: u32, color: Color, frame: &mut [u8]) -> &[u8] {
    let i: usize = (4 * (x + y * HEIGHT)).try_into().unwrap();
    frame[i..i + 4].copy_from_slice(&Color::rgb(255, 0, 0).to_u8_slice());
    frame
}
