enum State {
    Normal,
    Comment,
    Upper,
    Lower,
}

fn machine_cycle(state: &State, c: char) -> (Option<char>, State) {
    match *state {
        State::Normal => {
            match c {
                '#' => (None, State::Comment),
                '^' => (None, State::Upper),
                '_' => (None, State::Lower),
                d => (Some(d), State::Normal),
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
                _ => (Some(c.to_ascii_uppercase()), State::Upper),
            }
        },
        State::Lower => {
            match c {
                '_' => (None, State::Normal),
                _ => (Some(c.to_ascii_lowercase()), State::Lower),
            }
        },
    }
}

fn main() {
    let mut state = State::Normal;
    let input = "The quick brown fox # blah # jumps ^over^ the _LaZy_ dog.";

    for c in input.chars() {
        let s = machine_cycle(&state, c);
        match s.0 {
            Some(c) => print!("{}", c),
            None => (),
        }
        state = s.1;
    }
}
