pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        let mut hours = hours;
        let mut minutes = minutes;
        if minutes > 59 {
            hours += minutes / 60;
            minutes = minutes % 60;
        }
        if minutes < 0 {
            hours -= 1;
            minutes = 60 + minutes;
        }
        if hours > 23 {
            hours = hours % 24;
        }
        if hours < 0 {
            hours = 24 + hours;
        }
        Clock {hours, minutes}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_hours = self.hours;
        let mut new_minutes = self.minutes + minutes;
    
        if new_minutes >= 60 {
            new_hours += new_minutes / 60;
            new_minutes %= 60;
        } else if new_minutes < 0 {
            new_hours -= 1;
            new_minutes += 60;
        }
    
        if new_hours >= 24 {
            new_hours %= 24;
        } else if new_hours < 0 {
            new_hours += 24;
        }
    
        Clock { hours: new_hours, minutes: new_minutes }
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    
}
use std::fmt;

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Clock {{ hours: {:02}, minutes: {:02} }}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}