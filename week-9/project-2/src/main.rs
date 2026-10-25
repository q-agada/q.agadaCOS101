use std::fs::File;
use std::io::Write;

fn main() {
    let names = [
        "Oluchi Mordi",
        "Adams Aliyu",
        "Shania Bolade",
        "Adekunle Gold",
        "Blanca Edemoh",
    ];

    let matric = [
        "ACC10211111",
        "ECO10110101",
        "CSC10328828",
        "EEE11020202",
        "MEE10202001",
    ];

    let departments = [
        "Accounting",
        "Economics",
        "Computer",
        "Electrical",
        "Mechanical",
    ];

    let levels = [300, 100, 200, 200, 100];

    let mut file = File::create("students.csv").expect("File creation failed");

    file.write_all(b"Student Name,Matric Number,Department,Level\n")
        .expect("Write failed");

    for i in 0..5 {
        println!("{} {} {} {}", names[i], matric[i], departments[i], levels[i]);

        file.write_all(names[i].as_bytes()).expect("Write failed");
        file.write_all(b",").expect("Write failed");
        file.write_all(matric[i].as_bytes()).expect("Write failed");
        file.write_all(b",").expect("Write failed");
        file.write_all(departments[i].as_bytes()).expect("Write failed");
        file.write_all(b",").expect("Write failed");

        let level = levels[i].to_string();
        file.write_all(level.as_bytes()).expect("Write failed");
        file.write_all(b"\n").expect("Write failed");
    }
}