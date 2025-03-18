use crate::cli::Cli;
use color_eyre::Result;
use jiff::Zoned;
use std::io::IsTerminal;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    Layer, Registry, filter,
    fmt::{format::Writer, time::FormatTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn setup_logging(cli: &Cli) -> Result<()> {
    let use_colours = match cli.colour {
        clap::ColorChoice::Never => false,
        clap::ColorChoice::Always => true,
        _ => std::io::stdout().is_terminal(),
    };

    color_eyre::config::HookBuilder::new()
        .theme(if use_colours {
            color_eyre::config::Theme::dark()
        } else {
            color_eyre::config::Theme::new()
        })
        .install()
        .expect("Failed to install `color_eyre`");

    let log_level = match cli.verbose {
        0 => LevelFilter::WARN,
        1 => LevelFilter::INFO,
        2 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    let logs_filter = move |metadata: &tracing::Metadata<'_>| {
        log_level == LevelFilter::TRACE
            || (metadata.target().starts_with("chordle") && *metadata.level() <= log_level)
    };

    let stdout_log = tracing_subscriber::fmt::layer()
        // .pretty()
        .with_ansi(use_colours)
        .with_timer(JiffLocal::default())
        .with_target(false)
        .with_level(true)
        .with_writer(std::io::stdout)
        .with_filter(filter::filter_fn(logs_filter));

    Registry::default().with(stdout_log).init();
    Ok(())
}

#[derive(Default)]
struct JiffLocal {}

impl FormatTime for JiffLocal {
    fn format_time(&self, w: &mut Writer<'_>) -> core::fmt::Result {
        let now = Zoned::now();
        write!(w, "{now}")
    }
}
