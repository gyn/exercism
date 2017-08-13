use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    #[inline]
    fn normalize(hour: i32, minute: i32) -> (i32, i32) {
        const HOURS: i32 = 24;
        const MINUTES: i32 = 60;

        let mut c = 0;
        let mut m = minute % MINUTES;
        if m < 0 {
            m += 60;
            c = -1;
        }

        let mut h = (c + minute / MINUTES + hour) % HOURS;
        if h < 0 {
            h += 24;
        }

        (h, m)
    }

    pub fn new(hour: i32, minute: i32) -> Clock {
        let (h, m) = Clock::normalize(hour, minute);

        Clock { hour: h, minute: m }
    }

    pub fn add_minutes(&self, minutes: i32) -> Clock {
        let (h, m) = Clock::normalize(self.hour, self.minute + minutes);

        Clock { hour: h, minute: m }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}
