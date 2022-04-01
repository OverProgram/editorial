use std::cmp::Ordering;
use std::ops::Rem;

#[derive(Copy, Clone, PartialEq, Debug)]
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

    pub fn set_r(&mut self, r: f32) -> &mut Color {
        match self {
            Color::Rgba { red, ..} => {
                *red = r;
            },
            Color::Hsva { hue: h, saturation: s, value: v, alpha: a } => {
                let new_color = Color::Hsva {
                    hue: *h,
                    saturation: *s,
                    value: *v,
                    alpha: *a
                }
                    .as_rgba()
                    .set_r(r)
                    .as_hsva();

                *h = new_color.h();
                *s = new_color.s();
                *v = new_color.v();
                *a = new_color.a();
            }
        }
        self
    }

    pub fn set_g(&mut self, g: f32) -> &mut Color {
        match self {
            Color::Rgba { green, ..} => {
                *green = g;
            },
            Color::Hsva { hue: h, saturation: s, value: v, alpha: a } => {
                let new_color = Color::Hsva {
                    hue: *h,
                    saturation: *s,
                    value: *v,
                    alpha: *a
                }
                    .as_rgba()
                    .set_g(g)
                    .as_hsva();

                *h = new_color.h();
                *s = new_color.s();
                *v = new_color.v();
                *a = new_color.a();
            }
        }
        self
    }

    pub fn set_b(&mut self, b: f32) -> &mut Color {
        match self {
            Color::Rgba { blue, ..} => {
                *blue = b;
            },
            Color::Hsva { hue: h, saturation: s, value: v, alpha: a } => {
                let new_color = Color::Hsva {
                    hue: *h,
                    saturation: *s,
                    value: *v,
                    alpha: *a
                }
                    .as_rgba()
                    .set_b(b)
                    .as_hsva();

                *h = new_color.h();
                *s = new_color.s();
                *v = new_color.v();
                *a = new_color.a();
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

    pub fn set_h(&mut self, h: f32) -> &mut Color {
        match self {
            Color::Hsva { hue, ..} => {
                *hue = h;
            },
            Color::Rgba { red: r, green: g, blue: b, alpha: a } => {
                let new_color = Color::Rgba {
                    red: *r,
                    green: *g,
                    blue: *b,
                    alpha: *a
                }
                    .as_hsva()
                    .set_h(h)
                    .as_rgba();

                println!("{:?}", new_color);

                *r = new_color.r();
                *g = new_color.g();
                *b = new_color.b();
                *a = new_color.a();
            }
        }
        self
    }

    pub fn set_s(&mut self, s: f32) -> &mut Color {
        match self {
            Color::Hsva { saturation, ..} => {
                *saturation = s;
            },
            Color::Rgba { red: r, green: g, blue: b, alpha: a } => {
                let new_color = Color::Rgba {
                    red: *r,
                    green: *g,
                    blue: *b,
                    alpha: *a
                }
                    .as_hsva()
                    .set_s(s)
                    .as_rgba();

                *r = new_color.r();
                *g = new_color.g();
                *b = new_color.b();
                *a = new_color.a();
            }
        }
        self
    }

    pub fn set_v(&mut self, v: f32) -> &mut Color {
        match self {
            Color::Hsva { value, ..} => {
                *value = v;
            },
            Color::Rgba { red: r, green: g, blue: b, alpha: a } => {
                let new_color = Color::Rgba {
                    red: *r,
                    green: *g,
                    blue: *b,
                    alpha: *a
                }
                    .as_hsva()
                    .set_v(v)
                    .as_rgba();

                *r = new_color.r();
                *g = new_color.g();
                *b = new_color.b();
                *a = new_color.a();
            }
        }
        self
    }

    pub fn a(&self) -> f32 {
        match *self {
            Color::Rgba { alpha, .. } => alpha,
            Color::Hsva { alpha, .. } => alpha,
        }
    }

    pub fn set_a(&mut self, a: f32) -> &mut Color {
        match self {
            Color::Rgba { alpha, .. } => *alpha = a,
            Color::Hsva { alpha, .. } => *alpha = a,
        }
        self
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
            let c_max = match [r, g, b].iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)) {
                Some(val) => **val,
                None => unreachable!()
            };
            let c_min = match [r, g, b].iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)) {
                Some(val) => **val,
                None => unreachable!()
            };
            let delta = c_max - c_min;

            let hue = if delta == 0.0 {
                0.0
            } else if c_max == *r {
                60.0 * ((g - b) / delta).rem(6.0)
            } else if c_max == *g {
                60.0 * (((b - r) / delta) + 2.0)
            } else if c_max == *b {
                60.0 * (((r - g) / delta) + 4.0)
            } else {
                unreachable!()
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
            *self
        }
    }

    pub fn as_rgba(&self) -> Color {
        if let Color::Hsva { hue: h, saturation: s, value: v, alpha: a } = self {
            let c = v * s;
            let x = c * (1.0 - ((h / 60.0).rem(2.0) - 1.0).abs());
            let m = v - c;

            let (red, green, blue) = if *h >= 0.0 && *h < 60.0 {
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
                red: red + m,
                green: green + m,
                blue: blue + m,
                alpha: *a,
            }
        } else {
            *self
        }
    }
}

