use std::io::{self, Write};

fn main() {
    loop {
        print!("Digite um número ou 'sair' para sair: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "sair" {
            println!("Saindo...");
            break;
        }

        let numero1 = input.trim().parse::<f64>().unwrap();

        print!("Digite outro número: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let numero2 = input.trim().parse::<f64>().unwrap();

        let resultado = numero1 + numero2;

        println!("O resultado da soma é {}", resultado);
    }
}
