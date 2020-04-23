enum State {
    Normal,
    Comment,
    Upper,
    Lower,
}

fn machine_cycle(state: &State, c: char) -> (Option<char>, State) {
    match *state {
        State::Normal => (),
        State::Comment => (),
        State::Upper => (),
        State::Lower => (),
    };

    (Some('a'), State::Normal)
}

fn main() {

}
