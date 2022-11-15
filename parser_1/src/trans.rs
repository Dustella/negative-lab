use crate::table::State;

pub fn trans(state: &State, cha: char) -> Result<Vec<State>, String> {
    match state {
        State::E => match cha {
            '(' => Ok(vec![State::T, State::Ee]),
            'n' => Ok(vec![State::T, State::Ee]),
            _ => Err("".to_string()),
        },
        State::T => match cha {
            '(' => Ok(vec![State::F, State::Tt]),
            'n' => Ok(vec![State::F, State::Tt]),
            _ => Err("".to_string()),
        },
        State::F => match cha {
            '(' => Ok(vec![State::Terminal('('), State::E, State::Terminal(')')]),
            'n' => Ok(vec![State::Terminal('n')]),
            _ => Err("".to_string()),
        },
        State::Ee => match cha {
            '+' => Ok(vec![State::Terminal('+'), State::T, State::Ee]),
            _ => Err("".to_string()),
        },
        State::Tt => match cha {
            '*' => Ok(vec![State::Terminal('*'), State::F, State::Tt]),
            _ => Err("".to_string()),
        },
        State::Terminal(_) => Err("expect terminal".to_string()),
    }
}
