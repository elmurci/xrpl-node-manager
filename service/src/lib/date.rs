use chrono::offset::Utc;
use chrono::DateTime;

pub fn _now() -> DateTime<Utc> {
  Utc::now()
}
