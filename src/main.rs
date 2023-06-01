mod data_types;

use std::{io::{self, Write}, collections::VecDeque};

use rand::seq::SliceRandom;
use rand::thread_rng;

use rpassword;

use data_types::{GameState, MarketForce, PlayerState, Round, f};

fn play_round(current_state: &mut GameState, players: &mut Vec<PlayerState>, market_forces: &mut VecDeque<MarketForce>) {
    println!("{}", current_state);
    println!();
    // reset the current state influences
    current_state.bank_influence = 0;
    current_state.bond_influence = 0;
    current_state.insurance_influence = 0;

    phase1(current_state, players);
    phase2(current_state, players);
    phase3(current_state, players);
    calculate_states(current_state, players, market_forces);
}

fn market_close(current_state: &mut GameState, players: &mut Vec<PlayerState>) {
    println!("{}", current_state.round);
    current_state.phase = 3;
    
    for player in &mut *players {
        player.quantity = 0;
    }

    // reset the current state influences
    current_state.bank_influence = 0;
    current_state.bond_influence = 0;
    current_state.insurance_influence = 0;

    phase3(current_state, players);
    calculate_states(current_state, players, &mut VecDeque::new());
}

fn phase1(current_state: &mut GameState, players: &mut Vec<PlayerState>) {
    assert!(current_state.round != Round::String("Market Close".to_owned()), "round must not be Market Close");
    assert!(current_state.phase == 1, "phase must be 1");
    
    for player in &mut *players {
        let mut decision_is_not_valid = true;
        let mut decision = String::new();
        while decision_is_not_valid {
            print!("{}, which security would you like to trade? (ba for Bank, bo for Bond, i for Insurance) ", player.name);
            io::stdout().flush().unwrap();
            
            decision.clear();
            decision = rpassword::read_password().unwrap();
            decision = decision.trim().to_string();
            match decision.as_str() {
                "ba" => {
                    player.security = "Bank".to_string();
                    decision_is_not_valid = false;
                }
                "bo" => {
                    player.security = "Bond".to_string();
                    decision_is_not_valid = false;
                }
                "i" => {
                    player.security = "Insurance".to_string();
                    decision_is_not_valid = false;
                }
                _ => {
                    println!("Invalid security - must be one of ba, bo, or i");
                }
            }
        }
        println!("");
    }
    
    for player in &mut *players {
        println!("{} chose to trade {}", player.name, player.security);
    }
    println!("");
    
    current_state.phase = 2;
}

fn phase2(current_state: &mut GameState, players: &mut Vec<PlayerState>) {
    assert!(matches!(current_state.round, Round::Integer(_)), "round must not be Market Close");
    assert_eq!(current_state.phase, 2, "phase must be 2");

    for player in &mut *players {
        let mut decision_is_not_valid = true;
        let mut quantity = 0;
        let mut decision = String::new();
        while decision_is_not_valid {
            print!("{}, would you like to buy or sell? (b for Buy, s for Sell) ", player.name);
            io::stdout().flush().unwrap();

            decision.clear();
            decision = rpassword::read_password().unwrap();
            decision = decision.trim().to_string();

            
            print!("How much {} would you like to trade? ", player.security);
            io::stdout().flush().unwrap();
            let quantity_str = rpassword::read_password().unwrap();
            quantity = match quantity_str.trim().parse() {
                Ok(qty) => qty,
                Err(_) => {
                    println!("Invalid quantity - must be an integer");
                    continue;
                }
            };

            match decision.as_str() {
                "b" => {
                    if quantity < 1 || quantity > 9 {
                        println!("Invalid quantity - must be in range [1, 9]");
                        continue;
                    }
                    if player.security == "Bank" {
                        if quantity + player.banks <= 24 {
                            decision_is_not_valid = false;
                        }
                    } else if player.security == "Bond" {
                        if quantity + player.bonds <= 24 {
                            decision_is_not_valid = false;
                        }
                    } else if player.security == "Insurance" {
                        if quantity + player.insurance <= 24 {
                            decision_is_not_valid = false;
                        }
                    }
                }
                "s" => {
                    if quantity < 1 || quantity > 9 {
                        println!("Invalid quantity - must be in range [1, 9]");
                        continue;
                    }
                    if player.security == "Bank" {
                        if quantity <= player.banks {
                            decision_is_not_valid = false;
                        }
                    } else if player.security == "Bond" {
                        if quantity <= player.bonds {
                            decision_is_not_valid = false;
                        }
                    } else if player.security == "Insurance" {
                        if quantity <= player.insurance {
                            decision_is_not_valid = false;
                        }
                    }
                }
                _ => {
                    println!("Invalid decision - must be one of b or s");
                    continue;
                }
            }

            if decision_is_not_valid {
                println!("Invalid quantity - must be within your holdings");
            }
        }
        
        if decision == "b" {
            player.action = "Buy".to_string();
        } else if decision == "s" {
            player.action = "Sell".to_string();
        }

        player.quantity = quantity;
        println!("");
    }

    for player in &mut *players {
        println!("{} chose to {} {}", player.name, player.action, player.security);
    }
    println!("");
    
    current_state.phase = 3;
}

