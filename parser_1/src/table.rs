#[derive(Debug, Clone, Copy)]
pub enum State {
    E,
    Ee,
    T,
    Tt,
    F,
    Terminal(char),
    Eps,
}

impl State {
    pub fn is_terminal(self) -> bool {
        let mut res = false;
        if let State::Terminal(_) = self {
            res = true
        }
        if let State::Eps = self {
            res = true
        }
        res
    }
}
