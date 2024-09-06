use clap::Parser;
use html_docx::convert::{process, read_html_file};

#[derive(Parser)]
#[command(
    version,
    about = "HTML to docx converter",
    long_about = "An opinionated converter from HTMLBook-like HTML file(s) to docx"
)]
struct Cli {
    /// HTML file to convert to docx
    files: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    for file in cli.files {
        let html = read_html_file(file);
        process(html);

    }
}
