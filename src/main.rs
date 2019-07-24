use hamster_wheel::file_walker::file_walker::FileWalker;

fn main() {
    //     let output = Interpreter::new(
    //         "{{ output localization.dart }}
    // {{import}}
    // name: localizations;
    // key: 1JqduyjCwmjKmmyBT0eM7YuD_El2Z_5grafi2hLV8OX4;
    // {{end}}

    // class Localized {
    //     static String languageCode = \"en\";
    // {{Loop localizations[1, ..] as localizationRow}}
    // static String get {{localizationRow[0]}} {
    //   {{Loop(i, collectionIndex)  localizations[0][1, ..] as languageCode }}
    //     if (languageCode == \"{{languageCode}}\") return \"{{localizationRow[collectionIndex]}}\";
    //   {{End}}
    //     return \"{{localizationRow[3]}}\";
    //   }
    // {{END}}
    // }
    // ",
    //     )
    //     .interpret();
    // println!("{}\n output to: {}", output.text, output.output_file);
    FileWalker::walk_directory(String::from("."));
    println!("Hamster Wheel Done!");
}
