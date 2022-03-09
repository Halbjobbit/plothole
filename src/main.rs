use bevy::{
    app::Events,
    prelude::*,
    window::{WindowResizeConstraints, WindowResized},
};
use bevy_pixels::prelude::*;

mod color;
mod function;
mod window;

use function::FunctionValueProvider;
use window::WindowInfoRessource;

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
        .insert_resource(WindowInfoRessource {
            width: WIDTH,
            height: HEIGHT,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PixelsPlugin)
        .add_system(resize_notificator)
        .add_system(draw)
        .run();
}

fn resize_notificator(
    resize_event: Res<Events<WindowResized>>,
    mut pixels_ressource: ResMut<PixelsResource>,
    mut window_info_ressource: ResMut<WindowInfoRessource>,
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
        window_info_ressource.width(e.width as u32);
        window_info_ressource.height(e.height as u32);
    }
}

fn draw(
    mut pixels_ressource: ResMut<PixelsResource>,
    window_info_ressource: Res<WindowInfoRessource>,
) {
    let frame: &mut [u8] = pixels_ressource.pixels.get_frame();
    let buffer = &color::Color::rgb(255, 255, 255)
        .to_u8_slice()
        .repeat(frame.len() / 4);
    frame.copy_from_slice(buffer);
    let f = FunctionValueProvider::new(|x| x);
    for i in 0..window_info_ressource.width - 1 {
        if let Ok(y) = (window_info_ressource.height as i64 - f.get(i.into())).try_into() {
            if y < window_info_ressource.height {
                draw_pixel(i, y, color::Color::rgb(255, 0, 0), frame, &window_info_ressource);
            }
        }
    }
}

fn draw_pixel<'a>(
    x: u32,
    y: u32,
    color: color::Color,
    frame: &'a mut [u8],
    window_info_ressource: &Res<WindowInfoRessource>,
) -> &'a [u8] {
    let i: usize = (4 * (x + y * window_info_ressource.width))
        .try_into()
        .unwrap();
    frame[i..i + 4].copy_from_slice(&color.to_u8_slice());
    frame
}

