#[derive(Clone, Debug, Copy)]
pub struct Market {
    pub wood: i32,
    pub rice: i32,
    pub land: i32,
}

const WOOD_PRICING: i32 = 1;
const RICE_PRICING: i32 = 1;
const LOAN_RATE: i32 = 10;
const BUY_RATE: i32 = 2;

impl Market {
    pub fn new() -> Market {
        Market {
            wood: 1,
            rice: 1,
            land: 1,
        }
    }
}

#[derive(Clone, Debug, Copy, Default)]
pub struct Player {
    pub rice: i32,
    pub wood: i32,
    pub land: i32,
    pub farm: i32,
    pub forest: i32,
    pub gold: i32,
    pub debt: i32,
    pub skip: usize,
}

impl Player {
    pub fn new() -> Player {
        Player::default()
    }
}

pub fn buy_wood(m: &mut Market, p: &mut Player) -> bool {
    if p.gold >= m.wood {
        p.gold -= m.wood;
        p.wood += BUY_RATE;
        m.wood += WOOD_PRICING;
        return true;
    }
    false
}

pub fn sell_wood(m: &mut Market, p: &mut Player) -> bool {
    if p.wood >= 1 {
        p.wood -= 1;
        m.wood = 1.max(m.wood - WOOD_PRICING);
        p.gold += m.wood;
        return true;
    }
    false
}

pub fn buy_rice(m: &mut Market, p: &mut Player) -> bool {
    if p.gold >= m.rice {
        p.gold -= m.rice;
        p.rice += BUY_RATE;
        m.rice += RICE_PRICING;
        return true;
    }
    false
}

pub fn buy_land(m: &mut Market, p: &mut Player) -> bool {
    if p.gold >= m.land {
        p.gold -= m.land;
        p.land += 1;
        m.land += 1;
        return true;
    }
    false
}

pub fn sell_rice(m: &mut Market, p: &mut Player) -> bool {
    if p.rice >= 1 {
        p.rice -= 1;
        m.rice = 1.max(m.rice - RICE_PRICING);
        p.gold += m.rice;
        return true;
    }
    false
}

pub fn build_farm(_m: &mut Market, p: &mut Player) -> bool {
    if p.wood >= 1 && p.land >= 1 {
        p.farm += 1;
        p.wood -= 1;
        p.land -= 1;
        return true;
    }
    false
}

pub fn build_forest(_m: &mut Market, p: &mut Player) -> bool {
    if p.wood >= 1 && p.land >= 1 {
        p.forest += 1;
        p.wood -= 1;
        p.land -= 1;
        return true;
    }
    false
}

pub fn recolt_rice(_m: &mut Market, p: &mut Player) -> bool {
    if p.farm >= 1 {
        p.farm -= 1;
        p.land += 1;
        p.rice += 10;
        return true;
    }
    false
}

pub fn recolt_wood(_m: &mut Market, p: &mut Player) -> bool {
    if p.forest >= 1 {
        p.forest -= 1;
        p.land += 1;
        p.wood += 10;
        return true;
    }
    false
}

pub fn take_loan(_m: &mut Market, p: &mut Player) -> bool {
    p.gold += LOAN_RATE;
    p.debt += LOAN_RATE;
    true
}

#[must_use]
pub fn score(p: &Player) -> f32 {
    let r = p.rice as f32;
    let w = p.wood as f32;
    let l = p.land as f32;
    let g = p.gold as f32;
    let d = p.debt as f32;
    // let _d = (p.gold - p.debt).max(1) as f32;
    // let _diff = (p.gold - p.debt) as f32;
    // r * w + l + r + w + g - d
    g - d
}

#[must_use]
pub fn get_actions() -> Vec<fn(&mut Market, &mut Player) -> bool> {
    vec![
        take_loan,
        sell_wood,
        buy_rice,
        sell_rice,
        build_farm,
        build_forest,
        recolt_rice,
        recolt_wood,
        buy_land,
        buy_wood,
    ]
}

pub fn get_actions_str() -> Vec<&'static str> {
    vec![
        "take_loan   +10 gold  +10 debt",
        "sell_wood",
        "buy_rice",
        "sell_rice",
        "build_farm",
        "build_forest",
        "recolt_rice",
        "recolt_wood",
        "buy_land",
        "buy_wood",
    ]
}

pub fn print_state(m: &Market, ps: &[Player]) {
    println!(" --------------------------------------------------------------------------------");
    println!("|        |  rice  |  wood  |  land  |  farm  | forest |  gold  |  debt  |  score |");
    println!(
        "| market | {:06} | {:06} | {:06} |        |        |        |        |        |",
        m.rice, m.wood, m.land
    );
    for (i, p) in ps.iter().enumerate() {
        println!(
            "|     p{} | {:06} | {:06} | {:06} | {:06} | {:06} | {:06} | {:06} | {:06} |",
            i + 1,
            p.rice,
            p.wood,
            p.land,
            p.farm,
            p.forest,
            p.gold,
            p.debt,
            score(p),
        );
    }
    println!(" --------------------------------------------------------------------------------");
}
