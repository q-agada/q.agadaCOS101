fn main() {
    let category = vec![
        "Public Servant", "Public Servant",
        "Public Servant", "Public Servant",
        "Public Servant", "Public Servant",
        "Office Admin", "Office Admin",
        "Office Admin", "Office Admin",
        "Office Admin", "Office Admin",
        "Academic", "Academic", "Academic",
        "Academic", "Academic", "Academic",
        "Lawyer", "Lawyer", "Lawyer",
        "Lawyer", "Lawyer", "Lawyer",
        "Teacher", "Teacher", "Teacher",
        "Teacher", "Teacher", "Teacher",
    ];

    let position = vec![
        "Intern", "Administrator", "Senior Administrator",
        "Office Manager", "Director", "CEO",
        "Intern", "Administrator", "Senior Administrator",
        "Office Manager", "Director", "CEO",
        "Research Assistant", "PhD Candidate",
        "Post-Doc Researcher", "Senior Lecturer",
        "Dean", "CEO", "Paralegal", "Junior Associate",
        "Associate", "Senior Associate 1-2",
        "Senior Associate 3-4", "Partner",
        "Placement", "Classroom Teacher", "Senior Teacher",
        "Leading Teacher", "Deputy Principal", "Principal",
    ];

    let level = vec![
        "APS 1-2", "APS 3-5", "APS 5-8",
        "EL1 8-10", "EL2 10-13", "SES",
        "APS 1-2", "APS 3-5", "APS 5-8",
        "EL1 8-10", "EL2 10-13", "SES",
        "APS 3-5", "APS 5-8", "EL1 8-10",
        "EL2 10-13", "SES", "SES",
        "APS 1-2", "APS 3-5", "APS 5-8",
        "EL1 8-10", "EL2 10-13", "SES",
        "APS 1-2", "APS 3-5", "APS 5-8",
        "EL1 8-10", "EL2 10-13", "SES",
    ];

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter staff category:");
    std::io::stdin().read_line(&mut input1)
        .expect("Failed to read input");

    println!("Enter staff position:");
    std::io::stdin().read_line(&mut input2)
        .expect("Failed to read input");

    let mut found = false;

    for i in 0..position.len() {
        if category[i].to_lowercase() == input1.trim()
            && position[i].to_lowercase() == input2.trim()
        {
            println!("Staff Level: {}", level[i]);
            found = true;
        }
    }

    if !found {
        println!("Staff category or position not found.");
    }
}