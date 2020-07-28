use std::io;
use std::process::exit;

fn main() {
    println!("Digite um número para calcular o fatorial");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Falha ao ler a linha");

    let number: u16 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada inválida");
            exit(1);
        }
    };

    if number > 20 {
        println!("Não é possível calcular fatorial para este número");
        exit(1);
    }

    let result = factorial(number, 1);
    
    println!("Resultado: {}", result);
}


fn factorial(number: u16, result: u64) -> u64 {
    if number == 1 {
        return result;
    }
    else {
        return factorial(number - 1, result*(number as u64))
    }
}
