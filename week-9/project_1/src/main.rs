use std::io::Write;
fn main() {

    let high_quality_lager = "HIGH QUALITY LAGER - 33 Exports, Desperados, Goldberg, Gulger, Heineken, Star";
    let high_quality_stout = "HIGH QUALITY STOUT - Legend, Turbo King, Williams";
    let high_quality_nonalch = "HIGH QUALITY NON_ALCHOHOLIC DRINKS - Maltina, Amstel Malta, Fayrouz, Malta Gold";
    let mut file = std::fs::File::create("HIGH QUALITY DRINKS.txt").expect("Failed to create");
    file.write_all("The following shows the high quality drinks\n".as_bytes()).expect("Failed");
    file.write_all(high_quality_lager.as_bytes()).expect("Failed");
    file.write_all("\n".as_bytes()).expect("Failed");
    file.write_all(high_quality_stout.as_bytes()).expect("Failed");
file.write_all("\n".as_bytes()).expect("Failed");
    file.write_all(high_quality_nonalch.as_bytes()).expect("Failed");
    file.write_all("\n".as_bytes()).expect("Failed");
    println!("DATA WRITTEN TO FILE");

    
}
