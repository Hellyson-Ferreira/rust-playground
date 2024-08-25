use crate::models::student::Student;
use crate::utils::terminal::esperar_enter;
use cli_table::{print_stdout, Style, Table, TableStruct};
use std::io;

pub fn create_student(all_students_base_data: &mut Vec<Student>) {
    let mut name = String::new();
    let mut age = String::new();
    let id = all_students_base_data.len() + 1; // This is a temporary solution, we need to change this to a better solution

    println!("Digite o nome do aluno");
    io::stdin().read_line(&mut name).unwrap();

    println!("Digite a idade do aluno");
    io::stdin().read_line(&mut age).unwrap();

    let age: usize = age.trim().parse().expect("Invalid Age!");

    all_students_base_data.push(Student::create_student(id, name.clone().trim().to_string(), age));

    println!("Aluno {name} cadastrado")
}


pub fn list_all_students(all_students_base_data: &Vec<Student>) {
    let mut vec_student_to_print: Vec<Vec<String>> = vec![];

    for all_students_base_datum in all_students_base_data {
        vec_student_to_print.push(
            vec![
                all_students_base_datum.id.to_string(),
                all_students_base_datum.name.clone(),
                all_students_base_datum.age.to_string(),
                all_students_base_datum.get_string_notas(),
                all_students_base_datum.average.to_string(),
            ]
        )
    }


    let table: TableStruct = vec_student_to_print
        .table()
        .title(vec!["ID", "Name", "Age", "Notas", "Média"])
        .bold(true);

    print_stdout(table).unwrap();
    esperar_enter();
}

pub fn insert_initial_students(all_students_base_data: &mut Vec<Student>) {
    all_students_base_data.push(Student::create_student(1, "Jaw".to_string(), 20));
    all_students_base_data.push(Student::create_student(2, "Felipe".to_string(), 22));
    all_students_base_data.push(Student::create_student(3, "Bedfod".to_string(), 999));
}

// fn get_student_by_id(all_students_base_data: &mut Vec<Student>, student_id: usize) -> Option<&mut &mut Student> {
//     all_students_base_data
//         .iter_mut()
//         .filter(|x| x.id == student_id)
//         .collect::<Vec<&mut Student>>().first_mut()
// }
// pub fn add_nota_to_student(all_students_base_data: &mut Vec<Student>, student_id: usize, index: usize, nota: f64) {
//     let student = get_student_by_id(all_students_base_data, student_id);
//
//     if student.is_none() {
//
//     }
//     student.unwrap().notas[index] = nota;
// }
//
