use weather::arguments;
use weather::processor;

use clap::Parser;

fn main() {
    let cli = arguments::Cli::parse();

    let dependency_factory = Box::new(processor::ProductionDependencyFactory);
    let processor = processor::Processor::new(dependency_factory);

    if let Err(e) = processor.run(cli) {
        println!("{e}");
    }
}
