// main.rs

fn main() {
    // Ownership konsepti
    let string1 = String::from("Merhaba"); // string1 sahibi
    let _string2 = string1; // _string2 şimdi sahibi oldu, string1 kullanılamaz

    // Borrowing konsepti
    let sayi = 42;
    fonksiyonla_ode(&sayi); // sayi'yi fonksiyona ödünç verdik
}

fn fonksiyonla_ode(deger: &i32) {
    println!("Fonksiyonla ödünç alınan değer: {}", deger);
}
