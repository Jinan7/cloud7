use chrono::{DateTime, Utc};

use crate::s3::utils::to_yyymmdd;

pub fn get_scope(
    date: DateTime<Utc>,
    region: &str,
    service: &str,

) -> String {
    //date -> yyyymmdd
    let date = to_yyymmdd(date);
    format!("{}/{}/{}/aws4_request", date, region, service)
}