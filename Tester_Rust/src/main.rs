use std::io;

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut resultado: String = String::new();
    io::stdin().read_line(&mut resultado).unwrap();
    resultado.trim().to_string()
}

fn main() {
    println!("Digite seu nome: ");
    let nome = input("Digite:");
    println!("Olá {nome}!")
}
