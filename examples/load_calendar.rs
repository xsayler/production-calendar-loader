use production_calendar_loader::{Country, ProductionCalendarLoader};

#[tokio::main]
pub async fn main() {
    let loader = ProductionCalendarLoader::new(reqwest::Client::new());

    let calendar = loader.load(Country::Ru, 2024).await.unwrap();

    println!(
        "Working days in {}: {}",
        calendar.get_year(),
        calendar.get_work_days_count()
    );
}
