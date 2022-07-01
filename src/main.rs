use std::collections::HashSet;
use std::io::{stdin, stdout, Write};

// Our best on picking a random word
static WORD: &str = "chair";

type Letter = char;
type Word = String;

#[derive(Clone, Debug)]
struct GameState {
    user_requested_finish: bool,
    player_won: bool,
    guesses: Vec<Word>,
    misses: HashSet<Letter>,
    hits: HashSet<Letter>,
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

fn should_finish(game: &GameState) -> bool {
    game.player_won
}

fn process_player_guess(guess: String, state: &GameState) -> GameState {
    let mut new_state = state.clone();

    if guess == String::from(WORD) {
        new_state.player_won = true;
        return new_state;
    }

    new_state.guesses.push(guess.clone());

    guess.chars().for_each(|l| {
        match String::from(WORD).contains(l) {
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

    // Pop the final \n.
    word.pop();

    Ok(word)
}

fn clear_screen() -> () {
    print!("{}[2J{esc}[1;1H", esc = 27 as char);
}

fn show_ui(game: &GameState) -> () {
    clear_screen();

    // TODO. improve

    println!("{}", GAME_TITLE);
    println!("{}", apply_color(ALPHABET, &game.misses, &game.hits));
    println!();

    // Show missed words.
    game.guesses.iter().for_each(|g| {
        println!("+ {}  ❌", g);
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

        if g.player_won {
            println!("You won! 🤟");
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    main_loop(GameState {
        guesses: Vec::new(),
        misses: HashSet::new(),
        hits: HashSet::new(),
        user_requested_finish: false,
        player_won: false,
    })?;

    Ok(())
}
