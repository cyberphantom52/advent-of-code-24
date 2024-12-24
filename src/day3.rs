#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum State {
    Start,
    M,
    D,
    U,
    L,
    O,
    N,
    SingleQuote,
    T,
    OpenParen,
    Comma,
    ClosingParen,
}

impl From<char> for State {
    fn from(value: char) -> Self {
        match value {
            'm' => State::M,
            'd' => State::D,
            'u' => State::U,
            'l' => State::L,
            'o' => State::O,
            'n' => State::N,
            '\'' => State::SingleQuote,
            't' => State::T,
            '(' => State::OpenParen,
            ',' => State::Comma,
            ')' => State::ClosingParen,
            _ => State::Start,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum TransitionAction {
    Push,
    Collect,
    Reset,
}

impl From<(State, char)> for Transition {
    fn from(value: (State, char)) -> Self {
        match value {
            (State::Start, 'm')
            | (State::M, 'u')
            | (State::U, 'l')
            | (State::L, '(')
            | (State::OpenParen, ',')
            | (State::Start, 'd')
            | (State::D, 'o')
            | (State::O, '(')
            | (State::O, 'n')
            | (State::N, '\'')
            | (State::SingleQuote, 't')
            | (State::T, '(') => Transition {
                next_state: State::from(value.1),
                action: TransitionAction::Push,
            },
            (State::OpenParen, c) | (State::Comma, c) if c.is_digit(10) => Transition {
                next_state: value.0,
                action: TransitionAction::Push,
            },
            (State::OpenParen, ')') | (State::Comma, ')') => Transition {
                next_state: State::from(value.1),
                action: TransitionAction::Collect,
            },
            _ => Transition {
                next_state: State::Start,
                action: TransitionAction::Reset,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Transition {
    next_state: State,
    action: TransitionAction,
}

struct Processor {
    state: State,
    command: Vec<char>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            state: State::Start,
            command: Vec::new(),
        }
    }

    pub fn consume(&mut self, token: char) -> Option<String> {
        let transition = Transition::from((self.state, token));
        match transition.action {
            TransitionAction::Collect => {
                self.command.push(token);
                let cmd = self.collect_command();
                return Some(cmd);
            }
            TransitionAction::Push => self.command.push(token),
            TransitionAction::Reset => self.reset(),
        }
        self.state = transition.next_state;

        None
    }

    fn reset(&mut self) {
        self.state = State::Start;
        self.command.clear();
    }

    fn collect_command(&mut self) -> String {
        let cmd: String = self.command.iter().collect();
        self.reset();
        cmd
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<(u32, u32, bool)> {
    let mut pairs = Vec::new();
    let mut processor = Processor::new();
    let mut active = true;

    for line in input.lines() {
        let mut tokens = line.chars();
        while let Some(token) = tokens.next() {
            if let Some(cmd) = processor.consume(token) {
                match cmd.as_str() {
                    "do()" => active = true,
                    "don't()" => active = false,
                    _ => {
                        let mut cmd = cmd
                            .strip_prefix("mul(")
                            .unwrap()
                            .strip_suffix(")")
                            .unwrap()
                            .split(',');
                        let num1 = cmd.next().unwrap().parse::<u32>().unwrap();
                        let num2 = cmd.next().unwrap().parse::<u32>().unwrap();
                        pairs.push((num1, num2, active));
                    }
                }
            }
        }
    }

    pairs
}

#[aoc(day3, part1)]
fn part1(input: &Vec<(u32, u32, bool)>) -> u32 {
    input.iter().map(|tup| tup.0 * tup.1).sum()
}

#[aoc(day3, part2)]
fn part2(input: &Vec<(u32, u32, bool)>) -> u32 {
    input
        .iter()
        .filter(|tup| tup.2)
        .map(|tup| tup.0 * tup.1)
        .sum()
}
