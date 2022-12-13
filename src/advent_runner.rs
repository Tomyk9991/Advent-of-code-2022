use chrono::Datelike;
use seq_macro::seq;
use crate::{year_2021, year_2022};


#[macro_export]
macro_rules! amount_2022 {
    ($expand:path) => {
        $expand!(13);
    };
}

#[macro_export]
macro_rules! amount_2021 {
    ($expand:path) => {
        $expand!(3);
    };
}

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
        
        macro_rules! expand_2021 {
            ($limit:literal) => {
                seq!(N in 1..=$limit {
                    days.push(Box::new(year_2021::day~N::Day~N));
                });
            }
        }

        macro_rules! expand_2022 {
            ($limit:literal) => {
                seq!(N in 1..=$limit {
                    days.push(Box::new(year_2022::day~N::Day~N));
                });
            }
        }
        
        amount_2021!(expand_2021);
        amount_2022!(expand_2022);
        
        
        AdventRunner {
            current_year: chrono::Utc::now().year(),
            days
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