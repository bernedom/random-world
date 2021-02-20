use rand::Rng;

fn main() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789! ";
    let target_string: &'static str = "Random World!";
    let mut output_string = String::with_capacity(target_string.len());
    let mut position_index = 0;
    let mut iterations = 0;
    let mut rng = rand::thread_rng();

    while output_string != target_string {
        let idx = rng.gen_range(0..CHARSET.len());
        let rand_string = CHARSET[idx] as char;
        let current_char = target_string.chars().enumerate().nth(position_index);

        match current_char {
            None => {
                assert!(
                    current_char.is_some(),
                    "Could not retrieve {}th character from target string",
                    position_index
                );
            }
            Some(c) => {
                assert!(
                    CHARSET.contains(&(c.1 as u8)),
                    "Character to be found is in available charset"
                );
                if c.1 == rand_string {
                    output_string.insert(output_string.len(), rand_string);
                    println!(
                        "Found another char '{}'  after {} iterations!",
                        rand_string, iterations
                    );
                    position_index += 1
                }
            }
        }

        iterations += 1;
    }

    println!(
        "Found desired string '{}' after {} iterations ",
        output_string, iterations
    );
}
