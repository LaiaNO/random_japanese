use rand::seq::IteratorRandom;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;

mod category;
mod words;

use category::Category;
use words::Words;

fn main() -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir("Words")?
        .map(|res| res.map(|e| Category(e.path())))
        .collect::<Result<Vec<_>, io::Error>>()?;

    println!("Select one category:");
    for (index, category) in files.iter().enumerate() {
        println!("{}: {}", index, category.file_name());
    }
    let mut category = String::new();
    io::stdin().read_line(&mut category)?;

    // Importar archivo
    let category: usize = category.trim().parse()?;
    let message = fs::read_to_string(&files[category].0)?;
    let mut words = Words::new(&message);

    let mut user_entry = String::new();
    loop {
        user_entry.clear();
        let (eng, jap) = words.random().expect("Siempre hay lineas para elegir");

        // Devolver llave random
        println!("{}", eng);
        // Enter para ver respuesta
        io::stdin().read_line(&mut user_entry).unwrap();
        if user_entry.trim() == "exit" {
            break;
        }
        println!("{}", jap);
    }
    Ok(())
}

#[allow(unused)]
fn main_second_try() -> Result<(), Box<dyn Error>> {
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
