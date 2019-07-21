use hamster_wheel::interpreter::interpreter::Interpreter;
use hamster_wheel::parser::parser::Parser;

fn main() {
    // Parser::new("hello there {{ ok }} something {{ ok }}").parse();
    // let output = Interpreter::new("hello there {{ ok }} something {{ ok }}").interpret();
    // let output = Interpreter::new(
    //     "{{Loop(i) rows[0, ..] as row}}
    //         Map<String, String> {{row [ 0 ] }} = {
    //             {{loop(j) row[1, ..] as cell}}
    //                 {{cell.col[0]}} : {{cell}}
    //                 {{rows[0][j]}} : {{cell}}
    //             {{END}}
    //         }
    //         {{END}}",
    // )
    // .interpret();
    let output = Interpreter::new(
        "{{Loop(i) rows[0, ..] as someVar}}
            Loop number {{ i }}, value is: {{ someVar }}
            {{Loop(someVar) rows as other}}
                shadowing parent variable {{ someVar }}, value is: {{ other }}
            {{END}}
        {{END}}",
    )
    .interpret();
    println!("{}", output);
}
