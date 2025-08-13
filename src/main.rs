use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

use crate::{
    game::Game,
    reserve::{Applier, ApplyGameCommand},
    user::User,
};

mod game;
mod reserve;
mod user;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let applier = Applier::builder()
        .origin("https://www.xn--289a1m3b91b.kr")
        .build();
    let user = User::builder()
        .id("dvvvvvv")
        .area("양주 무호정")
        .jung("양주 무호정")
        .name("임석민")
        .nickname("임석민")
        .sido("경기")
        .team("양주 무호정")
        .build();

    let game = Game::builder().year(2025).sequence(1).build();

    let command = ApplyGameCommand::builder().game(game).user(user).build();

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
