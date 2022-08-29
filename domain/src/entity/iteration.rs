use chrono::{DateTime, Utc};

#[cfg_attr(test, faux::create)]
pub struct Iteration {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub hours: i32,
}

#[cfg_attr(test, faux::methods)]
impl Iteration {
    pub fn new(start_date: DateTime<Utc>, end_date: DateTime<Utc>, hours: i32) -> Self {
        Self {
            start_date,
            end_date,
            hours,
        }
    }
}
