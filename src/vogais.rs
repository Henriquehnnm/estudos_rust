use std::io;

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut resultado: String = String::new();
    io::stdin().read_line(&mut resultado).unwrap();
    resultado.trim().to_string()
}

fn main() {
    println!("== CONTADOR DE VOGAIS ==");
    let frase: String = input("Digite a frase para contar suas vogais:");
    let mut contador = 0;
    for character in frase.chars() {
        if "aeiouAEIOU".contains(character) {
            contador += 1;
        }
    }
    println!("Sua frase tem {contador} vogais")
}