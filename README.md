<h1> production-calendar-loader </h1>
<p> Production calendar loader </p>

## Overview

This crate contains an implementation of the production calendar loader from https://production-calendar.ru.

## Usage

```rust
#[tokio::main]
pub async fn main() {
    let loader = ProductionCalendarLoader::new(reqwest::Client::new());
        
    let calendar = loader.load(Country::Ru, 2024).await.unwrap();

    println!("Working days in {}: {}", calendar.get_year(), calendar.get_work_days_count());
}
```

For a full example, see: [examples](https://github.com/xsayler/production-calendar-loader/tree/main/examples/load_calendar.rs)

## License

This project is licensed under [MIT license](https://github.com/xsayler/production-calendar-loader/blob/main/LICENSE).