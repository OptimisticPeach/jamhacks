pub fn test() {
    let num = 24;
    println!("Guess a number between 1 and 100:");
    loop {
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let int_guess = guess.parse::<u32>();

        if let Ok(guess_inside) = int_guess {
            if guess_inside == num {
                println!("You got it!");
                break;
            } else if guess > num {
                println!("Too High! Guess Again:");
            } else {
                println!("Too Low! Try Again:");
            }
        } else {
            println!("You need to put in a number!");
        }
    }
}
