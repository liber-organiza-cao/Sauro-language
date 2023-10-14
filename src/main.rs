use logos::Logos;
mod lexer;

fn main() {
	let mut lex = lexer::Token::lexer(
		r#"
        struct Human{
            name: String,
            age: i32
        }
        fn main(){
            let ferros = Human{
                name: "ferroscraft",
                age: 19
            };
            let my_string = "pao de batata";
            let my_int = -10;
            let my_float = 10.4;
            loop {
                print(my_string);
            }
        }"#,
	);
	while let Some(t) = lex.next() {
		println!("{t:?}");
	}
}
