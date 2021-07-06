use clap::App;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs:: File;
use yajlish::ndjson_handler::Selector;
use flaterer::{FlatFiles, flatten_from_jl, flatten};



fn main() -> Result<(), ()> {
    let matches = App::new("flatterer")
        .version("0.1")
        .author("David Raznick")
        .about("Make JSON flatterer")
        .args_from_usage(
            "<INPUT>             'Sets the input file to use'
                           <OUT_DIR>           'Sets the output directory'
                           -p --path=[path]    'key where array lives, leave if array is at root'
                           -j --jl             'Treat input as JSON Lines, path will be ignored'
                           -m --main=[main]    'Table name of top level object'
                           -f --force          'Delete output directory if it exist'",
        )
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let input_path = PathBuf::from(input);
    if !input_path.exists() {
        eprintln!("Can not find file {}", input);
        return Ok(());
    }
    let input = BufReader::new(File::open(input).unwrap());

    let output_dir = matches.value_of("OUT_DIR").unwrap();

    let mut selectors = vec![];

    if let Some(path) = matches.value_of("path") {
        selectors.push(Selector::Identifier(path.to_string()))
    }

    let main_table_name: String;

    if let Some(main) = matches.value_of("main") {
        main_table_name = main.to_string();
    } else {
        main_table_name = format!("main");
    }

    let flat_files = FlatFiles::new (
        output_dir.to_string(),
        matches.is_present("force"),
        main_table_name,
        vec![],
    ).unwrap();

    if matches.is_present("jl") {
        flatten_from_jl(input, flat_files)
    } else {
        flatten(input, flat_files, selectors)
    }


    Ok(())
}
