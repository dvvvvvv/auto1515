use bon::Builder;
use derivative::Derivative;
use reqwest::{
    Client,
    header::{COOKIE, HeaderMap, HeaderValue},
};
use tracing::instrument;

use crate::game::Game;

#[derive(Derivative, Builder)]
#[derivative(Debug)]
#[builder(on(String, into))]
pub struct Applier {
    #[derivative(Debug = "ignore")]
    #[builder(default = applier_default_http_client())]
    http_client: reqwest::Client,
    origin: String,
}

fn applier_default_http_client() -> reqwest::Client {
    Client::builder()
        .gzip(true)
        .build()
        .expect("http client build should be successful")
}

#[derive(Derivative, Builder)]
#[derivative(Debug)]
pub struct ApplyGameCommand {
    pub game: Game,
}

impl Applier {
    #[instrument(err)]
    pub async fn apply(&self, command: ApplyGameCommand) -> Result<(), ApplyError> {
        let url = format!("{}/15/src/frm42_GameAcceptIntro.php", &self.origin);

        let yy = command.game.year.to_string();
        let seq = command.game.sequence.to_string();
        let params = [
            ("pw2", "65260"),
            ("user_sido1", "경기"),
            ("user_team1", "양주 무호정"),
            ("pw1", "65260"),
            ("yy", &yy),
            ("seq", &seq),
        ];

        let res = self.http_client.post(url).form(&params).send().await?;

        let body = res.text().await?;
        // info!(body);
        println!("{}", body);

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ApplyError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl From<reqwest::Error> for ApplyError {
    fn from(value: reqwest::Error) -> Self {
        Self::Anyhow(value.into())
    }
}
