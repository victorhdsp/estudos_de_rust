pub mod features;

use features::age_calculator;
use std::io::{self, Write};

pub fn execute() {
    println!("Este é um calculador de idade, baseado no ano em que você nasceu.");

    print!("Em qual ano você nasceu? ");
    io::stdout().flush().expect("Falha ao limpar o buffer de saída");
    let mut year = String::new();
    let _ = io::stdin().read_line(&mut year);
    
    print!("Em qual mês você nasceu? ");
    io::stdout().flush().expect("Falha ao limpar o buffer de saída");
    let mut month = String::new();
    let _ = io::stdin().read_line(&mut month);

    let age = age_calculator::calcule(year, month);

    if age.is_some() {
        println!("Sua idade é {}", age.unwrap())
    }
}
