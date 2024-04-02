pub mod features;
use features::guess_the_number;
use features::random_value;
use std::io::{self, Write};

pub fn execute() {
    println!("Você precisa descobrir o numero que o software escolheu");
    println!("");

    let expected = random_value::generate();
    let mut pass: bool = false;
    
    while !pass {
        print!("Escolha um numero entre 1 e 100: ");
        io::stdout().flush().expect("Erro ao limpar o fluxo");
        
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        let result = guess_the_number::search(expected, input);

        if result == 0 {
            pass = true
        } else if result < 0 {
            println!("O numero é maior!")
        } else {
            println!("O numero é menor")
        }

        println!("")
    }

    println!("Parabens você conseguiu !")
}
