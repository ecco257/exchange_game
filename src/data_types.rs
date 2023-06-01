pub enum Round {
    Integer(i32),
    String(String),
}

pub fn f(round: &Round) -> i32 {
    match round {
        Round::Integer(i) => *i,
        // throw an error if the round is not an integer
        Round::String(_s) => panic!("Round is not an integer"),
    }
}

impl std::fmt::Display for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Round::Integer(i) => write!(f, "{}", i),
            Round::String(s) => write!(f, "{}", s),
        }
    }
}

impl PartialEq for Round {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Round::Integer(i1), Round::Integer(i2)) => i1 == i2,
            (Round::String(s1), Round::String(s2)) => s1 == s2,
            _ => false,
        }
    }
}

pub struct GameState {
    pub round: Round,
    pub phase: i32,
    pub lobbyist: String,
    pub bank_price: i32,
    pub bond_price: i32,
    pub insurance_price: i32,
    pub bank_influence: i32,
    pub bond_influence: i32,
    pub insurance_influence: i32,
}

impl GameState {
    pub fn new(
        round: Round,
        phase: i32,
        lobbyist: String,
        bank_price: i32,
        bond_price: i32,
        insurance_price: i32,
        bank_influence: i32,
        bond_influence: i32,
        insurance_influence: i32,
    ) -> Self {
        GameState {
            round,
            phase,
            lobbyist,
            bank_price,
            bond_price,
            insurance_price,
            bank_influence,
            bond_influence,
            insurance_influence,
        }
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  Round: {}\n  Phase: {}\n  Lobbyist: {}\n  Bank Price: ${}\n  Bond Price: ${}\n  Insurance Price: ${}",
            self.round, self.phase, self.lobbyist, self.bank_price, self.bond_price, self.insurance_price
        )
    }
}

pub struct PlayerState {
    pub name: String,
    pub cash: i32,
    pub banks: i32,
    pub bonds: i32,
    pub insurance: i32,
    pub security: String,
    pub action: String,
    pub quantity: i32,
    pub influence: (String, String),
}

impl PlayerState {
    pub fn new(
        name: String,
        cash: i32,
        banks: i32,
        bonds: i32,
        insurance: i32,
        security: String,
        action: String,
        quantity: i32,
        influence: (String, String),
    ) -> Self {
        PlayerState {
            name,
            cash,
            banks,
            bonds,
            insurance,
            security,
            action,
            quantity,
            influence,
        }
    }
    
}

impl Clone for PlayerState {
    fn clone(&self) -> Self {
        PlayerState {
            name: self.name.clone(),
            cash: self.cash,
            banks: self.banks,
            bonds: self.bonds,
            insurance: self.insurance,
            security: self.security.clone(),
            action: self.action.clone(),
            quantity: self.quantity,
            influence: self.influence.clone(),
        }
    }
}

impl std::fmt::Display for PlayerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  Name: {}\n  Cash: ${}\n  Banks: {}\n  Bonds: {}\n  Insurance: {}",
            self.name, self.cash, self.banks, self.bonds, self.insurance
        )
    }
}

pub struct MarketForce {
    pub title: String,
    pub description: String,
    pub impact: String,
}

impl MarketForce {
    pub fn new(title: String, description: String, impact: String) -> Self {
        MarketForce {
            title,
            description,
            impact,
        }
    }
}

impl std::fmt::Display for MarketForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "  {}\n  {}\n  {}",
            self.title, self.description, self.impact
        )
    }
}
