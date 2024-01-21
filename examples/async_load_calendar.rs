use production_calendar_loader::{Country, ProductionCalendarLoader};

pub fn main() {
    let loader = ProductionCalendarLoader::new_sync();

    let calendar = loader.load(Country::Ru, 2024).unwrap();

    println!(
        "Working days in {}: {}",
        calendar.get_year(),
        calendar.get_work_days_count()
    );
}
