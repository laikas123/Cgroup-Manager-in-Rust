use chrono::{Datelike, Timelike, Local};



pub fn get_hours_mins_secs() -> (u32, u32, u32) {
    let now = Local::now();
    let (is_pm, hour) = now.hour12();
    (if is_pm { hour+12} else { hour}, now.minute(), now.second())
}


pub fn get_year_month_day() -> (u32, u32, u32) {
    let now = Local::now();
    let (_, year) = now.year_ce();
    (year, now.month(), now.day())
}


pub fn generate_timestamp_string() -> String {
    let h_m_s = get_hours_mins_secs();
    let y_m_d = get_year_month_day();

    format!("{}-{}-{}-{}-{}-{}", y_m_d.0, y_m_d.1, y_m_d.2, h_m_s.0, h_m_s.1, h_m_s.2).to_string()
}