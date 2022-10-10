use serde_derive::Deserialize;

const USAGE: &str = "
Usage: treestats analyze <directory>
       treestats -h | --help
       treestats --version

Commands:
    analyze                  Analyze a directory tree and output statistics.

Arguments:
    directory                Root directory of the directory tree to analyze.

Options:
    -h, --help               Show this screen.
    --version                Show version.
";

#[derive(Deserialize)]
struct Args {
    cmd_analyze: bool,
    arg_directory: String,
    flag_version: bool,
}

fn main() {
    let args: Args =
        docopt::Docopt::new(USAGE)
            .and_then(|docopts|
                docopts.argv(std::env::args().into_iter())
                   .deserialize()
            )
            .unwrap_or_else(|error|
                error.exit()
            );

    if args.flag_version {
        println!("treestats v{}", treestats::version());
    } else {
        if args.cmd_analyze {
            treestats::analyze(&args.arg_directory);
        }
    }
}
