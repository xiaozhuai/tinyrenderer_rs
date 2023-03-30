use tinyrenderer_rs::{Color, Framebuffer};

#[test]
fn test_framebuffer() {
    let mut framebuffer = Framebuffer::create(64, 64).unwrap();
    assert_eq!(Color::transparent(), *framebuffer.get_color(0, 0).unwrap());
    framebuffer.set_color(10, 10, &Color::red());
    assert_eq!(Color::red(), *framebuffer.get_color(10, 10).unwrap());
}
