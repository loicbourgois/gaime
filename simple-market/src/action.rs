use crate::Market;
use crate::Player;
const LOAN_RATE: i32 = 10;
const BUY_RATE: i32 = 1;
const WOOD_PRICING: i32 = 1;
pub trait Action {
    fn run(&self, m: &mut Market, p: &mut Player) -> bool;
    fn to_string(&self) -> &str;
}
pub struct TakeLoan {}
impl Action for TakeLoan {
    fn run(&self, _m: &mut Market, p: &mut Player) -> bool {
        p.gold += LOAN_RATE;
        p.debt += LOAN_RATE;
        true
    }
    fn to_string(&self) -> &str {
        "take loan"
    }
}
pub struct BuyWood {}
impl Action for BuyWood {
    fn run(&self, m: &mut Market, p: &mut Player) -> bool {
        if p.gold >= m.wood {
            p.gold -= m.wood;
            p.wood += BUY_RATE;
            m.wood += WOOD_PRICING;
            return true;
        }
        false
    }
    fn to_string(&self) -> &str {
        "buy wood"
    }
}
pub struct SellWood {}
impl Action for SellWood {
    fn run(&self, m: &mut Market, p: &mut Player) -> bool {
        if p.wood >= 1 {
            p.wood -= 1;
            m.wood = 1.max(m.wood - WOOD_PRICING);
            p.gold += m.wood;
            return true;
        }
        false
    }
    fn to_string(&self) -> &str {
        "sell wood"
    }
}
pub fn get_actions() -> Vec<Box<dyn Action>> {
    vec![
        Box::new(BuyWood {}),
        // Box::new(BuyRice {}),
        // Box::new(BuyLand {}),
        Box::new(SellWood {}),
        // Box::new(SellRice {}),
        // Box::new(SellLand {}),
        // Box::new(BuildFarm {}),
        // Box::new(BuildForest {}),
        // Box::new(RecoltRice {}),
        Box::new(TakeLoan {}),
        // Box::new(RecoltWood {}),
    ]
}
pub fn get_actions_str(actions: &'static [Box<dyn Action>]) -> Vec<&'static str> {
    actions.iter().map(|x| x.to_string()).collect()
}
