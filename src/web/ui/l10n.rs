use std::{collections::HashMap, fmt::Display, sync::RwLock};

use axum_extra::extract::CookieJar;
use color_eyre::eyre::ContextCompat;
use fluent::{FluentArgs, FluentResource, bundle::FluentBundle};
use unic_langid::langid;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub enum Lang {
    #[default]
    En,
    Fr,
}

type TranslationType = FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;

pub struct L10N {
    bundles: RwLock<HashMap<Lang, TranslationType>>,
}

impl std::fmt::Debug for L10N {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("L10N").finish()
    }
}

impl L10N {
    fn load_bundle(lang: Lang) -> TranslationType {
        let langid = match lang {
            Lang::En => langid!("en"),
            Lang::Fr => langid!("fr"),
        };
        let mut bundle = FluentBundle::new_concurrent(vec![langid]);

        let resource = match lang {
            Lang::En => include_str!("./l10n/en.ftl"),
            Lang::Fr => include_str!("./l10n/fr.ftl"),
        };
        let resource =
            FluentResource::try_new(resource.to_string()).expect("Failed to parse FTL resource");
        bundle
            .add_resource(resource)
            .expect("Failed to add FTL resource");

        bundle
    }

    pub fn new() -> L10N {
        let bundles: HashMap<Lang, TranslationType> = [Lang::En, Lang::Fr]
            .map(|lang| (lang, Self::load_bundle(lang)))
            .into_iter()
            .collect();

        let bundles = RwLock::new(bundles);
        L10N { bundles }
    }

    fn _translate<S: AsRef<str>>(&self, lang: Lang, key: S, args: Option<&FluentArgs>) -> String {
        let bundles = self.bundles.read().expect("Can read bundles");

        let bundle = bundles
            .get(&lang)
            .wrap_err_with(|| "Language {lang:?} not found in bundles")
            .expect("Can get language bundle");
        let key = key.as_ref();
        let msg = bundle
            .get_message(key)
            .wrap_err_with(|| format!("Message `{key}` not found in bundle for language {lang:?}"))
            .expect("Can get message by key");
        let pattern = msg
            .value()
            .wrap_err_with(|| format!("Message `{key}` in language {lang:?} has no pattern"))
            .expect("Can get pattern of message");

        let mut errors = vec![];
        let result = bundle.format_pattern(&pattern, args, &mut errors);

        if !errors.is_empty() {
            tracing::error!(
                "L10N errors were encountered while translating `{key}` in {lang:?}: {:?}",
                errors
            );
        }

        result.into_owned()
    }

    pub fn translate_with<S: AsRef<str>>(&self, lang: Lang, key: S, args: FluentArgs) -> String {
        self._translate(lang, key, Some(&args))
    }

    pub fn translate<S: AsRef<str>>(&self, lang: Lang, key: S) -> String {
        self._translate(lang, key, None)
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lang::En => write!(f, "en"),
            Lang::Fr => write!(f, "fr"),
        }
    }
}

impl Lang {
    pub fn from_accept_language_header_and_cookie(header: Option<&str>, jar: &CookieJar) -> Lang {
        if jar.get("lang").is_some() {
            match jar.get("lang").unwrap().value() {
                "en" => return Lang::En,
                "fr" => return Lang::Fr,
                _ => {}
            };
        }

        if let Some(header) = header {
            for lang in header.split(',') {
                if let Some(lang) = lang.split(';').next() {
                    if let Some(lang) = lang.split('-').next() {
                        match lang.trim() {
                            "*" | "en" => return Lang::En,
                            "fr" => return Lang::Fr,
                            _ => {}
                        }
                    }
                }
            }
        }

        Lang::En
    }

    pub fn from_str(s: &str) -> Lang {
        match s.to_lowercase().as_str() {
            "fr" => Lang::Fr,
            _ => Lang::En,
        }
    }
}
