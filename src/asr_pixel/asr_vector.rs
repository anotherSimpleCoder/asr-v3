#[derive(Copy, Clone)]
pub struct ASRVector {
    pub values: [i64; 2]
}

impl ASRVector {
    pub fn new(x: i64, y:i64) -> ASRVector {
        let buf = ASRVector{values: [x,y]};

        return buf;
    }
}