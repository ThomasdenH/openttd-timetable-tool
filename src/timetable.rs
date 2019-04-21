use chrono::naive::NaiveDate;
use chrono::Duration;

/// The type of constraint that determines the timetable configuration.
pub enum Constraint {
    /// The number of vehicles is specified.
    NumberOfVehicles(u16),
    /// The frequency a vehicle arrives at a station is specified.
    Frequency(u16),
}

pub struct Timetable {
    start_date: NaiveDate,
    length: Duration,
    constraint: Constraint,
}

impl Timetable {
    /// Create a new OpenTTD timetable.
    ///
    /// # Arguments
    ///
    /// - `start_date` - The date on which the first vehicle departs.
    /// - `length` - The duration of the Timetable
    /// - `constraint` - The constraint by which to fill in the rest
    ///     of the time table.
    pub fn from_constraint(
        start_date: NaiveDate,
        length: Duration,
        constraint: Constraint,
    ) -> Self {
        Timetable {
            start_date,
            length,
            constraint,
        }
    }

    pub fn start_dates(&self) -> impl Iterator<Item = NaiveDate> + '_ {
        let vehicle_count: u16 = match self.constraint {
            Constraint::NumberOfVehicles(vehicle_count) => vehicle_count,
            Constraint::Frequency(frequency) => {
                // For the frequency constraint, we should make sure that the error
                // is homogeneous over all vehicles. To do this, compute the
                // vehicle count while rounding to the closest whole number.
                (self.length.num_days() as f64 / f64::from(frequency)).round() as u16
            }
        };

        // For the vehicle count constraint, divide the length in equal
        // parts. Because of the precision of Duration, there should be no
        // systematic error.
        (0..vehicle_count)
            .map(move |vehicle_index| {
                self.length * i32::from(vehicle_index) / i32::from(vehicle_count)
            })
            .map(move |time_since_start| self.start_date + time_since_start)
    }
}
