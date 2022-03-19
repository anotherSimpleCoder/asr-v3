use crate::asr_pixel::asr_pixel::ASRPixel;
use crate::asr_pixel::asr_color::ASRColor;
use crate::asr_pixel::asr_vector::ASRVector;
use crate::asr_screen::asr_screen::ASRScreen;
use speedy2d::Window;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::shape::Rectangle;
use speedy2d::dimen::Vector2;

struct ASRWindowHandler {
    pub scr_instance: ASRScreen
}

pub struct ASR {
    screen: ASRScreen
}

impl ASRWindowHandler {
    fn new(asr_inst: ASRScreen) -> ASRWindowHandler {
        let buf = ASRWindowHandler{scr_instance: asr_inst};

        return buf;
    }

    fn get_scr_width(&self) -> i64 {
        return self.scr_instance.width;
    }

    fn get_scr_height(&self) ->i64 {
        return self.scr_instance.height;
    }

    fn get_scr_pixel(&mut self, x:i64, y:i64) -> Result<&mut ASRPixel, String> {
        return self.scr_instance.get_pixel(x, y);
    }
}

impl WindowHandler for ASRWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::from_rgb(0.0, 0.0, 0.0));
        
        
        for y in 0..self.get_scr_height() {
            for x in 0..self.get_scr_width() {
                let px = self.get_scr_pixel(x, y);
                
                match px {
                    Ok(a) => {
                        let c = &a.color;
                        let v = &a.position;

                        let new_x = v.values[0] as f32;
                        let new_y = v.values[1] as f32;

                        graphics.draw_rectangle( Rectangle::new(Vector2::new(new_x, new_y), Vector2::new(new_x + 1.0, new_y + 1.0)), Color::from_int_rgba(c.values[0], c.values[1], c.values[2], c.values[3]) );
                    },

                    Err(b) => print!("{}", b)
                }
            }
        }
        

        helper.request_redraw();
    }
}

impl ASR {
    pub fn new(w: i64, h: i64) -> ASR {
        let scr = ASRScreen::new(w, h);
        let buf = ASR{screen: scr};

        return buf;
    }

    pub fn assign(&mut self, color: ASRColor, position: ASRVector) -> Result<String, String> {
        let res = self.screen.assign(color, position);

        return res;
    }

    pub fn render (&self) {
        let win = Window::new_centered(String::from("ASR3"), (self.screen.width as u32, self.screen.height as u32)).unwrap();
        let window_handler = ASRWindowHandler::new(self.screen.clone());
        win.run_loop(window_handler);
    }
}