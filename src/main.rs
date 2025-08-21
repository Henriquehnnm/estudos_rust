use std::io;

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut resultado: String = String::new();
    io::stdin().read_line(&mut resultado).unwrap();
    resultado.trim().to_string()
}

fn main() {
    println!("== ÍMPAR OU PAR ==");
    let input_number: String = input("Digite um numero para ver e é impar ou par:");
    let number: i64 = input_number
        .trim()
        .parse()
        .expect("Digite un número válido...");
    if number % 2 == 0 {
        println!("O número {number} é par!")
    } else {
        println!("O número {number} é ímpar!")
    }
}
