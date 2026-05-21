use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd)]
enum Kolor {
    Trefl,
    Karo,
    Kier,
    Pik,
}

enum Error {
    NoError,
    BadFormat,
    NotExist(String),
    TooLarge(i32, i32)
}

impl Error {
    fn pokaz_komunikat(&self) {
        match self {
            Error::NoError => {
                println!("Brak błędu");
            },
            Error::BadFormat => {
                println!("Zły format pliku");
            },
            Error::NotExist(filename) => {
                println!("{filename} nie istnieje");
            },
            Error::TooLarge(current, max) => {
                println!("Maksymalny rozmiar pliku to {max}. Aktualny to {current}");
            }
        }
    }
}

fn main() {
    let pik = Kolor::Pik;
    let kier = Kolor::Kier;
    let karo = Kolor::Karo;
    let trefl = Kolor::Trefl;
    println!("{}", pik > kier);
    println!("{}", kier > karo);
    println!("{}", karo > trefl);

    let brak = Error::NoError;
    brak.pokaz_komunikat();
    let format = Error::BadFormat;
    format.pokaz_komunikat();
    let exists = Error::NotExist("plik.txt".to_string());
    exists.pokaz_komunikat();
    let large = Error::TooLarge(1200, 1000);
    large.pokaz_komunikat();
    println!();

    // zestaw 9b -----------------
    // własna implementacja cech wbudowanych
    let d1 = Date::from_3(21, Month::Maj, 2026);
    let d2 = Date::from_string(&d1.to_string(), '-');
    println!("{}", d2.to_string());
}

#[derive(PartialEq, PartialOrd)]
struct Time {
    hour: u8,
    minute: u8,
    second: u8,
}

impl Time {
    fn from_3(hour: u8, minute: u8, second: u8) -> Time {
        Time { hour, minute, second }
    }
    fn to_string(&self) -> String {
        format!("{}:{}:{}", self.hour, self.minute, self.second)
    }
    fn from_string(string: &str, delim: char) -> Time {
        let parts: Vec<&str> = string.split(delim).collect();
        let hour: u8 = parts[0].parse().unwrap();
        let minute: u8 = parts[1].parse().unwrap();
        let second: u8 = parts[2].parse().unwrap();
        Time { hour, minute, second }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Month {
    Styczen, Luty, Marzec, Kwiecien, Maj, Czerwiec, Lipiec, 
    Sierpien, Wrzesien, Pazdziernik, Listopad, Grudzien
}

impl FromStr for Month {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> { 
        match s {
            "1" => Ok(Month::Styczen),
            "2" => Ok(Month::Luty),
            "3" => Ok(Month::Marzec),
            "4" => Ok(Month::Kwiecien),
            "5" => Ok(Month::Maj),
            "6" => Ok(Month::Czerwiec),
            "7" => Ok(Month::Lipiec),
            "8" => Ok(Month::Sierpien),
            "9" => Ok(Month::Wrzesien),
            "10" => Ok(Month::Pazdziernik),
            "11" => Ok(Month::Listopad),
            "12" => Ok(Month::Grudzien),
            _ => Err("Nieprawidłowy miesiąc".to_string()),
        }
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        let month: u8 = match self {
            Month::Styczen => 1,
            Month::Luty => 2,
            Month::Marzec => 3,
            Month::Kwiecien => 4,
            Month::Maj => 5,
            Month::Czerwiec => 6,
            Month::Lipiec => 7,
            Month::Sierpien => 8,
            Month::Wrzesien => 9,
            Month::Pazdziernik => 10,
            Month::Listopad => 11,
            Month::Grudzien => 12,
        };
        write!(f, "{}", month)
    }
}
struct Date {
    day: u8,
    month: Month,
    year: u16,
    time: Option<Time>,
}

impl Date {
    fn to_string(&self) -> String {
        format!("{}-{}-{}", self.day, self.month, self.year)
    }

    fn from_3(day: u8, month: Month, year: u16) -> Date {
        Date { day, month, year, time: None }
    }

    fn from_string(string: &str, delim: char) -> Date {
        let parts: Vec<&str> = string.split(delim).collect();
        let day: u8 = parts[0].parse().unwrap();
        let month: Month = parts[1].parse().unwrap();
        let year: u16 = parts[2].parse().unwrap();
        Date { day, month, year, time: None }
    }

    fn has_time(&self) -> bool {
        self.time != None
    }
    fn set_time(&mut self, time: Time) {
        self.time = Some(time);
    }
    fn clear_time(&mut self) {
        self.time = None;
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Date) -> bool { 
        self.year == other.year 
         && self.month == other.month
         && self.day == other.day
         && self.time == other.time
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<std::cmp::Ordering> { 
         let res = self.year.cmp(&other.year);
         if res != Ordering::Equal {
            return Some(res);
         }
         let res = self.month.cmp(&other.month);
         if res != Ordering::Equal {
            return Some(res);
         }
         let res = self.day.cmp(&other.day);
         if res != Ordering::Equal {
            return Some(res);
         }
         
         if self.has_time() && other.has_time() {
            return self.time.partial_cmp(&other.time);
         }

         if !self.has_time() && !other.has_time() {
            return Some(Ordering::Equal);
         }
         
         return None;
    }
}
