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
