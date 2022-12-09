use chrono::Datelike;
use seq_macro::seq;
use crate::{year_2021, year_2022};

pub trait Day {
    fn date(&self) -> (i32, i32);
    fn run(&self);
}

pub struct AdventRunner {
    days: Vec<Box<dyn Day>>,
    current_year: i32,
}

impl AdventRunner {
    pub fn new() -> Self {
        let mut days: Vec<Box<dyn Day>> = Vec::new();
        
        // check how many folders there are
        // check the year
        seq!(N in 1..=3 {
            days.push(Box::new(year_2021::day~N::Day~N));
        });
        
        
        AdventRunner {
            current_year: chrono::Utc::now().year(),
            days: days
            // days: vec![
            //     Box::new(year_2021::day1::Day1),
            //     Box::new(year_2021::day2::Day2),
            //     Box::new(year_2021::day3::Day3),
            //     Box::new(year_2022::day1::Day1),
            //     Box::new(year_2022::day2::Day2),
            //     Box::new(year_2022::day3::Day3),
            //     Box::new(year_2022::day4::Day4),
            //     Box::new(year_2022::day5::Day5),
            //     Box::new(year_2022::day6::Day6),
            //     Box::new(year_2022::day7::Day7),
            //     Box::new(year_2022::day8::Day8)
            // ]
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
        
        // at this point, just run the latest
        let day = self.days.last().unwrap();
        println!("Day: {}, Year {}", day.date().0, day.date().1);
        
        day.run();
    }
}