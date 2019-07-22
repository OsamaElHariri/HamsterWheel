use hamster_wheel::interpreter::interpreter::Interpreter;

fn main() {
    let output = Interpreter::new(
        "{{ output /some/file/path/../ok.txt }}
        {{import}}
        name: something;
        key: 1-hbZd6LH3153gVZi6CGq6n1HWv36Omt2k_OMc_6w3CE;
        {{end}}
        {{Loop(i) something as row}}
            Cell: ({{ row[0] }}, {{ row[1] }});
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
