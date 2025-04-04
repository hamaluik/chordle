use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::web::ui::STYLES_URI;

use super::l10n::Lang;

pub fn page(lang: Lang, title: &str, contents: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang=(lang.to_string()) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                (PreEscaped(r#"<link rel="icon" type="image/svg+xml" sizes="any" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🧹</text></svg>">"#))
                link rel="icon" type="image/x-icon" sizes=""16x16 href="/icon.png?s=16&ico=true";
                link rel="stylesheet" href=(STYLES_URI);

                @for s in &[180, 167, 152, 120, 114, 87, 80, 76, 58] {
                    link rel="apple-touch-icon" sizes=(s) href=(format!("/icon.png?s={s}"));
                }
                meta name="apple-mobile-web-app-capable" content="yes";
                meta name="apple-mobile-web-app-status-bar-style" content="black-translucent";
                link rel="manifest" href="/manifest.json";

                title { (title) }
            }
            body {
                (contents)
            }
        }
    }
}