fn phase3(current_state: &mut GameState, players: &mut Vec<PlayerState>) {
    assert_eq!(current_state.phase, 3, "phase must be 3");

    let mut max_cash = 0;
    let mut num_lobbyists = 0;

    for player in &mut *players {
        if player.cash > max_cash {
            max_cash = player.cash;
        }
    }

    for player in &mut *players {
        if player.cash == max_cash {
            current_state.lobbyist = player.name.clone();
            num_lobbyists += 1;
        }
    }

    if num_lobbyists > 1 {
        current_state.lobbyist = String::new();
    } else {
        println!("{} is the lobbyist", current_state.lobbyist);
    }

    println!("");

    for player in &mut *players {
        let mut decision_is_not_valid = true;
        let mut decision = String::new();
        let mut direction = String::new();

        while decision_is_not_valid {
            print!("{}, which security would you like to influence? (ba for Bank, bo for Bond, i for Insurance) ", player.name);
            io::stdout().flush().unwrap();

            decision.clear();
            decision = rpassword::read_password().unwrap();
            decision = decision.trim().to_string();

            print!("Would you like to increase or decrease the influence of this security by 1? (i for Increase, d for Decrease) ");
            io::stdout().flush().unwrap();

            direction.clear();
            direction = rpassword::read_password().unwrap();
            direction = direction.trim().to_string();

            if decision == "ba" || decision == "bo" || decision == "i" {
                if direction == "i" || direction == "d" {
                    decision_is_not_valid = false;
                } else {
                    println!("Invalid direction - must be one of i or d");
                }
            } else {
                println!("Invalid security - must be one of ba, bo, or i");
            }
        }

        let influence = match (decision.as_str(), direction.as_str()) {
            ("ba", "i") => {
                current_state.bank_influence += 1;
                ("Bank".to_string(), "Increase".to_string())
            }
            ("ba", "d") => {
                current_state.bank_influence -= 1;
                ("Bank".to_string(), "Decrease".to_string())
            }
            ("bo", "i") => {
                current_state.bond_influence += 1;
                ("Bond".to_string(), "Increase".to_string())
            }
            ("bo", "d") => {
                current_state.bond_influence -= 1;
                ("Bond".to_string(), "Decrease".to_string())
            }
            ("i", "i") => {
                current_state.insurance_influence += 1;
                ("Insurance".to_string(), "Increase".to_string())
            }
            ("i", "d") => {
                current_state.insurance_influence -= 1;
                ("Insurance".to_string(), "Decrease".to_string())
            }
            _ => unreachable!(),
        };

        player.influence = influence;

        println!("");
    }

    if !current_state.lobbyist.is_empty() {
        if let Some(lobbyist_player) = players.iter_mut().find(|player| player.name == current_state.lobbyist) {
            let mut decision_is_not_valid = true;
            let mut decision = String::new();
            let mut direction = String::new();

            while decision_is_not_valid {
                print!("{}, you are the lobbyist. Which security would you like to influence? (ba for Bank, bo for Bond, i for Insurance) ", current_state.lobbyist);
                io::stdout().flush().unwrap();

                decision.clear();
                decision = rpassword::read_password().unwrap();
                decision = decision.trim().to_string();

                print!("Would you like to increase or decrease the influence of this security by 1? (i for Increase, d for Decrease) ");
                io::stdout().flush().unwrap();

                direction.clear();
                direction = rpassword::read_password().unwrap();
                direction = direction.trim().to_string();

                if decision == "ba" || decision == "bo" || decision == "i" {
                    if direction == "i" || direction == "d" {
                        decision_is_not_valid = false;
                    } else {
                        println!("Invalid direction - must be one of i or d");
                    }
                } else {
                    println!("Invalid security - must be one of ba, bo, or i");
                }
            }

            let influence = match (decision.as_str(), direction.as_str()) {
                ("ba", "i") => {
                    current_state.bank_influence += 1;
                    ("Bank".to_string(), "Increase".to_string())
                }
                ("ba", "d") => {
                    current_state.bank_influence -= 1;
                    ("Bank".to_string(), "Decrease".to_string())
                }
                ("bo", "i") => {
                    current_state.bond_influence += 1;
                    ("Bond".to_string(), "Increase".to_string())
                }
                ("bo", "d") => {
                    current_state.bond_influence -= 1;
                    ("Bond".to_string(), "Decrease".to_string())
                }
                ("i", "i") => {
                    current_state.insurance_influence += 1;
                    ("Insurance".to_string(), "Increase".to_string())
                }
                ("i", "d") => {
                    current_state.insurance_influence -= 1;
                    ("Insurance".to_string(), "Decrease".to_string())
                }
                _ => unreachable!(),
            };

            lobbyist_player.influence = influence;

            println!("");
        }
    }

    for player in &mut *players {
        println!("{} chose to {} the {} market by 1", player.name, player.influence.1, player.influence.0);
    }

    if !current_state.lobbyist.is_empty() {
        if let Some(lobbyist_player) = players.iter().find(|player| player.name == current_state.lobbyist) {
            println!("{} (lobbyist) chose to {} the {} market by 1", current_state.lobbyist, lobbyist_player.influence.1, lobbyist_player.influence.0);
        }
    }

    println!("");
}

