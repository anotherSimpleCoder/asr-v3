use crate::asr_screen::asr_screen::ASRScreen;

use std::fs::File;
use std::io::Write;

use inline_python::python;

pub struct ASRPNG {
    filename: String,
    scr_instance: ASRScreen,
    width: i64,
    height: i64
}

impl ASRPNG {
    pub fn new(file: String, scr: ASRScreen) -> ASRPNG {
        let inst = scr;
        let inst_p = &inst;

        let w = inst_p.width;
        let h = inst_p.height;

        let buf = ASRPNG{filename: file, scr_instance: inst, width: w, height: h};

        return buf;
    }

    fn encode_to_bin(&self) -> Vec<u8> {
        let mut scr_clone = self.scr_instance.clone();
        let pm = scr_clone.get_pixel_map();
        let mut bytes: Vec<u8> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let p = pm[y as usize][x as usize];
                let c = p.color.values;
                //println!("{} {} {}", c[0], c[1], c[2]);
                bytes.push(c[0]);
                bytes.push(c[1]);
                bytes.push(c[2]);
            }
        }

        return bytes;
    }

    pub fn run_script(&self, bf: String, w: i64, h: i64, on: String) {
        let b_file: String = bf;
        let width: i64 = w;
        let height: i64 = h;
        let out_n: String = on;

        python! {
            import png
            import sys

            def encode_png(byte_file, width, height, out_name):
                f = open(byte_file, "rb")
                byte_data = bytearray(f.read())
                byte_matrix = []

                row = []
                for i in range((width * height * 3)):
                    if len(row) == width*3:
                        row_tuple = tuple(row)
                        byte_matrix.append(row_tuple)
                        row = []
            
                    else:
                        row.append(byte_data[i])
            
                out = open(out_name, "wb")
                writer = png.Writer(width, height-1, greyscale=False)
                writer.write(out, byte_matrix)
                out.close()

            encode_png('b_file, 'width, 'height, 'out_n)
        }
    }

    pub fn write_file(&self) -> Result<u8, String>{
        let file = File::create(self.filename.clone());
        let data = self.encode_to_bin();
        let data_size = data.len();

        match file {
            Ok(mut f) => {
                f.write_all(&data[0..data_size]);
                
                self.run_script(self.filename.clone(), self.width, self.height, self.filename.clone() + ".png");

                println!("File written!");
                return Ok(1)
            },
            Err(x) => return Err(String::from("An Error has occured"))
        }
    }

    
}