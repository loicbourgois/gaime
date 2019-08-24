use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MyGlicko2Rating {
    pub value: f64,
    pub deviation: f64,
    pub volatility: f64
}

#[derive(Serialize, Deserialize)]
pub struct Rating {
    pub old_rating: MyGlicko2Rating,
    pub new_rating: MyGlicko2Rating
}
