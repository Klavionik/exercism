macro_rules! planet {
    ($n:ident, period = $p:expr) => {
        pub struct $n;

        impl Planet for $n {
            fn years_during(d: &Duration) -> f64 {
                d.0 / (31_557_600.0 * $p)
            }
        }
    };
}

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

planet!(Mercury, period = 0.2408467);
planet!(Venus, period = 0.61519726);
planet!(Earth, period = 1.0);
planet!(Mars, period = 1.8808158);
planet!(Jupiter, period = 11.862615);
planet!(Saturn, period = 29.447498);
planet!(Uranus, period = 84.016846);
planet!(Neptune, period = 164.79132);