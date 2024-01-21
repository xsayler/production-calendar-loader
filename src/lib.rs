use std::{error::Error, fmt::Display};

use production_calendar::{
    calendar::ProductionCalendar,
    types::{Day, DayType},
};
use serde::Deserialize;
use time::{macros::format_description, Date};

#[derive(Debug)]
pub enum Country {
    Ru,
    Kz,
}

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
pub struct Calendar {
    pub country_code: String,
    pub country_text: String,
    pub dt_start: String,
    pub dt_end: String,
    pub work_week_type: String,
    pub period: String,
    pub days: Vec<CalendarDay>,
}

#[derive(Debug, Deserialize)]
pub struct CalendarDay {
    pub date: String,
    pub type_id: u8,
    pub type_text: String,
    pub week_day: String,
}

pub struct ProductionCalendarLoader {
    client: reqwest::Client,
}

impl ProductionCalendarLoader {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn load(
        &self,
        country: Country,
        year: u32,
    ) -> Result<ProductionCalendar, Box<dyn Error>> {
        let url = format!(
            "https://production-calendar.ru/get/{}/{}/json",
            country, year
        );
        let request = self.client.get(url).build()?;
        let calendar: Calendar = self.client.execute(request).await?.json().await?;

        self.map_to_production_calendar(year, calendar)
    }

    fn map_to_production_calendar(
        &self,
        year: u32,
        calendar: Calendar,
    ) -> Result<ProductionCalendar, Box<dyn Error>> {
        let mut days: Vec<Day> = vec![];
        for day in calendar.days {
            let day = self.map_to_day(day)?;
            days.push(day);
        }

        let prod_calendar = ProductionCalendar::new(year, days);

        Ok(prod_calendar)
    }

    fn map_to_day(&self, day: CalendarDay) -> Result<Day, Box<dyn Error>> {
        let format = format_description!("[day].[month].[year]");
        let date = Date::parse(day.date.as_str(), &format)?;
        let day = Day {
            date,
            day: date.day().into(),
            month: date.month().into(),
            year: date.year(),
            day_type: self.map_to_day_type(day.type_id)?,
        };

        Ok(day)
    }

    fn map_to_day_type(&self, day_type: u8) -> Result<DayType, Box<dyn Error>> {
        match day_type {
            1 => Ok(DayType::Working),
            2 | 6 => Ok(DayType::Weekend),
            3 | 4 => Ok(DayType::Holiday),
            5 => Ok(DayType::PreHoliday),
            _ => Err(Box::<dyn Error>::from("Unknown week day")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Country, ProductionCalendarLoader};

    #[tokio::test]
    async fn test() {
        let loader = ProductionCalendarLoader::new(reqwest::Client::new());

        let calendar = loader.load(Country::Ru, 2024).await.unwrap();

        assert_eq!(366, calendar.get_days_count());
    }
}
