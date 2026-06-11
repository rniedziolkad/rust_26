use std::ops::{Add, Sub, Mul, Div, AddAssign};
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Ulamek {
    licznik: i32,
    mianownik: i32,
}

impl Ulamek {
    pub fn new(licznik: i32, mianownik: i32) -> Self {
        if mianownik == 0 {
            panic!("mianownik nie może być równy 0");
        }
        let mut u = Ulamek {licznik, mianownik};
        u.skracaj();
        u
    }

    pub fn as_f64(&self) -> f64 {
        self.licznik as f64 / self.mianownik as f64
    }

    pub fn licznik(&self) -> i32 {
        self.licznik
    }

    pub fn mianownik(&self) -> i32 {
        self.mianownik
    }

    fn nwd(a: i32, b: i32) -> i32 {
        let mut a = a.abs();
        let mut b = b.abs();

        while b != 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }

        a
    }

    fn skracaj(&mut self) {
        let nwd = Self::nwd(self.licznik, self.mianownik);
        self.licznik /= nwd;
        self.mianownik /= nwd;
        if self.mianownik < 0 {
            self.licznik *= -1;
            self.mianownik *= -1;
        }
    }
}

impl Add for Ulamek {
    type Output = Ulamek;
    fn add(self, rhs: Ulamek) -> <Self as Add<Ulamek>>::Output { 
        Ulamek::new(
            self.licznik * rhs.mianownik + rhs.licznik * self.mianownik,
            self.mianownik * rhs.mianownik,
        ) 
    }
}

impl Sub for Ulamek {
    type Output = Ulamek;
    fn sub(self, rhs: Ulamek) -> <Self as Add<Ulamek>>::Output { 
        Ulamek::new(
            self.licznik * rhs.mianownik - rhs.licznik * self.mianownik,
            self.mianownik * rhs.mianownik,
        ) 
    }
}

impl Mul for Ulamek {
    type Output = Ulamek;
    fn mul(self, rhs: Ulamek) -> <Self as Mul<Ulamek>>::Output {
        Ulamek::new(
            self.licznik * rhs.licznik,
            self.mianownik * rhs.mianownik,
        )
    }
}

impl Div for Ulamek {
    type Output = Ulamek;
    fn div(self, rhs: Ulamek) -> <Self as Div<Ulamek>>::Output {
        Ulamek::new(
            self.licznik * rhs.mianownik,
            self.mianownik * rhs.licznik,
        )
    }
}

impl AddAssign for Ulamek {
    fn add_assign(&mut self, rhs: Ulamek) { 
        self.licznik = self.licznik * rhs.mianownik + rhs.licznik * self.mianownik;
        self.mianownik = self.mianownik * rhs.mianownik;
        self.skracaj();
    }
}

impl FromStr for Ulamek {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 1 {
            let licznik: i32 = parts[0].trim().parse().map_err(|_| "Błędny licznik".to_string())?;
            return Ok(Ulamek::new(licznik, 1));
        }
        if parts.len() != 2 {
            return Err("Zły format ułamka".to_string());
        }
        let licznik: i32 = parts[0].trim().parse().map_err(|_| "Błędny licznik".to_string())?;
        let mianownik: i32 = parts[1].trim().parse().map_err(|_| "Błędny mianownik".to_string())?;
        Ok(Ulamek::new(licznik, mianownik))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_assign() {
        let mut u1 = Ulamek::new(1, 3);
        let u2 = Ulamek::new(1, 2);
        u1 += u2;
        assert_eq!(u1, Ulamek::new(5, 6));
    }

    #[test]
    fn test_as_f64() {
        let u1 = Ulamek::new(3, 4);
        assert_eq!(u1.as_f64(), 0.75);
    }

    #[test]
    fn test_licznik_mianownik(){
        let u = Ulamek::new(4,-8);
        assert_eq!(u.licznik(), -1);
        assert_eq!(u.mianownik(), 2);
    }

    #[test]
    fn test_add() {
        let u1 = Ulamek::new(1, 3);
        let u2 = Ulamek::new(1, 2);
        assert_eq!(u1 + u2, Ulamek::new(5, 6));
    }
    #[test]
    fn test_sub() {
        let u1 = Ulamek::new(1, 2);
        let u2 = Ulamek::new(1, 4);
        assert_eq!(u1 - u2, Ulamek::new(1, 4));
    }
    #[test]
    fn test_mul() {
        let u1 = Ulamek::new(1, 2);
        let u2 = Ulamek::new(1, 2);
        assert_eq!(u1 * u2, Ulamek::new(1, 4));
    }
    #[test]
    fn test_div() {
        let u1 = Ulamek::new(1, 4);
        let u2 = Ulamek::new(1, 4);
        assert_eq!(u1 / u2, Ulamek::new(1, 1));
    }

    #[test]
    #[should_panic]
    fn test_zerowy_mianownik() {
        let _ = Ulamek::new(1, 0);
    }

    #[test]
    fn test_rozne_zapisy_tego_samego_ulamka() {
        assert_eq!(Ulamek::new(1, -3), Ulamek::new(-2, 6));
    }

    #[test]
    fn test_z_napisu_1() {
        let u1 = Ulamek::from_str("1/-3").unwrap();
        let u2 = Ulamek::from_str("-2/6").unwrap();
        assert_eq!(u1, u2);
        assert_eq!(u1, Ulamek::new(-1, 3));
    }

    #[test]
    fn test_z_napisu_2() {
        let u1 = Ulamek::from_str("13").unwrap();
        let u2 = Ulamek::from_str("-26/-2").unwrap();
        assert_eq!(u1, u2);
        assert_eq!(u1, Ulamek::new(13, 1));
    }

    #[test]
    #[should_panic]
    fn test_z_blednego_napisu_1() {
        let _ = Ulamek::from_str("x/-3").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_z_blednego_napisu_2() {
        let _ = Ulamek::from_str("1/3/5").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_z_blednego_napisu_3() {
        let _ = Ulamek::from_str("/5").unwrap();
    }
}
