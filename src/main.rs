use rand::seq::IteratorRandom;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    // Importar archivo
    let message = fs::read_to_string("words.txt")?;
    // Get variables
    let mut rng = rand::thread_rng();

    let var = message
        .lines()
        .map(|line| line.split_once(',').expect("No hay ,"))
        .choose(&mut rng)
        .expect("Siempre hay lineas para elegir");

    // Devolver llave random
    println!("{}", var.0);
    // Enter para ver respuesta
    io::stdin().read_line(&mut String::new()).unwrap();
    println!("{}", var.1);
    Ok(())
}

#[allow(unused)]
fn main_first_try() -> Result<(), Box<dyn Error>> {
    // Importar archivo
    let message = fs::read_to_string("words.txt")?;
    // Get variables
    let v: Vec<&str> = message.lines().collect();
    // Crear diccionario
    let mut dic_words = HashMap::new();
    for element in v.iter() {
        let word = element.split_once(',').expect("No hay ,");
        dic_words.insert(word.0, word.1);
    }
    // Devolver llave random
    let mut rng = rand::thread_rng();
    let var = dic_words.iter().choose(&mut rng).unwrap();
    println!("{}", var.0);
    // Enter para ver respuesta
    io::stdin().read_line(&mut String::new()).unwrap();
    println!("{}", var.1);
    Ok(())
}
