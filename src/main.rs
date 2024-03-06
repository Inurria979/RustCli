use clap::Parser;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli{
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {

    let args = Cli::parse();

    let resutl = std::fs::read_to_string(&args.path);

    match resutl {
        Ok(content)=> {content}, 
        Err(error) => {panic!("Cannot control {:#?}", error);} 
    };
/* *
    for line in content.lines(){

        if line.contains(&args.pattern){
            println!("Pattern find: {}", line);
        }

    }
*/

    println!("Pattern: {:?}, Path {:?}", args.pattern, args.path);

}
