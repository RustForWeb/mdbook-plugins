use crate::config::TabsConfig;

pub fn tabs(config: &TabsConfig) -> String {
    format!(
        "<div class=\"mdbook-tabs-container\"{}>\n<nav class=\"mdbook-tabs\">\n{}\n</nav>\n{}\n</div>",
        config
            .global
            .as_ref()
            .map(|global| format!(" data-tabglobal=\"{global}\""))
            .unwrap_or("".to_string()),
        config
            .tabs
            .iter()
            .enumerate()
            .map(|(index, (tab, _))| format!(
                "<button class=\"mdbook-tab{}\" data-tabname=\"{}\">{}</button>",
                match index == 0 {
                    true => " active",
                    false => "",
                },
                tab.name,
                tab.name
            ))
            .collect::<Vec<_>>()
            .join("\n"),
        config
            .tabs
            .iter()
            .enumerate()
            .map(|(index, (tab, tab_content))| format!(
                "<div class=\"mdbook-tab-content{}\" data-tabname=\"{}\">\n\n{}\n\n</div>",
                match index == 0 {
                    true => "",
                    false => " hidden",
                },
                tab.name,
                tab_content
            ))
            .collect::<Vec<_>>()
            .join("\n")
    )
}
