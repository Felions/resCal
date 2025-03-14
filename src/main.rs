use clap::Parser;
use std::collections::HashMap;
use std::process::exit;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Sequence of colors on the resistor e.g. w-b-r = white, black, red
    sequence: Option<String>,

    /// Color of multiplier band
    #[arg(short, long, default_value = "b")]
    multiplier: String,

    /// Color of tolerance band
    #[arg(short, long)]
    tolerance: Option<String>,

    /// Print colors
    #[arg(short, long)]
    colors: bool
}

fn main() {
    let args = Cli::parse();

    if (args.colors){
        print_colors();
        exit(0);
    }

    println!("Resistance of {}", calculate(&args.sequence.unwrap(), &args.multiplier));
    if args.tolerance != None{
        println!("With tolerance of {}%", get_tolerance(&args.tolerance.unwrap()));
    }
}

fn print_colors(){
    println!("letter-   color   -   sequence    -   multiplier  -   tolerance");
    println!("b     -   black   -   +           -   +           -   -");
    println!("br    -   brown   -   +           -   +           -   +");
    println!("r     -   red     -   +           -   +           -   +");
    println!("o     -   orange  -   +           -   +           -   -");
    println!("y     -   yellow  -   +           -   +           -   -");
    println!("g     -   green   -   +           -   +           -   -");
    println!("bl    -   blue    -   +           -   +           -   -");
    println!("v     -   violet  -   +           -   -           -   -");
    println!("gr    -   gray    -   +           -   -           -   -");
    println!("w     -   white   -   +           -   -           -   -");
    println!("s     -   silver  -   -           -   +           -   +");
    println!("go    -   gold    -   -           -   +           -   +");
}

fn calculate(arguments: &str, multiplier: &str) -> f64{
    let color_values = HashMap::from([
        ("b", "0"),
        ("br", "1"),
        ("r", "2"),
        ("o", "3"),
        ("y", "4"),
        ("g", "5"),
        ("bl", "6"),
        ("v", "7"),
        ("gr", "8"),
        ("w", "9")
    ]);
    let color_multipliers = HashMap::from([
        ("s", 0.01),
        ("go", 0.1),
        ("b", 1.0),
        ("br", 10.0),
        ("r", 100.0),
        ("o", 1000.0),
        ("y", 10000.0),
        ("g", 100000.0),
        ("bl", 1000000.0),
    ]);
    let colors = arguments.split("-");
    let mut number = String::new();
    for color in colors{
        number.push_str(color_values.get(color).unwrap())
    }
    let number = number.parse::<u32>().unwrap();
    number as f64 * color_multipliers.get(&multiplier).unwrap()
}

fn get_tolerance(argument: &str) -> i32{
    let tolerances = HashMap::from([
        ("s", 10),
        ("go", 5),
        ("br", 1),
        ("r", 2)
    ]);
    return tolerances.get(&argument).unwrap().clone();
}
