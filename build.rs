use std::collections::HashMap;

fn main() {
    println!("cargo:rerun-if-changed=migrations");

    // set a compile-time environment variable that can be used in the
    // Last-Modified header, by executing the `date` command at build time
    let last_modified = std::process::Command::new("date")
        .arg("-u")
        .arg("+%a, %d %b %Y %T GMT")
        .output()
        .expect("Can run date command");
    println!(
        "cargo:rustc-env=BUILD_TIME_LAST_MODIFIED={}",
        String::from_utf8_lossy(&last_modified.stdout)
    );

    ensure_translations_exist();
}

fn ensure_translations_exist() {
    // first ensure that all translation files exist
    let langs = ["en", "fr"];
    for lang in langs.iter() {
        let path = format!("src/web/ui/l10n/{}.ftl", lang);
        if !std::path::Path::new(&path).exists() {
            panic!("Translation file {} does not exist", path);
        }
    }

    // now collect the messages for each language
    let lang_messages: HashMap<&str, Vec<String>> = langs
        .iter()
        .map(|&lang| {
            let path = format!("src/web/ui/l10n/{}.ftl", lang);
            let contents = std::fs::read_to_string(&path).expect("Can read file");
            let resource =
                fluent::FluentResource::try_new(contents).expect("Can parse FTL resource");
            let entries: Vec<String> = resource
                .entries()
                .filter_map(|entry| {
                    if let fluent_syntax::ast::Entry::Message(msg) = entry {
                        Some(msg.id.name.to_string())
                    } else {
                        None
                    }
                })
                .collect();
            (lang, entries)
        })
        .collect();

    // finally, ensure that all languages have the same messages
    for (lang, messages) in lang_messages.iter() {
        for (other_lang, other_messages) in lang_messages.iter() {
            if lang == other_lang {
                continue;
            }

            let missing: Vec<&String> = messages
                .iter()
                .filter(|msg| !other_messages.contains(msg))
                .collect();
            if !missing.is_empty() {
                panic!(
                    "Language {other_lang} is missing messages from language {lang}: {missing:?}"
                );
            }
        }
    }
}
