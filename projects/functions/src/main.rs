fn main() {
    druck_beschriftete_messungen(15, 'm');
}

fn druck_beschriftete_messungen(wert:i32, einheit_etikett: char) {
    println!("die Messung lautet: {wert}{einheit_etikett}");
}