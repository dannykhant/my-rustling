// Implement a clock that handles times without dates.
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut mins = hours * 60 + minutes;

        if mins >= 1440 {
            mins %= 1440;
        } else if mins < 0 {
            mins = 1440 + mins % 1440;
        }
        
        Self { hours: mins / 60 , minutes: mins % 60 }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut mins = self.hours * 60 + self.minutes + minutes;
        
        if mins >= 1440 {
            mins %= 1440;
        } else if mins < 0 {
            mins = 1440 + mins % 1440;
        }
        
        Self { hours: mins / 60, minutes: mins % 60 }
    }
}