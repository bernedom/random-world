use rand::Rng;

fn main() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789! ";
    let target_string: &'static str = "Random World!";
    let mut output_string: String = "".to_string();
    let mut position_index = 0;
    let mut iterations = 0;
    let mut rng = rand::thread_rng();

    while output_string != target_string {
        let idx = rng.gen_range(0..CHARSET.len());
        let rand_string: String = (CHARSET[idx] as char).to_string();
        let current_char = target_string.chars().enumerate().nth(position_index);
        if let Some(c) = &current_char {
            assert!(
                CHARSET.contains(&(c.1 as u8)),
                "Character to be found is in available charset"
            );
            if c.1.to_string() == rand_string {
                output_string.push_str(&rand_string);
                println!(
                    "Found another char '{}'  after {} iterations!",
                    rand_string, iterations
                );

                position_index += 1
            }
        } else {
            println!("Somethign went terribly wrong -> we already filled to correct string");
            break;
        }

        iterations += 1;
    }

    println!(
        "Found desired string '{}' after {} iterations ",
        output_string, iterations
    );
}
