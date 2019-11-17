use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Raad het nummer!");
    let  secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Het nummer is?");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed read_ line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Je hebt {} gegokt", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Het getal is te klein"),
            Ordering::Greater => println!("Het getal is te groot"),
            Ordering::Equal => {
                println!("Je hebt gewonnen");
                break;
            },
        }
    }
}
