use chrono::{Datelike, Duration, Local, NaiveDate};

pub fn today() -> NaiveDate {
    Local::now().date_naive()
}

pub fn calendar_window(start: NaiveDate) -> Vec<NaiveDate> {
    (0..=7).map(|i| start + Duration::days(i)).collect()
}

pub fn weekday_es(date: NaiveDate) -> &'static str {
    match date.weekday().number_from_monday() {
        1 => "Lunes",
        2 => "Martes",
        3 => "Miércoles",
        4 => "Jueves",
        5 => "Viernes",
        6 => "Sábado",
        7 => "Domingo",
        _ => unreachable!(),
    }
}

pub fn day_header(date: NaiveDate) -> String {
    format!("{} {}", date.format("%Y-%m-%d"), weekday_es(date))
}
