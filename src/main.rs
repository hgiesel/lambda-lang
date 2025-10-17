mod lexer;
mod parser;

fn main() {
    let x = lexer::foo(r"foobar(). y x");
    let p = parser::Parser::new(x.collect()).parse_func();

    if let Some(f) = p {
        println!("{:?}", f);
        return;
    } else {
        println!("No parse");
    }
}
