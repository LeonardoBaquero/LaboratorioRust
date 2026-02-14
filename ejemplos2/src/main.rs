use std::cmp::Ordering;
use std::io;

use rand::Rng;


fn main() {
    println!("Adivina el número"); 
    println!("Ingrese su número a adivinar"); 
    
    let numero_secreto: u32 = rand::thread_rng().gen_range(1..=100);
    let mut adivinanza  = String::new(); 

        
    
    io::stdin()
       .read_line(&mut adivinanza)
       .expect("No se pudo leer la línea"); 

    let adivinanza: u32 = adivinanza.trim().parse().expect("Por favor escriba un número"); 
        
    println!("Su adivinanza es : {}", adivinanza); 
    println!("El número secreto es: {}", numero_secreto); 

        match adivinanza.cmp(&numero_secreto) {
            Ordering::Less => println!("Muy pequeño"),
            Ordering::Greater => println!("Muy grande"),
            Ordering::Equal => println!("Ganaste")
                        
        }
    
    }




