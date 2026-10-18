fn main() {
    let candidates = vec![
        ("Grace", 3),
        ("David", 5),
        ("Mary", 2),
        ("Samuel", 7),
        ("Esther", 4),
    ];

    let mut highest = candidates[0];

    for i in 0..candidates.len() {
        if candidates[i].1 > highest.1 {
            highest = candidates[i];
        }
    }

    println!("Candidate with highest experience:");
    println!("Name: {}", highest.0);
    println!("Years of experience: {}", highest.1);
}