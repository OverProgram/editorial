use std::cmp::max;
use std::ops::Rem;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AABB {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Rgba {
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    },
    Hsva {
        hue: f32,
        saturation: f32,
        value: f32,
        alpha: f32,
    },
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color::Rgba {
            red: r,
            green: g,
            blue: b,
            alpha: 1.0,
        }
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color::Rgba {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        }
    }

    pub fn hsv(h: f32, s: f32, v: f32) -> Color {
        Color::Hsva {
            hue: h,
            saturation: s,
            value: v,
            alpha: 1.0,
        }
    }

    pub fn r(&self) -> f32 {
        match self.as_rgba() {
            Color::Rgba { red, .. } => red,
            _ => unreachable!()
        }
    }

    pub fn g(&self) -> f32 {
        match self.as_rgba() {
            Color::Rgba { green, .. } => green,
            _ => unreachable!()
        }
    }

    pub fn b(&self) -> f32 {
        match self.as_rgba() {
            Color::Rgba { blue, .. } => blue,
            _ => unreachable!()
        }
    }

    pub fn set_g(&mut self, g: f32) -> &mut Color {
        match self {
            Color::Rgba { green, ..} => {
                *green = g;
            },
            Color::Hsva { hue, saturation, value, alpha } => {
                Color::Hsva{ hue: *hue, saturation: *saturation, value: *value, alpha: *alpha } = Color::Hsva {
                    hue: *hue,
                    saturation: *saturation,
                    value: *value,
                    alpha: *alpha
                }
                    .as_rgba()
                    .set_g(g)
                    .as_hsva();
            }
        }
        self
    }

    pub fn h(&self) -> f32 {
        match self.as_hsva() {
            Color::Hsva { hue , .. } => hue,
            _ => unreachable!()
        }
    }

    pub fn s(&self) -> f32 {
        match self.as_hsva() {
            Color::Hsva { saturation , .. } => saturation,
            _ => unreachable!()
        }
    }

    pub fn v(&self) -> f32 {
        match self.as_hsva() {
            Color::Hsva { value , .. } => value,
            _ => unreachable!()
        }
    }

    pub fn a(&self) -> f32 {
        match *self {
            Color::Rgba { alpha, .. } => alpha,
            Color::Hsva { alpha, .. } => alpha,
        }
    }

    pub fn hsva(h: f32, s: f32, v: f32, a: f32) -> Color {
        Color::Hsva {
            hue: h,
            saturation: s,
            value: v,
            alpha: a,
        }
    }

    pub fn as_hsva(&self) -> Color {
        if let Color::Rgba { red: r, green: g, blue: b, alpha: a } = self {
            let c_max = match [r, g, b].iter().max() {
                Some(val) => **val,
                None => unreachable!()
            };
            let c_min = match [r, g, b].iter().min() {
                Some(val) => **val,
                None => unreachable!()
            };
            let delta = c_max - c_min;

            let hue = if c_max == *r {
                60.0 * ((g - b) / delta).rem(6.0)
            } else if c_max == *g {
                60 * (((b - r) / delta) + 2)
            } else if c_max == *b {
                60 * (((r - g) / delta) + 4)
            } else {
                0.0
            };
            let saturation = if c_max == 0.0 {
                0.0
            } else {
                delta / c_max
            };
            let value = c_max;

            Color::Hsva {
                hue,
                saturation,
                value,
                alpha: *a,
            }
        } else {
            self
        }
    }

    pub fn as_rgba(&self) -> Color {
        if let Color::Hsva { hue: h, saturation: s, value: v, alpha: a } = self {
            let c = v * s;
            let x = c * (1.0 - ((h / 60.0).rem(2.0) - 1.0).abs());
            let m = v - c;

            let (red, green, blue) = if *h > 0.0 && *h < 60.0 {
                (c, x, 0.0)
            } else if *h < 120.0 {
                (x, c, 0.0)
            } else if *h < 180.0 {
                (0.0, c, x)
            } else if *h < 240.0 {
                (0.0, x, c)
            } else if *h < 300.0 {
                (x, 0.0, c)
            } else if *h < 360.0 {
                (c, 0.0, x)
            } else {
                unreachable!()
            };

            Color::Rgba {
                red,
                green,
                blue,
                alpha: *a,
            }
        } else {
            self
        }
    }
}