fn calculate_states(current_state: &mut GameState, players: &mut Vec<PlayerState>, market_forces: &mut VecDeque<MarketForce>) {
    let mut bubble_can_pop = true;
    if current_state.round != Round::String("Market Close".to_owned()) {
        assert!(!market_forces.is_empty(), "market_forces must not be empty");
        let market_force = market_forces.pop_front().unwrap();
        println!("Market Force: ");
        println!("{}", market_force);

        match market_force.title.as_str() {
            "HOUSING BOOM" => current_state.bond_influence += 2,
            "GAINING INFLUENCE" => {
                current_state.bank_influence *= 2;
                current_state.bond_influence *= 2;
                current_state.insurance_influence *= 2;
            }
            "OIL BUST" => current_state.bank_influence -= 2,
            "POLICY CHANGE" => current_state.insurance_influence -= 2,
            "LACK OF INTEREST" => {
                let mut bank_sold = 0;
                let mut bond_sold = 0;
                let mut insurance_sold = 0;

                for player in &mut *players {
                    if player.action == "Sell" {
                        match player.security.as_str() {
                            "Bank" => bank_sold += player.quantity,
                            "Bond" => bond_sold += player.quantity,
                            "Insurance" => insurance_sold += player.quantity,
                            _ => panic!("Invalid security"),
                        }
                    }
                }

                let max_sold = bank_sold.max(bond_sold).max(insurance_sold);
                if max_sold != 0 {
                    if max_sold == bank_sold {
                        current_state.bank_influence -= 1;
                    } else if max_sold == bond_sold {
                        current_state.bond_influence -= 1;
                    } else if max_sold == insurance_sold {
                        current_state.insurance_influence -= 1;
                    }
                }
            }
            "STABLE MARKET" => (),
            "SMALL BUSINESS BOOM" => current_state.bank_influence += 2,
            "FED INTERVENTION" => bubble_can_pop = false,
            "HIGH DEMAND" => {
                let mut bank_bought = 0;
                let mut bond_bought = 0;
                let mut insurance_bought = 0;

                for player in &mut *players {
                    if player.action == "Buy" {
                        match player.security.as_str() {
                            "Bank" => bank_bought += player.quantity,
                            "Bond" => bond_bought += player.quantity,
                            "Insurance" => insurance_bought += player.quantity,
                            _ => panic!("Invalid security"),
                        }
                    }
                }

                let max_bought = bank_bought.max(bond_bought).max(insurance_bought);
                if max_bought != 0 {
                    if max_bought == bank_bought {
                        current_state.bank_influence += 1;
                    } else if max_bought == bond_bought {
                        current_state.bond_influence += 1;
                    } else if max_bought == insurance_bought {
                        current_state.insurance_influence += 1;
                    }
                }
            }
            "NATIONAL DEBT" => current_state.bond_influence -= 2,
            "CURE ALL" => current_state.insurance_influence += 2,
            "TRADE DEAL" => {
                current_state.bank_influence += 1;
                current_state.bond_influence += 1;
                current_state.insurance_influence += 1;
            }
            "NATURAL DISASTER" => {
                current_state.bank_influence -= 1;
                current_state.bond_influence -= 1;
                current_state.insurance_influence -= 1;
            }
            _ => panic!("Invalid market force title"),
        }
    }

    current_state.bank_price += current_state.bank_influence * 10;
    current_state.bond_price += current_state.bond_influence * 10;
    current_state.insurance_price += current_state.insurance_influence * 10;

    if current_state.bank_price < 10 {
        if bubble_can_pop {
            current_state.bank_price = 90 + current_state.bank_price;
        } else {
            current_state.bank_price = 10;
        }
    } else if current_state.bank_price > 90 {
        if bubble_can_pop {
            current_state.bank_price -= 90;
        } else {
            current_state.bank_price = 90;
        }
    }

    if current_state.bond_price < 10 {
        if bubble_can_pop {
            current_state.bond_price = 90 + current_state.bond_price;
        } else {
            current_state.bond_price = 10;
        }
    } else if current_state.bond_price > 90 {
        if bubble_can_pop {
            current_state.bond_price -= 90;
        } else {
            current_state.bond_price = 90;
        }
    }

    if current_state.insurance_price < 10 {
        if bubble_can_pop {
            current_state.insurance_price = 90 + current_state.insurance_price;
        } else {
            current_state.insurance_price = 10;
        }
    } else if current_state.insurance_price > 90 {
        if bubble_can_pop {
            current_state.insurance_price -= 90;
        } else {
            current_state.insurance_price = 90;
        }
    }

    println!("Updated State:");
    println!("{}", current_state);

    if current_state.round != Round::String("Market Close".to_owned()) {
        println!("Quantities revealed:");
        for player in &mut *players {
            println!("{} is {}ing {} {}", player.name, player.action, player.quantity, player.security);
        }

        for player in &mut *players {
            if player.action == "Buy" {
                match player.security.as_str() {
                    "Bank" => {
                        player.cash -= current_state.bank_price * player.quantity;
                        player.banks += player.quantity;
                    }
                    "Bond" => {
                        player.cash -= current_state.bond_price * player.quantity;
                        player.bonds += player.quantity;
                    }
                    "Insurance" => {
                        player.cash -= current_state.insurance_price * player.quantity;
                        player.insurance += player.quantity;
                    }
                    _ => panic!("Invalid security"),
                }
            } else if player.action == "Sell" {
                match player.security.as_str() {
                    "Bank" => {
                        player.cash += current_state.bank_price * player.quantity;
                        player.banks -= player.quantity;
                    }
                    "Bond" => {
                        player.cash += current_state.bond_price * player.quantity;
                        player.bonds -= player.quantity;
                    }
                    "Insurance" => {
                        player.cash += current_state.insurance_price * player.quantity;
                        player.insurance -= player.quantity;
                    }
                    _ => panic!("Invalid security"),
                }
            }
        }
    }

    for player in &mut *players {
        let net_worth = player.cash + player.banks * current_state.bank_price +
            player.bonds * current_state.bond_price + player.insurance * current_state.insurance_price;
        println!("{} has {} in cash, {} banks, {} bonds, and {} insurance, for a net worth of {}",
            player.name, player.cash, player.banks, player.bonds, player.insurance, net_worth);
    }

    assert_eq!(current_state.phase, 3);
    current_state.phase = 1;

    if current_state.round != Round::String("Market Close".to_owned()) && f(&current_state.round) < 5 {
        let next_round = f(&current_state.round) + 1;
        current_state.round = Round::Integer(next_round);
    } else {
        current_state.round = Round::String("Market Close".to_owned());
    }

    println!("\n\n");
}

