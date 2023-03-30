use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default, Copy, Clone)]
pub struct Fps {
    last_time: u128,
    frame_count: i32,
    fps: i32,
}

pub enum FpsRet {
    NotReady,
    NotUpdated(i32),
    Update(i32),
}

const INTERVAL: u128 = 2000 * 1000;

impl Fps {
    pub fn update(&mut self) -> FpsRet {
        let cur_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros();
        self.frame_count += 1;
        if self.last_time != 0 {
            let elapsed = cur_time - self.last_time;
            if elapsed >= INTERVAL {
                self.fps = (self.frame_count as f64 / (elapsed as f64 / 1000000f64)).round() as i32;
                self.last_time = cur_time;
                self.frame_count = 0;
                return FpsRet::Update(self.fps);
            }
        } else {
            self.last_time = cur_time;
        }
        if self.fps == 0 {
            FpsRet::NotReady
        } else {
            FpsRet::NotUpdated(self.fps)
        }
    }
}

impl From<Fps> for i32 {
    fn from(val: Fps) -> Self {
        val.fps
    }
}
