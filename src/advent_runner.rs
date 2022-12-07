use chrono::Datelike;
use crate::{year_2021, year_2022};

pub trait Day {
    fn date(&self) -> (i32, i32);
    fn run(&self);
}

pub struct AdventRunner {
    days: Vec<Box<dyn Day>>,
    current_year: i32,
    run_target: Option<Box<dyn Day>>
}

impl AdventRunner {
    pub fn new() -> Self {
        AdventRunner {
            current_year: chrono::Utc::now().year(),
            days: vec![
                Box::new(year_2021::day1::Day1),
                Box::new(year_2021::day2::Day2),
                Box::new(year_2021::day3::Day3),
                Box::new(year_2022::day1::Day1),
                Box::new(year_2022::day2::Day2),
                Box::new(year_2022::day3::Day3),
                Box::new(year_2022::day4::Day4),
                Box::new(year_2022::day5::Day5),
                Box::new(year_2022::day6::Day6),
            ],
            run_target: None
        }
    }

    pub fn set_year(mut self, year: i32) -> Self {
        self.current_year = year;
        return self;
    }


    pub fn run(self) {
        let latest_year = chrono::Utc::now().year();
        if self.current_year == latest_year {
            let day = self.days.last().unwrap().clone();
            println!("Day: {}, Year {}", day.date().0, day.date().1);
            day.run();
            return;
        }

        for (i, day) in self.days.iter().enumerate() {
            if day.date().1 > self.current_year {
                if let Some(day) = self.days.get(i - 1) {
                    println!("Day: {}, Year {}", day.date().0, day.date().1);
                    day.run();
                }

                return;
            }
        }
    }
}