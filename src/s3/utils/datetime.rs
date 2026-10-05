use chrono::{DateTime, Utc};

pub fn to_iso8601(date: DateTime<Utc>) -> String {
    date.format("%Y%m%dT%H%M%SZ").to_string()
}

pub fn to_yyymmdd(date: DateTime<Utc>) -> String {
    date.format("%Y%m%d").to_string()
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use crate::s3::utils::{to_iso8601, to_yyymmdd};


    #[test]
    pub fn datetime_test() {

        let date = Utc::now();

        let iso8601 = to_iso8601(date);
        let yyymmdd = to_yyymmdd(date);

        dbg!(iso8601);
        dbg!(yyymmdd);
    }
}