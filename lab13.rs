#[derive(Debug)]
struct Ksiazka {
    tytul: String,
    autor: String,
    strony: u16,
    gatunek: Gatunek
}

#[derive(Debug)]
enum Gatunek {
    Fantastyka,
    Scifi,
    Dokument
}

#[derive(Debug, Clone, PartialEq)]
enum Status { 
    Otwarte, 
    Przetwarzane, 
    Zamkniete(Rezultat) 
}

#[derive(Debug, Clone)]
enum Rezultat{
    Przyjete,
    Odrzucone
}

#[derive(Debug, Clone)]
struct Zgloszenie{
    id: u32
    tytul: String
    status: Status
}

#[derive(Debug, Clone)]
struct Zgloszenia{
    zgloszenia: Vec<Zgloszenie>
}
#[derive(Debug, Clone)]
impl Zgloszenie{
    fn new(id: u32, tytul: &str, status: Status) -> Zgloszenie {
        Zgloszenie{
            id,
            tytul,
            status
        }
    }
}
#[derive(Debug, Clone)]
impl Zgloszenia{
    fn new() -> Zgloszenia {
        Zgloszenia{
            zgloszenia: Vec::new()
        }
    }

    fn update_status(&mut self, id: u32, new_status: Status){
        // do poprawy
        let mut found = self.zgloszenia.into_iter().find(|x| x.id == id);
        if found.is_some(){
            found.unwrap().status = new_status;
        }
    }
    fn list_by_status(&self, status: Status) -> Vec<&Zgloszenie>{
        self.zgloszenia.iter().filter(|x| x.status == status).collect()
    }
    fn add(&mut self, zgloszenie: Zgloszenie){
        self.zgloszenia.push(zgloszenie);
    }

}



fn main() {
    let ksiazka1 = Ksiazka{tytul: "Wiedźmin".to_string(), autor: "Sapkowki".to_string(), strony: 350, gatunek: Gatunek::Fantastyka};
    let ksiazka2 = Ksiazka{tytul: "A".to_string(), autor: "B".to_string(), strony: 400, gatunek: Gatunek::Scifi};
    let ksiazka3 = Ksiazka{tytul: "C".to_string(), autor: "D".to_string(), strony: 200, gatunek: Gatunek::Dokument};
  
    let ksiazki: Vec<Ksiazka> = vec![ksiazka1, ksiazka2, ksiazka3];

    println!("{:?} ", ksiazki);
    println!("{:?}", MoreThan300(&ksiazki));

    let mut zgloszenia = Zgloszenia::new();
    let mut zgloszenie1 = Zgloszenie::new(1, "aaa", Status::Otwarte);
    let mut zgloszenie2 = Zgloszenie::new(2, "aad", Status::Przetwarzane);
    let mut zgloszenie3 = Zgloszenie::new(3, "aac", Status::Otwarte);
    zgloszenia.add(zgloszenie1);
    zgloszenia.add(zgloszenie2);
    zgloszenia.add(zgloszenie3);

    zgloszenia.update_status(1, Status::Przetwarzane);
    println!("{:?}", zgloszenia.list_by_status(Status::Przetwarzane));

}

fn MoreThan300(x: &Vec<Ksiazka>) -> Vec<&Ksiazka> {
    x.iter().filter(|e| e.strony > 300).collect()
}

