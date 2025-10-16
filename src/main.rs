mod lexer;

fn main() {
    lexer::foo(r"\x y. y x").print_all();
    lexer::foo(r"(x y). y x").print_all();
}
