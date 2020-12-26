fn main() {
    let country_code = 44;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid",
    };

    println!("the country with code {} is {}", country_code, country);

    let x = false;

    let s = match x {
        true => "Yes",
        false => "No", // compilation fails if not all possibilities have a match, try commenting this line
    };
}
