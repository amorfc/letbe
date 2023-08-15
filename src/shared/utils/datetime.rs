pub struct LettDate {}

impl LettDate {
    pub fn utc_now() -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }
}
