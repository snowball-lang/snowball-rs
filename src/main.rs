use clap::Parser as ClapParser;

mod ast;
mod compiler;
mod frontend;
mod reports;
mod utils;

#[derive(ClapParser)]
#[clap(name = "Snowball Compiler", version = "0.1.0", author = "Snowball")]
struct Args {
    // we can have "run" or "build" as the subcommand
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(ClapParser)]
enum SubCommand {
    Run(Run),
    Build(Build),
}

#[derive(ClapParser)]
struct Run {
    #[clap(short, long, default_value = "./")]
    path: String,
}

#[derive(ClapParser)]
struct Build {
    #[clap(short, long, default_value = "./")]
    path: String,
}

fn main() {
    let args = Args::parse();

    match args.subcmd {
        SubCommand::Run(run) => run_command(run),
        SubCommand::Build(build) => build_command(build),
    }
}

fn run_command(run: Run) {
    let compiler = compiler::Compiler::new(run.path);
    compiler.run();
    // TODO: Run the code
}

fn build_command(build: Build) {
    let compiler = compiler::Compiler::new(build.path);
    compiler.run();
}

