use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::web::ui::STYLES_URI;

pub fn page(title: &str, contents: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta http-equiv="X-UA-Compatible" content="IE=edge";
                meta name="viewport" content="width=device-width, initial-scale=1";
                (PreEscaped(r#"<link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🧹</text></svg>">"#))
                                                                                            //
                link rel="stylesheet" href=(STYLES_URI);

                link rel="apple-touch-icon" sizes="60x60" href="/icons/iphone.png";
                link rel="apple-touch-icon" sizes="76x76" href="/icons/ipad.png";
                link rel="apple-touch-icon" sizes="114x114" href="/icons/iphone-retina.png";
                link rel="apple-touch-icon" sizes="144x144" href="/icons/ipad-retina.png";
                meta name="apple-mobile-web-app-capable" content="yes";
                // meta name="apple-mobile-web-app-status-bar-style" content="black-translucent";
                link rel="manifest" href="/manifest.json";

                title { (title) }
            }
            body {
                (contents)
            }
        }
    }
}
