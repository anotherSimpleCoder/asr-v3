use crate::asr_pixel::asr_pixel::ASRPixel;
use crate::asr_pixel::asr_color::ASRColor;
use crate::asr_pixel::asr_vector::ASRVector;

pub struct ASRScreen {
    pub width: i64,
    pub height: i64,
    pixel_map: Vec<Vec<ASRPixel>>
}

impl Clone for ASRScreen {
    fn clone(&self) -> ASRScreen {
        let mut map_buf: Vec<Vec<ASRPixel>> = Vec::new();
        let h = self.height;
        let w = self.width;

        for y in 0..h {
            map_buf.push(Vec::new());

            for x in 0..w {
                map_buf[y as usize].push(self.pixel_map[y as usize][x as usize]);
            }
        }

        let buf = ASRScreen{width: self.width, height: self.height, pixel_map: map_buf};

        return buf;
    }
}

impl ASRScreen {
    pub fn new(w: i64, h: i64) -> ASRScreen {
        let mut buf_map: Vec<Vec<ASRPixel>> = Vec::new();
    
        for i in 0..h {
            buf_map.push(Vec::new());
    
            for j in 0..w {
                let c = ASRColor::new(0,0,0,0);
                let v = ASRVector::new(i,j);
    
                buf_map[i as usize].push(ASRPixel{color: c, position: v});
            }
        }
    
    
        let res = ASRScreen{width: w, height: h, pixel_map: buf_map};
    
        return res;
    }


    pub fn get_pixel(&mut self, x: i64, y:i64) -> Result<&mut ASRPixel, String>{
        if x > self.width || y > self.height {
            return Err(String::from("Given coordinates are too big!"));
        }

        else {
            let pix = &mut self.pixel_map[x as usize][y as usize];
            return Ok(pix);
        }
    }

    pub fn assign(&mut self, color: ASRColor, position: ASRVector) -> Result<String, String> {
        let x = position.values[0];
        let y = position.values[1];
        let px = self.get_pixel(x, y);
        
        match px {
            Ok (x) => {
                x.color = color;
                return Ok(String::from("Success"))
            },

            Err(y) => return Err(y)
        }
    }
}

