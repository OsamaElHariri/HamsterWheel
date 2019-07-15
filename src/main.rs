use hamster_wheel::tokenizer::tokenizer;
use hamster_wheel::parser::parser::Parser;

fn main() {
    Parser::new("[ .. , i ]").parse();
    // let mut tokenizer = tokenizer::tokenize(
    //     "{{Loop(i) rows[0, ..] as row}}
    //         Map<String, String> {{row [ 0 ] }} = {
    //             {{loop(j) row[1, ..] as cell}}
    //                 {{cell.col[0]}} : {{cell}}
    //             {{END LOOP}}
    //         }
    //         {{END LOOP}}",
    // );
    
    // while tokenizer.token != tokenizer::Token::EOF {
    //     println!(
    //         "{:?}, {}, range: {:?}",
    //         tokenizer.token,
    //         tokenizer.slice(),
    //         tokenizer.range()
    //     );
    //     tokenizer.advance();
    // }
}
