/*
    3. Write letter grade function, takes grade as parameter and returns a letter grade
        A: 90-100
        B: 80-89
        C: 70-79
        D: 60-69
        F: Below 60
*/
fn get_letter_grade(grade: i32) -> &'static str {
    if 90 <= grade && grade <= 100 {
        return "A";
    } else if 80 <= grade && grade <= 89 {
        return "B";
    } else if 70 <= grade && grade <= 79 {
        return "C";
    } else if 60 <= grade && grade <= 69 {
        return "D";
    } else {
        return "F";
    }
}

fn main() {
    /* 1. Create an array of student scores */
    let students = [75, 90, 78, 92, 88];
    let mut grades = Vec::new();

    /* 2. Use the for loop to iterate through the array and call get_letter_grade_function */
    for student in students {
        grades.push(get_letter_grade(student));
    }

    /* 4. Print each student's scores along with the corresponding letter grade */
    print!("Students grades: ");
    for grade in grades {
        print!("{} ", grade);
    }
    println!(" ");
}
