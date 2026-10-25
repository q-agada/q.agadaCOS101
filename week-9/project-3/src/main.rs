use std::fs::File;
use std::io::Write;

fn main() {
    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = File::create("efcc.csv").expect("File creation failed");

    file.write_all(b"Commissioner,Ministry,Zone\n")
        .expect("Write failed");

    for i in 0..5 {
        println!("{} {} {}", commissioners[i], ministries[i], zones[i]);

        file.write_all(commissioners[i].as_bytes())
            .expect("Write failed");
        file.write_all(b",").expect("Write failed");
        file.write_all(ministries[i].as_bytes())
            .expect("Write failed");
        file.write_all(b",").expect("Write failed");
        file.write_all(zones[i].as_bytes())
            .expect("Write failed");
        file.write_all(b"\n").expect("Write failed");
    }
}