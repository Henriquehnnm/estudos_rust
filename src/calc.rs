use std::io;

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut resultado = String::new();
    io::stdin().read_line(&mut resultado).unwrap();
    resultado.trim().to_string()
}

fn main() {
    let n1: f64 = input("Digite o primeiro número: ").parse().unwrap();
    let op: String = input("Digite o operador: (+, -, *, /)") ;
    let n2: f64 = input("Digite o segundo número: ").parse().unwrap();
    if op == "+" {
        print!("{}", n1 + &n2);
    }
    if op == "-" {
        print!("{}", n1 - &n2);
    }
    if op == "*" {
        print!("{}", n1 * &n2);
    }
    
    if op == "/" {
        if n2 == 0.0 {
            println!("\x1b[31mERRO\x1b[0m: Divisão por 0!");
        } else {
            print!("{}", n1 / n2)
        }
    }
}