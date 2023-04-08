use std::io::{self, Write};

fn main() {
    loop {
        print!("Digite uma expressão matemática (+, -, *, /) ou 'sair' para sair: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "sair" {
            println!("Saindo...");
            break;
        }

        let parts = input.trim().split_whitespace().collect::<Vec<&str>>();

        let operator = parts[1];
        let n1 = parts[0].parse::<f64>().unwrap();
        let n2 = parts[2].parse::<f64>().unwrap();

        let resultado = match operator {
            "+" => n1 + n2,
            "-" => n1 - n2,
            "*" => n1 * n2,
            "/" => n1 / n2,
            _ => {
                println!("Operador inválido");
                continue;
            }
        };

        println!("O resultado é {}", resultado);
    }
}
