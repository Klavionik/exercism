macro_rules! planet {
    (name = $n:ident, period = $c:expr) => {
        pub struct $n;

        impl Planet for $n {
            fn years_during(d: &Duration) -> f64 {
                d.0 / (31_557_600.0 * $c)
            }
        }
    };
}

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { 0: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

planet!(name = Mercury, period = 0.2408467);
planet!(name = Venus, period = 0.61519726);
planet!(name = Earth, period = 1.0);
planet!(name = Mars, period = 1.8808158);
planet!(name = Jupiter, period = 11.862615);
planet!(name = Saturn, period = 29.447498);
planet!(name = Uranus, period = 84.016846);
planet!(name = Neptune, period = 164.79132);