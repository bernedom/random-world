use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn main() {
    //let target_string: &'static str = "Random World!";
    let target_string: &'static str = "R";
    let mut output_string: String = "".to_string();
    //let mut position_index = 0;
    let mut iterations = 0;

    while output_string != target_string {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(1)
            .map(char::from)
            .collect();

        output_string = rand_string;
        iterations += 1;
    }

    println!(
        "Found desired string '{}' after {} iterations ",
        output_string, iterations
    );
}
