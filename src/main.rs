use rand::Rng;
use std::str;

extern crate crossbeam;

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

//Try this out https://stackoverflow.com/questions/33818141/how-do-i-pass-disjoint-slices-from-a-vector-to-different-threads

fn main() {
    const TARGET_STRING: &'static str = "Random World!";
    
    let mut output_string: [u8; TARGET_STRING.len()] = [0; TARGET_STRING.len()];

    let mut chunks = output_string.chunks_mut(1);
    let _ = crossbeam::scope(|scope| {
        for current_char in TARGET_STRING.chars().enumerate() {
            
            let target_char = match chunks.next() { Some(v) => v, None => panic!("Could not chunk it"),};
            scope.spawn(move |_| {
                let match_result = match_random(current_char.1);
                println!(
                    "Found char '{}' at position {} after {} iterations!",
                    match_result.1, current_char.0, match_result.0
                );
                
                target_char[0] = match_result.1 as u8;
                
            });

        }

    });

   

   
    let output_string_str = match std::str::from_utf8(&output_string[0..output_string.len()]) {
        Ok(v) => v,
        Err(e) => panic!("Invalig UTF-8 sequence: {}", e),
    };

    println!("Found desired string '{}'", output_string_str);
}
