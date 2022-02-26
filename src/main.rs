use anyhow::Result;
use pulldown_cmark::{html, Options, Parser};
use std::fs::write;

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next().unwrap();
    let source = args.next().unwrap_or_else(|| ".".to_owned());

    let _vault = mdzk::VaultBuilder::default().source(source).build()?;

    let notes = serde_json::to_string(&_vault).unwrap();

    write("output.json", &notes).unwrap();

    _vault.into_iter().for_each(|(key, value)| {
        let filename = key.to_string() + ".html";

        let options = Options::empty();
        let parser = Parser::new_ext(&value.content, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        write(filename, &html_output).unwrap();
    });

    Ok(())
}
