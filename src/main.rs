use crate::Daterror::Invalidmonth;
use crate::Daterror::Invalidday;
use std::error::Error;
use std::fmt;


#[derive(Debug)]
enum Daterror {
    Invalidday,
    Invalidmonth

}

#[derive(Debug)]
struct Date {
    day: u8,
    month: u8,
    year: i16
}

impl fmt::Display for Daterror {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Daterror::Invalidday => write!(f, "Day {} is outside range!", Invalidday),
            &Daterror::Invalidmonth => write!(f, "Month {} is outside range!", Invalidmonth),
        }
    }
}

impl Error for Daterror {
    fn description(&self) -> &str {
        match self {
            &Daterror::Invalidday => "Day is outside range!",
            &Daterror::Invalidmonth => "Month is outside range!",
        }
    }

}
use std::io;

fn main() {
    println!("ENTER DAY");
    let mut day = String::new();
    io::stdin()
       .read_line(&mut day)
       .expect("invalid input");

    let day: u8 = day.trim().parse().ok().expect("invalid input");

    println!("ENTER MONTH");
    let mut month = String::new();
    io::stdin()
       .read_line(&mut month)
       .expect("invalid input");

    let month: u8 = month.trim().parse().ok().expect("invalid input");

    println!("ENTER YEAR");
    let mut year = String::new();
    io::stdin()
       .read_line(&mut year)
       .expect("invalid input");

    let year: i16 = year.trim().parse().ok().expect("invalid input");


    let date = Date {
        day: day,
        month: month,
        year: year
    };

    fn validate(date: &Date) -> Result<(), Daterror>  {
        if date.month < 1 || date.month > 12 {
        Err(Daterror::Invalidmonth)
    }   else if date.day < 1 || date.day > 31 {
        Err(Daterror::Invalidday)
    }   else {
        Ok(())
    }
}
fn add_days(date: Date, days: i32) -> Result<Date, Daterror> {
    validate(&date)?; 
   
    Ok(date)
}
println!("{:?}",validate(&date));
}