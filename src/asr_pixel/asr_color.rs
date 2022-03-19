#[derive (Copy, Clone)]
pub struct ASRColor {
    pub values: [u8; 4]
}

impl ASRColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> ASRColor {
        let buf = ASRColor{values: [r,g,b,a]};

        return buf;
    }
}