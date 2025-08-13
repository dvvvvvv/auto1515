use bon::Builder;
use derivative::Derivative;

#[derive(Derivative, Builder)]
#[derivative(Debug)]
#[builder(on(String, into))]
pub struct User {
    pub id: String,
    pub area: String,
    pub jung: String,
    pub name: String,
    pub nickname: String,
    pub sido: String,
    pub team: String,
}
