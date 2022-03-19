mod asr_pixel;
mod asr_screen;
mod asr;

use asr_pixel::asr_color::ASRColor;
use asr_pixel::asr_vector::ASRVector;
use asr::asr::ASR;

fn main() {
    let mut a: ASR = ASR::new(300, 300);
    
    
    let p = a.assign(ASRColor::new(255, 128, 199, 255), ASRVector::new(150, 150));
    
    match p {
        Ok(_) => a.render(),
        Err(y) => print!("{}", y)
    }
}
