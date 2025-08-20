use std::io;

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut resultado: String = String::new();
    io::stdin().read_line(&mut resultado).unwrap();
    resultado.trim().to_string()
}

fn main() {
    let frase: String = input("Digite uma frase:");
    let npalavras: usize = frase.split_whitespace().count();
    if npalavras == 1 {
        println!("Sua frase tem {npalavras} palavra")
    } else {
        println!("Sua frase tem {npalavras} palavras")
    }
}