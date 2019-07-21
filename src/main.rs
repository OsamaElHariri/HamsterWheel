use hamster_wheel::interpreter::interpreter::Interpreter;

fn main() {
    let output = Interpreter::new(
        "{{config}}
        name: something;
        {{end}}
        {{Loop(i) rows[1, ..] as row}}
            {{i}}
        {{END}}",
    )
    .interpret();

    // let output = Interpreter::new(
    //     "{{Loop(i) rows[1, ..] as row}}
    //         Map<String, String>  {{ row[0] }} = {
    //             {{loop(j) row[1, ..] as cell}}
    //                 \"{{row[0]}}\": {{cell}},
    //             {{END}}
    //         }
    //         {{END}}
    //         // another loop
    //         {{Loop(i) rows[1, ..] as row}}
    //         Map<String, String>  {{ row[0] }} = {
    //             {{loop(j) row[1, ..] as cell}}
    //             \"{{row[0]}}\": {{cell}},
    //             {{END}}
    //         }
    //         {{END}}
    //         ",
    // )
    // .interpret();

    // let output = Interpreter::new(
    //     "{{Loop(i) rows[0, ..] as someVar}}
    //         Loop number {{ i }}, value is: {{ someVar }}
    //         {{Loop(someVar) rows as other}}
    //             shadowing parent variable {{ someVar }}, value is: {{ other }}
    //         {{END}}
    //     {{END}}",
    // )
    // .interpret();
    println!("{}", output);
}
