#[derive(Debug, Clone)]
pub enum State {
    E,
    Ee,
    T,
    Tt,
    F,
    Terminal(char),
}
