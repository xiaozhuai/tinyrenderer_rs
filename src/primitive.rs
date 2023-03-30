use crate::{
    Color, Colorf, Framebuffer, Mat2x3, Mat3, Texture2D, Texture2DFilterMode, Texture2DWrapMode,
    Vec2, Vec2i, Vec3, Vec3i,
};
use std::cmp::{max, min};
use std::ops::Neg;

fn to_screen_pos(pos: &Vec3, screen_size: &Vec2i) -> Vec2i {
    Vec2i::new(
        (((pos.x + 1f32) / 2f32) * screen_size.x as f32).round() as i32,
        (((-pos.y + 1f32) / 2f32) * screen_size.y as f32).round() as i32,
    )
}

pub fn draw_line(framebuffer: &mut Framebuffer, p0: &Vec3, p1: &Vec3, color: &Color) {
    let fb_size = Vec2i::new(framebuffer.width, framebuffer.height);
    let mut p0_s = to_screen_pos(p0, &fb_size);
    let mut p1_s = to_screen_pos(p1, &fb_size);
    let direction = p1 - p0;
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
    let fb_width_2 = fb_size.x as f32 * 0.5f32;
    let fb_height_2 = fb_size.y as f32 * 0.5f32;
    let mut error2 = 0;
    let mut y = p0_s.y;
    if steep {
        for x in p0_s.x..=p1_s.x {
            let depth: f32;
            if direction.y != 0f32 {
                let t = ((x as f32 / fb_height_2 - 1f32) - p0.y) / direction.y;
                depth = p0.z + t * direction.z;
            } else {
                depth = direction.z;
            }
            framebuffer.set_color_with_depth(y, x, depth, color);
            error2 += derror2;
            if error2 > dx {
                y += y_step;
                error2 -= dx * 2;
            }
        }
    } else {
        for x in p0_s.x..=p1_s.x {
            let depth: f32;
            if direction.x != 0f32 {
                let t = ((x as f32 / fb_width_2 - 1f32) - p0.x) / direction.x;
                depth = p0.z + t * direction.z;
            } else {
                depth = direction.z;
            }
            framebuffer.set_color_with_depth(x, y, depth, color);
            error2 += derror2;
            if error2 > dx {
                y += y_step;
                error2 -= dx * 2;
            }
        }
    }
}

fn barycentric(p: &Vec2i, p0: &Vec2i, p1: &Vec2i, p2: &Vec2i) -> Vec3 {
    let s0 = Vec3i::new(p2.x - p0.x, p1.x - p0.x, p0.x - p.x);
    let s1 = Vec3i::new(p2.y - p0.y, p1.y - p0.y, p0.y - p.y);
    let u = s0.cross(&s1);
    if u.z.abs() < 1 {
        Vec3::new(-1f32, 1f32, 1f32)
    } else {
        Vec3::new(
            1f32 - (u.x + u.y) as f32 / u.z as f32,
            u.y as f32 / u.z as f32,
            u.x as f32 / u.z as f32,
        )
    }
}

#[allow(clippy::too_many_arguments)]
pub fn draw_triangle(
    framebuffer: &mut Framebuffer,
    texture: &Texture2D,
    p0: &Vec3,
    p1: &Vec3,
    p2: &Vec3,
    uv0: &Vec2,
    uv1: &Vec2,
    uv2: &Vec2,
    norm0: &Vec3,
    norm1: &Vec3,
    norm2: &Vec3,
    light_dir: &Vec3,
    light_intensity: f32,
) {
    let fb_size = Vec2i::new(framebuffer.width, framebuffer.height);
    let p0_s = &to_screen_pos(p0, &fb_size);
    let p1_s = &to_screen_pos(p1, &fb_size);
    let p2_s = &to_screen_pos(p2, &fb_size);
    let mut bounding_box_min = Vec2i::new(framebuffer.width - 1, framebuffer.height - 1);
    let mut bounding_box_max = Vec2i::new(0, 0);
    let clamp = Vec2i::new(framebuffer.width - 1, framebuffer.height - 1);

    bounding_box_min.x = max(0, min(bounding_box_min.x, p0_s.x));
    bounding_box_min.y = max(0, min(bounding_box_min.y, p0_s.y));
    bounding_box_min.x = max(0, min(bounding_box_min.x, p1_s.x));
    bounding_box_min.y = max(0, min(bounding_box_min.y, p1_s.y));
    bounding_box_min.x = max(0, min(bounding_box_min.x, p2_s.x));
    bounding_box_min.y = max(0, min(bounding_box_min.y, p2_s.y));
    bounding_box_max.x = min(clamp.x, max(bounding_box_max.x, p0_s.x));
    bounding_box_max.y = min(clamp.y, max(bounding_box_max.y, p0_s.y));
    bounding_box_max.x = min(clamp.x, max(bounding_box_max.x, p1_s.x));
    bounding_box_max.y = min(clamp.y, max(bounding_box_max.y, p1_s.y));
    bounding_box_max.x = min(clamp.x, max(bounding_box_max.x, p2_s.x));
    bounding_box_max.y = min(clamp.y, max(bounding_box_max.y, p2_s.y));

    for y in bounding_box_min.y..=bounding_box_max.y {
        for x in bounding_box_min.x..=bounding_box_max.x {
            let bc_screen = barycentric(&Vec2i::new(x, y), p0_s, p1_s, p2_s);
            if bc_screen.x < 0f32 || bc_screen.y < 0f32 || bc_screen.z < 0f32 {
                continue;
            }

            let bc_clip: Vec3 = bc_screen;
            let bc_clip: Vec3 = bc_clip / (bc_clip.x + bc_clip.y + bc_clip.z);

            let p_uv: Mat2x3 = Mat2x3::new(uv0.x, uv1.x, uv2.x, uv0.y, uv1.y, uv2.y);
            let uv: Vec2 = p_uv * bc_clip;

            let p_norm: Mat3 = Mat3::new(
                norm0.x, norm1.x, norm2.x, norm0.y, norm1.y, norm2.y, norm0.z, norm1.z, norm2.z,
            );
            let norm: Vec3 = p_norm * bc_clip;

            let n = norm.normalize().neg();
            let intensity = n.dot(light_dir);

            let depth = p0.z * bc_screen[0] + p1.z * bc_screen[1] + p2.z * bc_screen[2];
            let intensity = intensity * light_intensity;
            let intensity = Colorf::new(intensity, intensity, intensity, 1f32);
            let mut color = texture.texture(
                uv.x,
                uv.y,
                Texture2DWrapMode::ClampToEdge,
                Texture2DFilterMode::Linear,
            );
            color.component_mul_assign(&intensity);
            let color: Color = color.into();
            framebuffer.set_color_with_depth(x, y, depth, &color);
        }
    }
}
