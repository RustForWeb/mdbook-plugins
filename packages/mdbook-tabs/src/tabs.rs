use crate::config::TabsConfig;

pub fn tabs(config: &TabsConfig) -> String {
    format!(
        "<div class=\"mdbook-tabs-container\">\n<ul class=\"mdbook-tabs\">\n{}\n</ul>\n{}\n</div>",
        config
            .tabs
            .iter()
            .map(|(tab, _)| format!("<li class=\"mdbook-tab\">{}</li>", tab.name))
            .collect::<Vec<_>>()
            .join("\n"),
        config
            .tabs
            .iter()
            .map(|(_, tab_content)| format!(
                "<div class=\"mdbook-tab-content\">\n\n{}\n\n</div>",
                tab_content
            ))
            .collect::<Vec<_>>()
            .join("\n")
    )
}
