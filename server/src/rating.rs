use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyGlicko2Rating {
    pub value: f64,
    pub deviation: f64,
    pub volatility: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rating {
    pub old_rating: MyGlicko2Rating,
    pub new_rating: MyGlicko2Rating,
}
