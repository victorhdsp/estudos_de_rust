pub mod features;

use std::io::{self, Write};
use crate::features::fahrenheit_to_celsius;


pub fn execute() {
    println!("Olá, esse é um conversor de Fahrenheit para Célsius");
    
    print!("Escolha um valor em Fahrenheit: ");
    io::stdout().flush().expect("Falha ao limpar o buffer de saída");
    
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    let result:Option<f32> = fahrenheit_to_celsius::calcule(input);
    println!("O resultado é: {}", result.unwrap())
}