#[cfg(test)]
mod color_tests {
    use crate::utils::color::Color;

    fn assert_colors_eq(a: &Color, b: &Color) {
        assert_eq!(a.r(), b.r());
        assert_eq!(a.g(), b.g());
        assert_eq!(a.b(), b.b());
        assert_eq!(a.h(), b.h());
        assert_eq!(a.s(), b.s());
        assert_eq!(a.v(), b.v());
        assert_eq!(a.a(), b.a());
    }

    #[test]
    fn read_test() {
        let color_rgb = Color::rgb(1.0, 0.5, 0.25);
        let color_hsv = Color::hsv(20.0 ,0.75, 1.0);

        assert_eq!(color_rgb.h(), 20.0);
        assert_eq!(color_rgb.s(), 0.75);
        assert_eq!(color_rgb.v(), 1.0);

        assert_eq!(color_hsv.r(), 1.0);
        assert_eq!(color_hsv.g(), 0.5);
        assert_eq!(color_hsv.b(), 0.25);
    }

    #[test]
    fn conversion_test() {
        let color = Color::rgb(1.0, 0.5, 0.25);
        let color_hsv = color.as_hsva();

        assert_colors_eq(&color, &color_hsv);

        match color_hsv {
            Color::Hsva { hue, saturation, value, alpha } => {
                assert_eq!(hue, 20.0);
                assert_eq!(saturation, 0.75);
                assert_eq!(value, 1.0);
                assert_eq!(alpha, 1.0);
            },
            Color::Rgba { .. } => unreachable!(),
        }

        match color_hsv.as_rgba() {
            Color::Rgba { red, green, blue, alpha } => {
                assert_eq!(red, 1.0);
                assert_eq!(green, 0.5);
                assert_eq!(blue, 0.25);
                assert_eq!(alpha, 1.0)
            },
            _ => unreachable!()
        }
    }

    #[test]
    fn test_set_colors() {
        let mut color = Color::hsva(0.0, 0.0, 0.0, 0.0);
        let mut color_rgb = color.as_rgba();

        color.set_r(1.0);
        color_rgb.set_r(1.0);
        assert_colors_eq(&color, &color_rgb);

        color.set_g(1.0);
        color_rgb.set_g(1.0);
        assert_colors_eq(&color, &color_rgb);

        color.set_b(1.0);
        color_rgb.set_b(1.0);
        assert_colors_eq(&color, &color_rgb);

        color.set_h(1.0);
        color_rgb.set_h(1.0);
        println!("{:?}", color_rgb);
        assert_colors_eq(&color, &color_rgb);

        color.set_s(1.0);
        color_rgb.set_s(1.0);
        assert_colors_eq(&color, &color_rgb);

        color.set_v(1.0);
        color_rgb.set_v(1.0);
        assert_colors_eq(&color, &color_rgb);

        color.set_a(1.0);
        color_rgb.set_a(1.0);
        assert_colors_eq(&color, &color_rgb);
    }
}
