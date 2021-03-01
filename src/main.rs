use rand::Rng;

fn main() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789! ";
    let target_string: &'static str = "Random World!";
    let mut output_string = String::with_capacity(target_string.len());
    let mut iterations = 0;
    let mut rng = rand::thread_rng();

    for current_char in target_string.chars() {
        let mut retrieve_random = || -> char {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        };
        let mut rand_string = retrieve_random();

        assert!(
            CHARSET.contains(&(current_char as u8)),
            "Character to be found is in available charset"
        );
        while current_char != rand_string {
            iterations += 1;
            rand_string = retrieve_random();
        }
        output_string.insert(output_string.len(), rand_string);
        println!(
            "Found another char '{}'  after {} iterations!",
            rand_string, iterations
        );
    }

    println!(
        "Found desired string '{}' after {} iterations ",
        output_string, iterations
    );
}