fn print_results(current_state: &GameState, players: &[PlayerState]) {
    println!("The game is over! Here are the final results:");
    println!("Final State:");
    println!("{}", current_state);
    println!();

    println!("Final Player States:");
    for (i, player) in players.iter().enumerate() {
        println!("Player {}: ", i + 1);
        println!("{}", player);

        // Print net worth
        let net_worth = player.cash + player.banks * current_state.bank_price +
            player.bonds * current_state.bond_price + player.insurance * current_state.insurance_price;
        println!("  Net Worth: ${}", net_worth);

        // Print rank
        let mut rank = 1;
        for other_player in players {
            if other_player.cash + other_player.banks * current_state.bank_price +
                other_player.bonds * current_state.bond_price +
                other_player.insurance * current_state.insurance_price > net_worth
            {
                rank += 1;
            }
        }
        println!("  Rank: {}", rank);
    }

    println!("\nThanks for playing!");
}

fn run() {
    let mut decision_is_not_valid = true;
    let mut num_players = String::new();
    while decision_is_not_valid {
        print!("Welcome to Exchange! How many players are there? ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut num_players).unwrap();
        let num_players: u32 = match num_players.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number between 2 and 6.");
                continue;
            }
        };
        if num_players < 2 || num_players > 6 {
            println!("Please enter a number between 2 and 6.");
        } else {
            decision_is_not_valid = false;
        }
    }

    let mut rng = thread_rng();
    let mut players_to_choose_from = vec![
        PlayerState::new("David Reedy".to_owned(), 150, 0, 6, 11, String::new(), String::new(), 0, (String::new(), String::new())),
        PlayerState::new("Andrew Barclay".to_owned(), 300, 4, 4, 6, String::new(), String::new(), 0, (String::new(), String::new())),
        PlayerState::new("Benjamin Wintrop".to_owned(), 400, 6, 2, 4, String::new(), String::new(), 0, (String::new(), String::new())),
        PlayerState::new("Peter Auspach".to_owned(), 400, 2, 6, 4, String::new(), String::new(), 0, (String::new(), String::new())),
        PlayerState::new("Hugh Smith".to_owned(), 350, 5, 6, 2, String::new(), String::new(), 0, (String::new(), String::new())),
        PlayerState::new("Leonard Bleecker".to_owned(), 100, 8, 4, 6, String::new(), String::new(), 0, (String::new(), String::new())),
        PlayerState::new("Gideon McEvers".to_owned(), 300, 10, 4, 0, String::new(), String::new(), 0, (String::new(), String::new())),
        PlayerState::new("Augustine Lawrence".to_owned(), 100, 4, 8, 6, String::new(), String::new(), 0, (String::new(), String::new())),
        PlayerState::new("Samuel Beebe".to_owned(), 250, 5, 5, 5, String::new(), String::new(), 0, (String::new(), String::new())),
        PlayerState::new("Isaac M. Gomez".to_owned(), 200, 3, 10, 3, String::new(), String::new(), 0, (String::new(), String::new())),
        PlayerState::new("Alexander Zuntz".to_owned(), 200, 10, 3, 3, String::new(), String::new(), 0, (String::new(), String::new())),
        PlayerState::new("Ephraim Hart".to_owned(), 200, 3, 3, 10, String::new(), String::new(), 0, (String::new(), String::new())),
    ];

    players_to_choose_from.shuffle(&mut rng);

    let mut market_forces = vec![
        MarketForce::new(
            "HOUSING BOOM".to_owned(),
            "Lowered interest rates trigger a housing boom and cause bond values to rise.".to_owned(),
            "BONDS +2".to_owned(),
        ),
        MarketForce::new(
            "GAINING INFLUENCE".to_owned(),
            "The Street is abuzz with activity and the world is watching. Your economic influence is continuing to grow".to_owned(),
            "ALL PLAYERS' MARKET INFLUENCE CHANGES FROM +/- 1 TO +/- 2".to_owned(),
        ),
        MarketForce::new(
            "OIL BUST".to_owned(),
            "An oil bust is causing banks to take a steep loss in the oil industry.".to_owned(),
            "BANKS -2".to_owned(),
        ),
        MarketForce::new(
            "POLICY CHANGE".to_owned(),
            "Big moves in national healthcare leave insurance companies flailing.".to_owned(),
            "INSURANCE -2".to_owned(),
        ),
        MarketForce::new(
            "LACK OF INTEREST".to_owned(),
            "Everyone seems to be selling. The low demand causes the price to drop. Add up the total quantity (from Phase 2) of each security being sold.".to_owned(),
            "MOST SOLD SECURITY -1".to_owned(),
        ),
        MarketForce::new(
            "STABLE MARKET".to_owned(),
            "They say no news is good news. The market is performing as planned.".to_owned(),
            "NO EFFECT THIS ROUND".to_owned(),
        ),
        MarketForce::new(
            "GAINING INFLUENCE".to_owned(),
            "The Street is abuzz with activity and the world is watching. Your economic influence is continuing to grow".to_owned(),
            "ALL PLAYERS' MARKET INFLUENCE CHANGES FROM +/- 1 TO +/- 2".to_owned(),
        ),
        MarketForce::new(
            "SMALL BUSINESS BOOM".to_owned(),
            "The Federal Reserve lowers interest rates resulting in an increase in loans to small businesses.".to_owned(),
            "BANKS +2".to_owned(),
        ),
        MarketForce::new(
            "FED INTERVENTION".to_owned(),
            "In an attempt to prevent another financial crisis, the Fed works to keep the market bubble from popping".to_owned(),
            "DURING THIS ROUND THE BUBBLE CANNOT POP".to_owned(),
        ),
        MarketForce::new(
            "STABLE MARKET".to_owned(),
            "They say no news is good news. The market is performing as planned.".to_owned(),
            "NO EFFECT THIS ROUND".to_owned(),
        ),
        MarketForce::new(
            "HIGH DEMAND".to_owned(),
            "High demand is creating a bidding war, driving security prices up. Add up the total quantity (from Phase 2) of each security being bought.".to_owned(),
            "MOST BOUGHT SECURITY +1".to_owned(),
        ),
        MarketForce::new(
            "NATIONAL DEBT".to_owned(),
            "Increased tax revenue lowers the value of bonds.".to_owned(),
            "BONDS -2".to_owned(),
        ),
        MarketForce::new(
            "CURE ALL".to_owned(),
            "Huge medical breakthroughs have been made in the treatment of deadly diseases.".to_owned(),
            "INSURANCE +2".to_owned(),
        ),
        MarketForce::new(
            "TRADE DEAL".to_owned(),
            "Foreign trade relations are improved by revised trade agreements.".to_owned(),
            "ALL SECURITIES +1".to_owned(),
        ),
        MarketForce::new(
            "NATURAL DISASTER".to_owned(),
            "News of hurricanes and wild fires sends securities down.".to_owned(),
            "ALL SECURITIES -1".to_owned(),
        ),
    ];

    market_forces.shuffle(&mut rng);

    let mut market_forces = VecDeque::from(market_forces);

    let mut players = Vec::new();

    let num_players = num_players.trim().parse::<i32>().unwrap();

    for i in 0..num_players {
        players.push(players_to_choose_from[i as usize].clone());
        println!("Player {}:\n{}", i + 1, players[i as usize]);
    }

    println!();

    let mut current_state = GameState::new(Round::Integer(1), 1, String::new(), 50, 50, 50, 0, 0, 0);

    for _ in 0..5 {
        play_round(&mut current_state, &mut players, &mut market_forces);
    }

    market_close(&mut current_state, &mut players);

    println!();

    print_results(&current_state, &players);
}

fn main() {
    run();
}