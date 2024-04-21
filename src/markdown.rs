use comrak::{markdown_to_html_with_plugins, plugins::syntect::SyntectAdapterBuilder, Options, Plugins};
use fronma::parser::parse;
use serde::Deserialize;
use thiserror::Error;
use anyhow::Result;


pub type HtmlOutput = String;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("failed extract header")]
    FailedExtractHeader
}

pub fn render<T>(body: &str) -> Result<(T, HtmlOutput)>
where 
    T: serde::de::DeserializeOwned,
{
    return run_with::<T>(SyntectAdapterBuilder::new().theme("base16-ocean.dark"), body)
}

fn run_with<T>(builder: SyntectAdapterBuilder, body: &str) -> Result<(T, String)>
where 
    T: serde::de::DeserializeOwned,
{
    let adapter = builder.build();
    let options = Options::default();
    let mut plugins = Plugins::default();
    let (headers, body_without_headers) = parse::<T>(body)
        .map(|d| (d.headers, d.body))
        .map_err(|_| RenderError::FailedExtractHeader)?;
    plugins.render.codefence_syntax_highlighter = Some(&adapter);


    return Ok((headers, markdown_to_html_with_plugins(body_without_headers, &options, &plugins)));
}

#[cfg(test)]
mod tests {
    use crate::markdown::render;
    use indoc::indoc;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Empty {}

    #[test]
    fn parse_should_be_return_to_heading () {
        let html = render::<Empty>("---\nfoo: 12\n---\n## Hello");

        assert_eq!(html.unwrap().1, "<h2>Hello</h2>\n")
    }

    #[test]
    fn parse_should_be_return_to_highlighted_codeblock () {
        let codeblock = indoc! {"
            ---
            foo: 12
            ---
            ```rs
            fn main() -> String {
                \"fooo\"
            }
            ```
        "};
        let html = render::<Empty>(codeblock);

        let actual = "<pre style=\"background-color:#2b303b;\"><code class=\"language-rs\"><span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() -&gt; String {\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">fooo</span><span style=\"color:#c0c5ce;\">&quot;\n</span><span style=\"color:#c0c5ce;\">}\n</span></code></pre>\n";
        assert_eq!(html.unwrap().1, actual);
    }
}
