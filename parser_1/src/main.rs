fn main() {
    let mut raw = String::from("");
    let stdin = std::io::stdin();
    stdin.read_line(&mut raw).unwrap();
    let mut inp = raw.chars().rev().collect::<String>();

    let mut stack = vec![table::State::E];

    while !stack.is_empty() && !inp.is_empty() {
        let i = inp.chars().last().unwrap();
        let this = stack.last().unwrap();

        let res = trans(this, i);
        dbg!(&stack);
        if let Ok(mut ok) = res {
            dbg!(&ok);
            ok.reverse();
            for i in ok {
                stack.push(i);
            }
        } else {
            println!("Error");
            return;
        }
        while let table::State::Terminal(t) = stack.last().unwrap() {
            if *t == i {
                inp.pop();
                stack.pop();
                continue;
            } else {
                println!("error {} {}", t, i);
            }
        }
    }
}
mod trans;
use trans::*;
mod table;
