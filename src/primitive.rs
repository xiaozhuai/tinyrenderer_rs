use crate::{Color, Framebuffer, Vec2i};

pub fn draw_line(framebuffer: &mut Framebuffer, p0: &Vec2i, p1: &Vec2i, color: &Color) {
    let mut p0_s = *p0;
    let mut p1_s = *p1;
    let mut steep = false;
    if (p0_s.x - p1_s.x).abs() < (p0_s.y - p1_s.y).abs() {
        p0_s.swap_rows(0, 1);
        p1_s.swap_rows(0, 1);
        steep = true;
    }
    if p0_s.x > p1_s.x {
        std::mem::swap(&mut p0_s, &mut p1_s);
    }
    let dx = p1_s.x - p0_s.x;
    let dy = p1_s.y - p0_s.y;
    let derror2 = dy.abs() * 2;
    let y_step = if p1_s.y > p0_s.y { 1 } else { -1 };
    let mut error2 = 0;
    let mut y = p0_s.y;
    if steep {
        for x in p0_s.x..=p1_s.x {
            framebuffer.set_color(y, x, color);
            error2 += derror2;
            if error2 > dx {
                y += y_step;
                error2 -= dx * 2;
            }
        }
    } else {
        for x in p0_s.x..=p1_s.x {
            framebuffer.set_color(x, y, color);
            error2 += derror2;
            if error2 > dx {
                y += y_step;
                error2 -= dx * 2;
            }
        }
    }
}
