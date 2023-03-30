use minifb::{Key, Window, WindowOptions};
use std::slice;
use tinyrenderer_rs::{draw_line, Color, Fps, FpsRet, Framebuffer, Model};

fn draw(framebuffer: &mut Framebuffer, model: &Model) {
    let verts = &model.verts;
    for i in 0..(verts.len() / 3) {
        draw_line(
            framebuffer,
            &verts[i * 3],
            &verts[i * 3 + 1],
            &Color::white(),
        );
        draw_line(
            framebuffer,
            &verts[i * 3 + 1],
            &verts[i * 3 + 2],
            &Color::white(),
        );
        draw_line(
            framebuffer,
            &verts[i * 3 + 2],
            &verts[i * 3],
            &Color::white(),
        );
    }
    // framebuffer.write("output.png").unwrap();
}

fn rgba_to_bgra(dst: &mut [u32], src: &[u32]) {
    let src_u8 = unsafe { slice::from_raw_parts(src.as_ptr() as *const u8, src.len() * 4) };
    let dst_u8 = unsafe { slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u8, src.len() * 4) };
    for i in 0..src.len() {
        dst_u8[i * 4] = src_u8[i * 4 + 2];
        dst_u8[i * 4 + 1] = src_u8[i * 4 + 1];
        dst_u8[i * 4 + 2] = src_u8[i * 4];
        dst_u8[i * 4 + 3] = src_u8[i * 4 + 3];
    }
}

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 1024;

fn main() {
    let model = Model::load("assets/african_head/african_head.obj").unwrap();
    let mut window = Window::new(
        "Tiny Renderer - ESC to exit",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    )
    .unwrap();
    let mut fps = Fps::default();
    let mut framebuffer = Framebuffer::create_init_color(WIDTH, HEIGHT, &Color::black()).unwrap();
    let mut bgra_buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];
    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.clear_color_with(&Color::black());
        draw(&mut framebuffer, &model);
        let rgba_buffer = framebuffer.to_u32_slice();
        rgba_to_bgra(&mut bgra_buffer, rgba_buffer);
        window
            .update_with_buffer(
                &bgra_buffer,
                framebuffer.width as usize,
                framebuffer.height as usize,
            )
            .unwrap();
        if let FpsRet::Update(fps) = fps.update() {
            window.set_title(format!("Tiny Renderer - ESC to exit (FPS: {})", fps).as_str());
        }
    }
}
