use std::collections::HashMap;

fn main() {
    let mut students: HashMap<&str, usize> = HashMap::new();

    students.insert("Maria", 86);
    students.insert("Jose", 66);
    students.insert("Roberto", 96);
    students.insert("Laura", 67);
    students.insert("Pablo", 100);
    students.insert("Antonio", 80);

    let student = "Jose";
    match students.get(student) {
        Some(grade) => println!("{student} grade is {}", grade),
        None => println!("Student \'{student}\' not found."),
    }

    students.insert(student, 83);

    students.remove("Laura");

    println!("\nEnd of the semester grades:");
    for (student, grade) in students {
        println!("  {student} grade is {}", grade)
    }
}
