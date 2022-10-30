fn main() -> Result<(), Box<dyn std::error::Error>> {
    lex_float::main();
    Ok(())
}

mod lex_float;