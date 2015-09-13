#[macro_use]
extern crate clap;

fn main() {
    // This example shows how to create an application with several arguments using macro builder.
    // It combines the simplicity of the from_usage methods and the performance of the Builder Pattern.
    //
    // The example below is functionally identical to the one in 01a_quick_example.rs and 01b_quick_example.rs
    //
    // Create an application with 5 possible arguments (2 auto generated) and 2 subcommands (1 auto generated)
    //    - A config file
    //        + Uses "-c filename" or "--config filename"
    //    - An output file
    //        + A positional argument (i.e. "$ myapp output_filename")
    //    - A debug flag
    //        + Uses "-d" or "--debug"
    //        + Allows multiple occurrences of such as "-dd" (for vary levels of debugging, as an example)
    //    - A help flag (automatically generated by clap)
    //        + Uses "-h" or "--help" (Only autogenerated if you do NOT specify your own "-h" or "--help")
    //    - A version flag (automatically generated by clap)
    //        + Uses "-V" or "--version" (Only autogenerated if you do NOT specify your own "-V" or "--version")
    //    - A subcommand "test" (subcommands behave like their own apps, with their own arguments
    //        + Used by "$ myapp test" with the following arguments
    //            > A list flag
    //                = Uses "-l" (usage is "$ myapp test -l"
    //            > A help flag (automatically generated by clap
    //                = Uses "-h" or "--help" (full usage "$ myapp test -h" or "$ myapp test --help")
    //            > A version flag (automatically generated by clap
    //                = Uses "-V" or "--version" (full usage "$ myapp test -V" or "$ myapp test --version")
    //    - A subcommand "help" (automatically generated by clap because we specified a subcommand of our own)
    //        + Used by "$ myapp help" (same functionality as "-h" or "--help")
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Kevin K. <kbknapp@gmail.com>")
        (about: "Does awesome things")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg INPUT: +required "Sets the input file to use")
        (@arg debug: -d ... "Sets the level of debugging information")
        (@subcommand test =>
            (about: "controls testing features")
            (version: "1.3")
            (author: "Someone E. <someone_else@other.com>")
            (@arg verbose: -v --verbose "Print test information verbosely")
        )
    ).get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("CONFIG").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Vary the output based on how many times the user used the "debug" flag
    // (i.e. 'myapp -d -d -d' or 'myapp -ddd' vs 'myapp -d'
    match matches.occurrences_of("debug") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("verbose") {
            println!("Printing verbosely...");
        } else {
            println!("Printing normally...");
        }
    }

    // more porgram logic goes here...
}