use std::io::Write;
 
use crate::shapes::vec3::Vec3;
 
// Type alias
pub type Color = Vec3;
 
pub fn write_color(out: &mut impl Write, pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
 
    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    // Correction gamma
    // Les valeurs de couleur calculées doivent être corrigées pour refléter la réalité. Cela est fait en appliquant une correction gamma (γ=2.0),
    // ce qui équivaut à prendre la racine carrée des couleurs après échantillonnage.
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);
 
    // Write the translated [0, 255] value of each color component
    writeln!(
        out,
        "{} {} {}",
        (256.0 * super::common::clamp(r, 0.0, 0.999)) as i32,
        (256.0 * super::common::clamp(g, 0.0, 0.999)) as i32,
        (256.0 * super::common::clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("writing color");
}