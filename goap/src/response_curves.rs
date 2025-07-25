use num_complex::Complex;



pub trait RealExtensions {
    fn safe_powf(self, n: f64) -> f64;
}

/// Safely computes a^b and always returns the real part,
/// even for negative bases with fractional exponents.
// courtesy of copilot (I didn't know how to handle wierd exponents and negative bases)
impl RealExtensions for f64 {
    fn safe_powf(self, n: f64) -> f64 {
        if self.is_sign_negative() && n.fract() != 0.0 {
            Complex::new(self, 0.0).powf(n).re
        } else {
            self.powf(n)
        }

    }
}

pub struct CurveProperties {
    pub function: fn(f64, f32, f32, f32, f32) -> f64,
}

#[derive(Clone, Debug, Copy)]
pub struct ResponseCurve {
    pub curve: fn(f64, f32, f32, f32, f32) -> f64,
    m: f32,
    k: f32,
    c: f32,
    b: f32,
    pub min: f64,
}

impl ResponseCurve {
    pub fn output(&self, x: f64) -> f64 {
        (self.curve)(x, self.m, self.k, self.c, self.b).clamp(0.0,1.0)
    }

    pub fn unclamped_output(&self, x: f64) -> f64 {
        (self.curve)(x, self.m, self.k, self.c, self.b)
    }
    pub fn linear(m: f32, c:f32, b:f32) -> ResponseCurve {
        let mut function = ResponseCurve {
            curve: |x, m, k, c, b| (m as f64 * (x - c as f64 )) + b as f64,
            m,
            k: 1.0,
            c,
            b,
            min: 0.0,
        };
        function.min = ((function.output(1.0) < function.output(0.0)) as u8) as f64;
        function
    }

    pub fn polynomial(min: f64, m: f32, k: f32, c:f32, b:f32) -> ResponseCurve {
        ResponseCurve {
            curve: |x, m, k, c, b|
                m as f64 * (x - c as f64).safe_powf(k as f64) + b as f64,
            m,
            k,
            c,
            b,
            min,
        }
    }
}