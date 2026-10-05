pub enum Measurement {
    Celsius(i32),
    Offline,
    Invalid(String),
}

pub fn temperature(measurement: &Measurement) -> Option<i32> {
    match &measurement {
        Measurement::Celsius(t) => {
            if *t < -40 || *t > 125 {
                None
            } else {
                Some(*t)
            }
        }
        Measurement::Offline => None,
        Measurement::Invalid(_) => None,
    }
}

pub fn last_valid(samples: &[Measurement]) -> Option<i32> {
    let mut result: Option<i32> = None;
    if samples.is_empty() {
        return result;
    }
    for temp in samples.iter() {
        if let Some(t) = temperature(temp) {
            result = Some(t);
        }
    }
    result
}
