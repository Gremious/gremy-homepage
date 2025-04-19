use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub enum TimeKinds {
	#[default]
	Years,
	Weeks,
	Days,
	Hours,
	Minutes,
	Seconds,
	Miliseconds,
	Microseconds,
	Nanoseconds,
}

impl std::fmt::Display for TimeKinds {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			TimeKinds::Years => f.write_str("year"),
			TimeKinds::Weeks => f.write_str("week"),
			TimeKinds::Days => f.write_str("day"),
			TimeKinds::Hours => f.write_str("hour"),
			TimeKinds::Minutes => f.write_str("minute"),
			TimeKinds::Seconds => f.write_str("second"),
			TimeKinds::Miliseconds => f.write_str("milisecond"),
			TimeKinds::Microseconds => f.write_str("microsecond"),
			TimeKinds::Nanoseconds => f.write_str("nanosecond"),
		}
	}
}

impl TimeKinds {
	pub fn all() -> &'static [Self; 9] {
		&[TimeKinds::Years,
		TimeKinds::Weeks,
		TimeKinds::Days,
		TimeKinds::Hours,
		TimeKinds::Minutes,
		TimeKinds::Seconds,
		TimeKinds::Miliseconds,
		TimeKinds::Microseconds,
		TimeKinds::Nanoseconds]
	}

	pub fn next_smaller(&self) -> Option<Self> {
		match &self {
			TimeKinds::Years => Some(TimeKinds::Weeks),
			TimeKinds::Weeks => Some(TimeKinds::Days),
			TimeKinds::Days => Some(TimeKinds::Hours),
			TimeKinds::Hours => Some(TimeKinds::Minutes),
			TimeKinds::Minutes => Some(TimeKinds::Seconds),
			TimeKinds::Seconds => Some(TimeKinds::Miliseconds),
			TimeKinds::Miliseconds => Some(TimeKinds::Microseconds),
			TimeKinds::Microseconds => Some(TimeKinds::Nanoseconds),
			TimeKinds::Nanoseconds => None,
		}
	}

	// can impl double ended iter maybe? but don't need it
	// pub fn next_larger(&self) -> Option<Self> {
		// match &self {
			// TimeKinds::Years => None,
			// TimeKinds::Weeks => Some(TimeKinds::Years),
			// TimeKinds::Days => Some(TimeKinds::Weeks),
			// TimeKinds::Hours => Some(TimeKinds::Days),
			// TimeKinds::Minutes => Some(TimeKinds::Hours),
			// TimeKinds::Seconds => Some(TimeKinds::Minutes),
			// TimeKinds::Miliseconds => Some(TimeKinds::Seconds),
			// TimeKinds::Microseconds => Some(TimeKinds::Miliseconds),
			// TimeKinds::Nanoseconds => Some(TimeKinds::Microseconds),
		// }
	// }

	pub fn duration(&self, amount: i64) -> chrono::Duration {
		match &self {
			TimeKinds::Years => chrono::Duration::days(amount * 365),
			TimeKinds::Weeks => chrono::Duration::weeks(amount),
			TimeKinds::Days => chrono::Duration::days(amount),
			TimeKinds::Hours => chrono::Duration::hours(amount),
			TimeKinds::Minutes => chrono::Duration::minutes(amount),
			TimeKinds::Seconds => chrono::Duration::seconds(amount),
			TimeKinds::Miliseconds => chrono::Duration::milliseconds(amount),
			TimeKinds::Microseconds => chrono::Duration::microseconds(amount),
			TimeKinds::Nanoseconds => chrono::Duration::nanoseconds(amount),
		}
	}

	pub fn chrono_fn(&self) -> impl Fn(&chrono::TimeDelta) -> i64 {
		match &self {
			TimeKinds::Years => |x: &chrono::TimeDelta| x.num_days() / 365,
			TimeKinds::Weeks => chrono::Duration::num_weeks,
			TimeKinds::Days => chrono::Duration::num_days,
			TimeKinds::Hours => chrono::Duration::num_hours,
			TimeKinds::Minutes => chrono::Duration::num_minutes,
			TimeKinds::Seconds => chrono::Duration::num_seconds,
			TimeKinds::Miliseconds => chrono::Duration::num_milliseconds,
			TimeKinds::Microseconds => |x: &chrono::TimeDelta| chrono::Duration::num_microseconds(x).unwrap_or(0),
			TimeKinds::Nanoseconds => |x: &chrono::TimeDelta| chrono::Duration::num_nanoseconds(x).unwrap_or(0),
		}
	}
}

pub fn dur_leftover(dur: chrono::Duration) -> Option<String> {
	let find = TimeKinds::all().iter()
		.find(|time_kind| time_kind.chrono_fn()(&dur).abs() > 0);

	let Some(curr_time_type) = find else { return Some(String::from("a moment")) } ;

	if let Some(next) = curr_time_type.next_smaller() {
		let smaller_fn = next.chrono_fn();
		let prev_time = smaller_fn(&dur);

		let rounded_time = curr_time_type.chrono_fn()(&dur);
		let rouned_as_smaller = smaller_fn(&curr_time_type.duration(rounded_time));

		let leftover = prev_time - rouned_as_smaller;
		let type_name = next.to_string();

		return Some(format!("{leftover} {type_name}{}",
			if leftover.abs() > 1 { "s" } else { "" }
		));
	}

	None
}

pub fn dur_as_largest_word(dur: chrono::Duration, second_largest: bool) -> String {
	let durations = [
		(dur.num_days() / 365,                "year"),
		(dur.num_weeks(),                     "week"),
		(dur.num_days(),                      "day"),
		(dur.num_hours(),                     "hour"),
		(dur.num_minutes(),                   "minute"),
		(dur.num_seconds(),                   "second"),
		(dur.num_milliseconds(),              "milisecond"),
		(dur.num_microseconds().unwrap_or(0), "microsecond"),
		(dur.num_nanoseconds().unwrap_or(0),  "nanosecond"),
	];

	let Some((i, _)) = durations.iter().enumerate().find(|(_, (x, _))| x.abs() > 0) else { return String::from("a moment")} ;

	if let Some((value, word)) = durations.get(if second_largest { i + 1 } else { i }) {
		format!("{value} {word}{}", if value.abs() > 1 { "s" } else { "" })
	} else {
		String::from("a moment")
	}
}
