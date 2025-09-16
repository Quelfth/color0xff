use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {

    pub const fn rgb24(rgb: u32) -> Self {
        Self::rgb(((rgb & 0xff0000) >> 16) as u8, ((rgb & 0x00ff00) >> 8) as u8, (rgb & 0x0000ff) as u8)
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b, 0xFF)
    }

    pub fn rgb_f32(r: f32, g: f32, b: f32) -> Self {
        Self::rgb((r * 255.).clamp(0., 255.) as u8, (g * 255.).clamp(0., 255.) as u8, (b * 255.).clamp(0., 255.) as u8)
    }

    pub fn rgb_f64(r: f64, g: f64, b: f64) -> Self {
        Self::rgb((r * 255.).clamp(0., 255.) as u8, (g * 255.).clamp(0., 255.) as u8, (b * 255.).clamp(0., 255.) as u8)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r, g, b, a)
    }

    pub fn rgba_f64(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self::rgba((r * 255.).clamp(0., 255.) as u8, (g * 255.).clamp(0., 255.) as u8, (b * 255.).clamp(0., 255.) as u8, (a * 255.).clamp(0., 255.) as u8)
    }

    pub fn r(self) -> u8 { self.0 }
    pub fn g(self) -> u8 { self.1 }
    pub fn b(self) -> u8 { self.2 }
    pub fn a(self) -> u8 { self.3 }

    pub fn black_body(temperature: f64) -> Self {
        fn integral(a: f64, b: f64, temperature: f64) -> f64 {
            fn planck_law(wavelength: f64, temperature: f64) -> f64 {
                const K_L: f64 = 143877.68775;
                const A: f64 = 10000000000.;
                A/(wavelength.powi(5) * ((K_L/(wavelength * temperature)).exp() - 1.))
            }

            let mut sum: f64 = 0.;
            for i in 0..=100 {
                let wavelength = a + (b-a)*(i as f64)/100.;
                sum += planck_law(wavelength, temperature);
            }
            sum / 101.
        }

        let total = integral(3.8, 7.5, temperature);

        let red = integral(6., 7., temperature)/total;
        let green = integral(5., 6., temperature)/total;
        let blue = integral(3.8, 5., temperature)/total;

        let max = if red > green && red > blue {red} else if green > blue {green} else {blue};

        Self::rgba_f64(red/max, green/max, blue/max, total)
    }

    pub fn as_f32_array(self) -> [f32; 4] {
        [self.0 as f32/255., self.1 as f32/255., self.2 as f32/255., self.3 as f32/255.]
    }

    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
    pub const RED: Self = Self::rgb(0xff, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 0xff, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 0xff);
    pub const CYAN: Self = Self::rgb(0, 0xff, 0xff);
    pub const MAGENTA: Self = Self::rgb(0xff, 0, 0xff);
    pub const YELLOW: Self = Self::rgb(0xff, 0xff, 0);
    pub const WHITE: Self = Self::rgb(0xff, 0xff, 0xff);
    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const GREY: Self = Self::rgb(0x7f, 0x7f, 0x7f);
}


