fn main() {
    let target_string: &'static str = "Random World!";
    let mut output_string: String = "".to_string();
    //let mut position_index = 0;

    while output_string != target_string {
        output_string = "Random World!".to_string();
    }

    println!("Found desired string '{}' ", output_string);
}
