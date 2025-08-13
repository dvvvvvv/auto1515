use bon::Builder;
use cookie::Cookie;
use derivative::Derivative;
use reqwest::{Client, header::COOKIE};
use tracing::instrument;

use crate::{game::Game, user::User};

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
    pub user: User,
}

impl Applier {
    #[instrument(err)]
    pub async fn apply(&self, command: ApplyGameCommand) -> Result<(), ApplyError> {
        let url = format!("{}/15/src/frm42_GameAcceptIntro.php", &self.origin);

        let user = &command.user;

        let cookies = vec![
            Cookie::new("user_1515", "1515"),
            Cookie::new("user_15id", &user.id),
            Cookie::new("user_15area", &user.area),
            Cookie::new("user_15jung", &user.jung),
            Cookie::new("user_15name", &user.name),
            Cookie::new("user_15nick", &user.nickname),
            Cookie::new("user_15sido", &user.sido),
            Cookie::new("user_15team", &user.team),
        ];

        let yy = command.game.year.to_string();
        let seq = command.game.sequence.to_string();
        let params = [
            ("pw2", "65260"),
            ("user_sido1", &user.sido),
            ("user_team1", &user.team),
            ("pw1", "65260"),
            ("yy", &yy),
            ("seq", &seq),
        ];

        let res = self
            .http_client
            .post(url)
            .header(
                COOKIE,
                cookies
                    .iter()
                    .map(|cookie| cookie.encoded().to_string())
                    .collect::<Vec<_>>()
                    .join("; "),
            )
            .form(&params)
            .send()
            .await?;

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
