use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Booting...");
    Mouse::new(1).run();
}
pub struct Mouse{
    mouse: mouse_rs::Mouse,
    screen: (i32, i32),
    x: i32,
    y: i32,
    force_y: i32,
    force_x: i32,
}

impl Mouse {
    pub fn new(speed: i32) -> Self {
        let screen = rdev::display_size().expect("Failed to get display size");
        let screen = (screen.0 as i32, screen.1 as i32);

        let x = screen.0 / 2;
        let y = screen.1 / 2;

        Self{
            mouse: mouse_rs::Mouse::new(),
            screen,
            x,
            y,
            force_x: speed,
            force_y: speed,
        }
    }

    pub fn run(&mut self){
        const SLEEP_TIME: Duration = Duration::from_millis(1);
        self.move_to_position();
        let mut trigger;
        let mut curr_pos;
        loop {
            trigger = false;
            self.x += self.force_x;
            self.y += self.force_y;

            if self.x <= 0 || self.x >= self.screen.0  {
                trigger = true;
                self.force_x = -self.force_x;
            }

            if self.y <= 0 || self.y >= self.screen.1 {
                trigger = true;
                self.force_y = -self.force_y;
            }

            self.move_to_position();

            sleep(SLEEP_TIME);

            curr_pos = self.mouse.get_position().unwrap();
            if (curr_pos.x != self.x || curr_pos.y != self.y) && !trigger {
                break
            }
        }
    }

    #[inline]
    pub fn move_to_position(&self){ self.move_to(self.x, self.y); }
    #[inline]
    pub fn move_to(&self, x: i32, y: i32){ self.mouse.move_to(x, y).expect("Failed to move mouse"); }
}
