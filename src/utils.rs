use chrono::Utc;


pub fn get_exp(minutes: u8) -> usize {
    let now = Utc::now().timestamp() as usize;
    now + (minutes as usize * 60)
}