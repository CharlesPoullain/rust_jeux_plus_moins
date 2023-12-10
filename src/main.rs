use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Devinez le nombre !");
    let number_secret = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Veuillez saisir un nombre :");
        let mut supposition = String::new();


        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");

        let supposition : u32 = match supposition.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match supposition.cmp(&number_secret) {
            Ordering::Less => println!("Trop petit !"),
            Ordering::Greater => println!("Trop grand !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
        println!("Votre nombre : {}", supposition);
    }
}
