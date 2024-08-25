mod utils;
mod models;
mod view;

use crate::models::student::Student;
use crate::utils::terminal::print_menu;
use crate::view::student::{create_student, insert_initial_students, list_all_students};


fn main() {
    let mut all_students: Vec<Student> = Vec::new();

    let menu_options = [
        "Cadastrar aluno",
        "Listar alunos",
        "INserir alunos iniciais",
    ];

    loop {
        let selected_option = print_menu(
            "Menu inicial",
            &menu_options,
            false,
        );

        match selected_option {
            1 => create_student(&mut all_students),
            2 => list_all_students(&all_students),
            3 => insert_initial_students(&mut all_students),
            _ => break
        }
    }
}
