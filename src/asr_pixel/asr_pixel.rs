use super::asr_color::ASRColor;
use super::asr_vector::ASRVector;

#[derive(Copy, Clone)]
pub struct ASRPixel {
    pub color: ASRColor,
    pub position: ASRVector
}