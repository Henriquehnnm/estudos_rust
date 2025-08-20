use std::io;

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut resultado: String = String::new();
    io::stdin().read_line(&mut resultado).unwrap();
    resultado.trim().to_string()
}

fn main() {

    let mut pont: f64 = 0.0;

    // Pergunta 1
    println!("\n== Responda as perguntas a seguir: ==\n");
    println!("Em que ano o linux foi criado?\n a) 1992\n b) 1988\n c) 1990\n d) 1991\n");
    let r1: String = input("Responta:");
    if r1 == "d" {
        println!("\nResposta correta!\n");
        pont += 1.0;
    } else {
        println!("\nResposta incorreta...\n")
    }

    // Pergunta 2
    println!("O que o comando chmod +x script.sh faz?\n a) Deleta o arquivo\n b) Compila o script\n c) Torna o script executável\n d) Abre o script no editor\n");
    let r2: String = input("Responta:");
    if r2 == "c" {
        println!("\nResposta correta!\n");
        pont += 1.0;
    } else {
        println!("\nResposta incorreta...\n")
    }

    // Pergunta 3
    println!("Qual comando é usado para listar todos os arquivos e diretórios, incluindo os ocultos?\n a) ls -l\n b) ls -a\n c) ls -h\n d) ls -r\n");
    let r3: String = input("Responta:");
    if r3 == "b" {
        println!("\nResposta correta!\n");
        pont += 1.0;
    } else {
        println!("\nResposta incorreta...\n")
    }

    // Pergunta 4
    println!("Qual diretório contém os arquivos de configuração do sistema no Linux?\n a) /home\n b) /var\n c) /etc\n d) /usr\n");
    let r4: String = input("Responta:");
    if r4 == "c" {
        println!("\nResposta correta!\n");
        pont += 1.0;
    } else {
        println!("\nResposta incorreta...\n")
    }

    // Pergunta 5
    println!("Qual comando é usado para procurar por texto dentro de arquivos?\n\na) grep\nb) locate\nc) find\nd) search\n");
    let r5: String = input("Responta:");
    if r5 == "a" {
        println!("\nResposta correta!\n");
        pont += 1.0;
    } else {
        println!("\nResposta incorreta...\n")
    }

    // Final
    println!("\nSua pontuação foi de {} pontos!", pont);
    if pont >= 3.0 {
        println!("Parabéns, vc sabe bastante de linux!")
    } else {
        println!("Não desista, continue estudando!")
    }
}