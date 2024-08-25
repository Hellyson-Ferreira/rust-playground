pub struct Student {
    pub id: usize,
    pub name: String,
    pub age: usize,
    pub notas: [f64; 4],
    pub average: f64,
}

impl Student {
    pub fn create_student(id: usize, name: String, age: usize) -> Student {
        let mut student = Student {
            id,
            name,
            age,
            notas: [0.0; 4],
            average: 0.0,
        };

        student.calculate_student_average();

        student
    }

    #[allow(dead_code)]
    pub fn add_nota(&mut self, nota: f64, bimestre: usize) {
        self.notas[bimestre] = nota;
    }
    #[allow(dead_code)]
    pub fn calculate_student_average(&mut self) -> &mut Self {
        let mut average: f64 = 0.0;
        for nota in self.notas {
            average += nota
        }

        self.average = average / 4.0;

        self
    }

    pub fn get_string_notas(&self) -> String {
        let mut notas = String::new();
        for (i, nota) in self.notas.iter().enumerate() {
            if i > 0 {
                notas.push_str(", ");
            }
            notas.push_str(&nota.to_string());
        }
        notas
    }
}