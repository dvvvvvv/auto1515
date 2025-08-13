use bon::Builder;
use derivative::Derivative;

#[derive(Derivative, Builder)]
#[derivative(Debug)]
pub struct Game {
    pub year: u64,
    pub sequence: u64,
}
