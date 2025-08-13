use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

use crate::{
    game::Game,
    reserve::{Applier, ApplyGameCommand},
};

mod game;
mod reserve;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let applier = Applier::builder()
        .origin("https://www.xn--289a1m3b91b.kr")
        .build();

    let game = Game::builder().year(2025).sequence(1).build();

    let command = ApplyGameCommand::builder().game(game).build();

    applier.apply(command).await?;

    Ok(())
}

fn init_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::{Registry, filter::LevelFilter, fmt::layer, layer::SubscriberExt};

    let level_filter = LevelFilter::from_level(Level::INFO);

    let fmt_layer = layer()
        .json()
        .with_current_span(true)
        .with_span_list(true)
        .flatten_event(true)
        .with_target(true)
        .with_level(true);

    Registry::default()
        .with(level_filter)
        .with(ErrorLayer::default())
        .with(fmt_layer)
        .init();
}
