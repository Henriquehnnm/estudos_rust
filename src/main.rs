use colored::*;
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io;

fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}

#[derive(Serialize, Deserialize, Debug)]
struct Users {
    user: HashMap<String, UserData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserData {
    pass: String,
}

fn load_users() -> Users {
    let content = fs::read_to_string("login.toml").unwrap_or_else(|_| String::new());
    if content.is_empty() {
        Users {
            user: HashMap::new(),
        }
    } else {
        toml::from_str(&content).unwrap_or(Users {
            user: HashMap::new(),
        })
    }
}

fn save_users(users: &Users) {
    let toml_str = toml::to_string(&users).unwrap();
    fs::write("login.toml", toml_str).unwrap();
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut resultado: String = String::new();
    io::stdin().read_line(&mut resultado).unwrap();
    resultado.trim().to_string()
}

fn main() {
    println!("{}", "\n== LOGIN ==".green());
    loop {
        println!("Selecione a opção:\n Criar conta(n)\n Logar(l)\n sair(e)\n");
        let op: String = input("Prompt: ");
        match op.as_str() {
            "n" => new_user(),
            "l" => login(),
            "e" => break,
            _ => println!("{}", "ERROR: opção invália!\n".red()),
        }
    }
}

fn new_user() {
    let mut users = load_users();

    println!("{}", "\n== Criar nova conta ==".blue());
    let username = input("Digite seu user name: ");

    if users.user.contains_key(&username) {
        println!("{}", "Usuário já existe!".red());
        return;
    }

    println!("Digite sua senha: ");
    let pass = read_password().unwrap();

    println!("Confirme sua senha: ");
    let confirm = read_password().unwrap();

    if pass != confirm {
        println!("{}", "As senhas não são iguais!".red());
        return;
    }

    let hashed = hash_password(&pass);
    users
        .user
        .insert(username.clone(), UserData { pass: hashed });

    save_users(&users);
    println!(
        "{}",
        format!("Usuário {} criado com sucesso!", username).green()
    );
}

fn login() {
    let users = load_users();

    let username = input("Digite seu username: ");
    if !users.user.contains_key(&username) {
        println!("{}", "Usuário não encontrado!".red());
        return;
    }

    println!("Digite sua senha: ");
    let password = read_password().unwrap();
    let hashed_input = hash_password(&password);

    let stored_hash = &users.user[&username].pass;

    if &hashed_input == stored_hash {
        println!(
            "{}",
            format!("Login bem-sucedido! Bem-vindo, {}!", username).green()
        );
    } else {
        println!("{}", "Senha incorreta!".red());
    }
}
