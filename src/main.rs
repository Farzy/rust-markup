#[derive(Copy, Clone)]
enum State {
    Normal,
    Comment,
    Upper,
    Lower,
}

fn machine_cycle(state: State, c: char) -> (Option<String>, State) {
    match state {
        State::Normal => {
            match c {
                '#' => (None, State::Comment),
                '^' => (None, State::Upper),
                '_' => (None, State::Lower),
                _ => (Some(c.to_string()), State::Normal),
            }
        },
        State::Comment => {
            match c {
                '#' => (None, State::Normal),
                _ => (None, State::Comment),
            }
        },
        State::Upper => {
            match c {
                '^' => (None, State::Normal),
                _ => (Some(c.to_uppercase().to_string()), State::Upper),
            }
        },
        State::Lower => {
            match c {
                '_' => (None, State::Normal),
                _ => (Some(c.to_lowercase().to_string()), State::Lower),
            }
        },
    }
}

fn main() {
    let mut state = State::Normal;
    let input = "The quick brown fox # blah # jumps ^over^ the _LaZy_ dog. ^ça et là^. ^Heiß^. _RÊŸ._";

    for c in input.chars() {
        let (output, new_state) = machine_cycle(state, c);
        if let Some(chr) = output {
            print!("{}", chr);
        }
        state = new_state;
    }
    println!();
}
