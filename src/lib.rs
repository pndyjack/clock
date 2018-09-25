use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
  hours: i32,
  minutes: i32,
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {
    if hours < 24 && minutes < 60 && hours >= 0 && minutes >= 0 {
      return Clock {
        hours: hours,
        minutes: minutes,
      };
    }
    let minutes_added = minutes + (hours * 60);
    Clock {
      hours: 0,
      minutes: 0,
    }.add_minutes(minutes_added)
  }

  pub fn add_minutes(self, minutes: i32) -> Self {
    let mut whole_hours_added = minutes / 60;
    let minutes_added = minutes % 60;

    if minutes_added < 60 && (self.minutes + minutes_added) >= 60 && minutes_added > 0 {
      whole_hours_added += 1;
    } else if minutes_added < 0 && (self.minutes + minutes_added) < 0 {
      whole_hours_added -= 1;
    }
    Clock::new(
      wrap_around(self.hours + whole_hours_added, 24),
      wrap_around(self.minutes + minutes_added, 60),
    )
  }
}

fn wrap_around(value: i32, around: i32) -> i32 {
  let mut wrapped_value = value;
  if wrapped_value < 0 {
    while wrapped_value < 0 {
      wrapped_value += around;
    }
  } else {
    wrapped_value %= around;
  }
  wrapped_value
}

impl fmt::Display for Clock {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:02}:{:02}", &self.hours, &self.minutes)
  }
}
