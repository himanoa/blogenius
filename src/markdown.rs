use comrak::{markdown_to_html_with_plugins, plugins::syntect::SyntectAdapterBuilder, Options, Plugins};


pub type HtmlOutput = String;

pub fn parse<'a>(body: &str) -> HtmlOutput {
    return run_with(SyntectAdapterBuilder::new().theme("base16-ocean.dark"), body)
}

fn run_with(builder: SyntectAdapterBuilder, body: &str) -> String {
    let adapter = builder.build();
    let options = Options::default();
    let mut plugins = Plugins::default();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    return markdown_to_html_with_plugins(body, &options, &plugins);
}

#[cfg(test)]
mod tests {
    use crate::markdown::parse;
    use indoc::indoc;

    #[test]
    fn parse_should_be_return_to_heading () {
        let html = parse("## Hello");

        assert_eq!(html, "<h2>Hello</h2>\n")
    }

    #[test]
    fn parse_should_be_return_to_highlighted_codeblock () {
        let codeblock = indoc! {"
            ```rs
            fn main() -> String {
                \"fooo\"
            }
            ```
        "};
        let html = parse(codeblock);

        let actual = "<pre style=\"background-color:#2b303b;\"><code class=\"language-rs\"><span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() -&gt; String {\n</span><span style=\"color:#c0c5ce;\">    &quot;</span><span style=\"color:#a3be8c;\">fooo</span><span style=\"color:#c0c5ce;\">&quot;\n</span><span style=\"color:#c0c5ce;\">}\n</span></code></pre>\n";
        assert_eq!(html, actual);
    }
}
