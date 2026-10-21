use std::io::Write;

fn main(){

    let mut file = std::fs::File::create("breweries.csv").expect("create failed");

    file.write_all("Lager,Stout,Non-alcoholic\n".as_bytes())
    .expect("Write failed");

    file.write_all("33 Export,Legend,Maltina\n".as_bytes())
    .expect("write failed");

    file.write_all("Desperados,Turbo king ,Amstel Maltina\n".as_bytes())
    .expect("Write failed");

    file.write_all("Goldberg,Williams,Malta Gold\n".as_bytes())
    .expect("Write failed");

    file.write_all("Gulder,,Fayrouz\n".as_bytes())
    .expect("Write failed");

    file.write_all("Heineken,,\n".as_bytes())
    .expect("Write failed");

    file.write_all("Star\n".as_bytes())
    .expect("Write failed");

    println!("NIGERIAN BREWERIES CATEGORIES SUCCESSFULLY CREATED!!!!!!!")
}