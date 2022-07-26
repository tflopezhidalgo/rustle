use std::fs;
use std::collections::HashSet;
use std::io::{stdin, stdout, Write};

use rand::Rng;


type Letter = char;
type Word = String;

#[derive(Clone, Debug)]
struct GameState {
    user_requested_finish: bool,
    player_won: bool,
    guesses: Vec<Word>,
    misses: HashSet<Letter>,
    hits: HashSet<Letter>,
    target: String,
    errors: HashSet<UserInputError>,
}

static GAME_TITLE: &str = r"
 --- RUSTLE 🦀
";

static ALPHABET: &str = r"
 > A B C D E F
 G H I J L M N
 O P Q R S T U
 V W X Y Z < < ";

enum TextColor {
    Red,
    Green,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum UserInputError {
    InvalidLength
}

fn should_finish(game: &GameState) -> bool {
    game.player_won || game.guesses.len() >= 5
}

fn process_player_guess(guess: String, state: &GameState) -> GameState {
    let mut new_state = state.clone();

    // If player guess is empty just skip it.
    if guess.len() == 0 {
        return new_state;
    }

    // If it does not have the right lenght, skip it.
    if guess.len() > 5 {
        new_state.errors.insert(UserInputError::InvalidLength);
        return new_state;
    }

    if guess == new_state.target {
        new_state.player_won = true;
        return new_state;
    }

    new_state.guesses.push(guess.clone());

    guess.chars().for_each(|l| {
        match new_state.target.contains(l) {
            true => new_state.hits.insert(l),
            false => new_state.misses.insert(l),
        };
    });

    new_state
}

fn color_char(c: char, color: TextColor) -> String {
    match color {
        TextColor::Red => format!("\x1b[41m{c}\x1b[0m"),
        TextColor::Green => format!("\x1b[42m{c}\x1b[0m"),
    }
}

fn apply_color(
    target_str: &str,
    missed_letters: &HashSet<Letter>,
    hit_letters: &HashSet<Letter>,
) -> String {
    target_str
        .chars()
        .map(|l| {
            if missed_letters.contains(&l.to_ascii_lowercase()) {
                return color_char(l, TextColor::Red);
            }
            if hit_letters.contains(&l.to_ascii_lowercase()) {
                return color_char(l, TextColor::Green);
            }

            return String::from(l);
        })
        .collect::<String>()
}

fn ask_for_word() -> std::io::Result<String> {
    let mut word = String::new();
    stdin().read_line(&mut word)?;

    // Sanitize the input.

    // Pop the final \n.
    word.pop();

    Ok(String::from(word.trim()))
}

fn clear_screen() -> () {
    print!("{}[2J{esc}[1;1H", esc = 27 as char);
}

fn format_missed_word(w: &String) -> String {
    format!("+ {}  ❌", w)
}

fn show_ui(game: &GameState) -> () {
    clear_screen();

    // TODO. improve

    println!("{}", GAME_TITLE);
    println!("{}", apply_color(ALPHABET, &game.misses, &game.hits));
    println!();

    // Show missed words.
    game.guesses.iter().for_each(|g| {
        println!("{}", format_missed_word(g));
    });

    game.errors.iter().for_each(|e| {
        match e {
            UserInputError::InvalidLength => println!("Invalid length!"),
        }
    });

    println!();
    print!("> ");

    stdout().flush().unwrap();
}

fn main_loop(game: GameState) -> std::io::Result<()> {
    let mut g = game.clone();

    while !should_finish(&g) {
        show_ui(&g);

        let guess = ask_for_word()?;

        g = process_player_guess(guess, &g);
    }

    if g.player_won {
        println!("You won! 🤟");
    } else {
        println!("You lost! X");
    }

    Ok(())
}

fn pick_word() -> String {
    let content = fs::read_to_string("src/words.txt").unwrap();

    let line = rand::thread_rng().gen_range(0..content.lines().count());

    for (i, l) in content.lines().enumerate() {
        if i == line {
            return String::from(l);
        }
    }

    return String::from("invalid");
}

fn main() -> std::io::Result<()> {

    let target: String = pick_word();

    main_loop(GameState {
        target,
        guesses: Vec::new(),
        misses: HashSet::new(),
        hits: HashSet::new(),
        errors: HashSet::new(),
        user_requested_finish: false,
        player_won: false,
    })?;

    Ok(())
}
