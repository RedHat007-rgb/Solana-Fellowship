use std::io::stdin;

fn main() {
    let student: String = String::new();

    println!("\n========== 📝 STUDENT GRADE TRACKER ==========\n");

    println!("👩‍🎓 Enter Student Name:");
    let mut name = String::new();
    stdin().read_line(&mut name).expect("please enter the name");

    println!("\n📚 Enter the number of subjects:");
    let mut subjects = String::new();
    stdin().read_line(&mut subjects).expect("please enter the number of subjects.");

    let subject_count: usize = subjects.trim().parse().expect("please enter a valid number");

    let mut subject_name: Vec<String> = Vec::new();
    let mut marks: Vec<usize> = Vec::new();

    for i in 0..subject_count {
        println!("\n➡️ Subject {} Name:", i + 1);
        let mut subject = String::new();
        stdin().read_line(&mut subject).expect("please neter a valid SUbject");

        println!("✏️  Marks for {}:", subject.trim());
        let mut mark_input = String::new();
        stdin().read_line(&mut mark_input).expect("please enter a valid number");

        let mark: usize = mark_input.trim().parse().expect("enter a number");
        subject_name.push(subject.trim().to_string());
        marks.push(mark);
    }

    println!("\n✅ Student Name: {}", name.trim());

    println!("\n📋 Subjects and Marks:");
    for i in 0..subject_count {
        println!("   {}. {:<10} ➜ {:>3}", i + 1, subject_name[i], marks[i]);
    }

    let avg = average(&marks);
    println!("\n📊 Average Marks: {}", avg);
}

fn average(marks: &Vec<usize>) -> usize {
    let mut average: usize = 0;
    let mut sum: usize = 0;
    let number_of_subjects = marks.len();

    for i in 0..number_of_subjects {
        sum += marks[i];
        if i + 1 == number_of_subjects {
            average = sum / number_of_subjects;
        }
    }

    return average;
}
