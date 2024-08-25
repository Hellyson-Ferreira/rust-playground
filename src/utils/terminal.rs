use std::io::Write;
use rpassword::prompt_password;
pub fn clear_terminal() {
    print!("{esc}c", esc = 27 as char);
}
fn print_items(items: &[&str]) {
    for (i, item) in items.iter().enumerate() {
        println!("{} - {}", i + 1, item);
    }
}

pub fn esperar_enter() {
    prompt_password("Pressione ENTER para continuar...").unwrap();
}
pub fn print_menu(title: &str, items: &[&str], exit: bool) -> u32 {
    clear_terminal();

    let full_title = String::from("Gestor de Notas:: ") + title;
    println!("{full_title}");
    println!("{}", String::from("=").repeat(full_title.len()));

    print_items(items);

    println!("{}", if exit { "* - Sair" } else { "* - Voltar" });
    print!("\nEscolha uma opção: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let op: Result<u32, _> = input.trim().parse();

    match op {
        Ok(op) => op,
        _ => 0,
    }
}

