use rand::Rng;

// consider returning an optional for the fun of having it and removing the assert
fn match_random(current_char: char) -> (i32, char) {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789! ";
    assert!(
        CHARSET.contains(&(current_char as u8)),
        "Character to be found is in available charset"
    );
    let mut retrieve_random = || -> char {
        let idx = rng.gen_range(0..CHARSET.len());
        CHARSET[idx] as char
    };

    let mut rand_string = retrieve_random();
    let mut iterations = 1;
    while current_char != rand_string {
        rand_string = retrieve_random();
        iterations += 1;
    }
    (iterations, rand_string)
}

fn main() {
    let target_string: &'static str = "Random World!";
    let mut output_string = String::with_capacity(target_string.len());
    let mut iterations = 0;

    for current_char in target_string.chars() {
        let match_result = match_random(current_char);
        output_string.insert(output_string.len(), match_result.1);
        iterations += match_result.0;
        println!(
            "Found another char '{}'  after {} iterations!",
            match_result.1, iterations
        );
    }

    println!(
        "Found desired string '{}' after {} iterations ",
        output_string, iterations
    );
}
