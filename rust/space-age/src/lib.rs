#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / 31557600.0
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (0.2408467 * 31557600.0)
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (0.61519726 * 31557600.0)
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (1.0 * 31557600.0)
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (1.8808158 * 31557600.0)
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (11.862615 * 31557600.0)
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (29.447498 * 31557600.0)
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (84.016846 * 31557600.0)
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / (164.79132 * 31557600.0)
    }
}
