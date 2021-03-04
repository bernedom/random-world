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
        assert!(
            CHARSET.contains(&(current_char as u8)),
            "Character to be found is in available charset"
        );

        let mut retrieve_random = || -> char {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        };

        let mut match_random = || -> (u32, char) {
            let mut rand_string = retrieve_random();
            let mut iterations = 1;
            while current_char != rand_string {
                rand_string = retrieve_random();
                iterations+=1;
            }
            (iterations, rand_string)
        };
                
        let match_result = match_random();
        
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
