#[derive(Debug)]
enum Daterr {
    InvalidDay,
    InvalidMonth,
    InvalidYear,
}

use std::io;
#[derive(Debug)]
struct Date {
    day: u16,
    month: u8,
    year: u32,
}

fn main() {
    let mut day = String::new();
    println!("Enter day");
    io::stdin().read_line(&mut day).expect("Invalid day format");
    let day: u16 = day.trim().parse().expect("Please enter in u16 format");
    println!("Enter month");
    let mut month = String::new();
    io::stdin()
        .read_line(&mut month)
        .expect("Invalid month format");
    let month: u8 = month.trim().parse().expect("Please enter in u16 format");
    println!("Enter year");
    let mut year = String::new();
    io::stdin()
        .read_line(&mut year)
        .expect("Invalid year format");
    let year: u32 = year.trim().parse().expect("Please enter in u16 format");

    let date_a = Date {
        day: day,
        month: month,
        year: year,
    };
    println!("{:?}",date_a);
    fn get_date(date: &Date) -> Result<(), Daterr> {
        
        
        if date.month < 1 || date.month > 12 {
            Err(Daterr::InvalidMonth)
        } else if date.day < 1 || date.day > 31 {
            Err(Daterr::InvalidDay)
        } else if date.year < 1 {
            Err(Daterr::InvalidYear)
        } else {
            Ok(println!("Date : {:?}",date))
        }
    }
    println!("{:?}", get_date(&date_a));
    match get_date(&Date { day, month, year}) {
        Ok(date) => println!("{:?}" ,date),
        Err(e) => println!("Invalid Date format : {:?}", e)
    };

    

}
