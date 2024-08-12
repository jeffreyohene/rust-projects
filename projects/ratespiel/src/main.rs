use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Willkommen bei: Errate die Nummer!");

    let geheim_nummer = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Gib bitte Deine Schätzung ein:");

        let mut schaetzung = String::new();

        io::stdin()
            .read_line(&mut schaetzung)
            .expect("Zeile konnte nicht gelesen werden");

        let schaetzung: u32 = match schaetzung
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            

        println!("Du hast {} erraten", schaetzung);

        match schaetzung.cmp(&geheim_nummer) {
            Ordering::Less => println!("zu klein!"),
            Ordering::Greater => println!("zu groß!"),
            Ordering::Equal => {
                println!("RICHTIG! Du bist der HAMMER, toll gemacht!");
                break
            }
        }
    } 
}