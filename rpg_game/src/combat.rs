use crate::character::{BattleResult, Character, CombatSystem};
use crate::enemies::Enemy;
use crate::players::Player;

pub struct Arena;

impl Arena {
    pub fn player_vs_enemy<P, E>(player: &mut P, enemy: &mut E) -> BattleResult
    where
        P: Player + Character,
        E: Enemy + Character,
    {
        println!("=== ARENA COMBAT ===");
        println!(
            "{} (Player, Level {}) vs {} (Enemy, Threat {})",
            player.get_name(),
            player.get_level(),
            enemy.get_name(),
            enemy.get_threat_level()
        );

        let result = CombatSystem::battle(player, enemy);

        match result {
            BattleResult::Winner1 => {
                let exp_gained = enemy.get_threat_level() * 25;
                player.add_experience(exp_gained);
                println!(
                    "{} wins and gains {} experience!",
                    player.get_name(),
                    exp_gained
                );
            }
            BattleResult::Winner2 => {
                println!(
                    "{} has been defeated by {}!",
                    player.get_name(),
                    enemy.get_name()
                );
            }
            BattleResult::Draw => {
                println!("The battle ends in a draw!");
            }
        }

        result
    }

    pub fn enemy_vs_enemy<E1, E2>(enemy1: &mut E1, enemy2: &mut E2) -> BattleResult
    where
        E1: Enemy + Character,
        E2: Enemy + Character,
    {
        println!("=== MONSTER BATTLE ===");
        println!(
            "{} (Threat {}) vs {} (Threat {})",
            enemy1.get_name(),
            enemy1.get_threat_level(),
            enemy2.get_name(),
            enemy2.get_threat_level()
        );

        CombatSystem::battle(enemy1, enemy2)
    }

    pub fn tournament<T>(mut participants: Vec<T>) -> Option<T>
    where
        T: Character + Clone,
    {
        if participants.is_empty() {
            return None;
        }

        if participants.len() == 1 {
            return participants.into_iter().next();
        }

        println!("=== TOURNAMENT START ===");
        println!("Participants: {}", participants.len());

        let mut round = 1;

        while participants.len() > 1 {
            println!("\n--- Round {} ---", round);
            let mut next_round = Vec::new();

            // Pair up fighters
            for chunk in participants.chunks_mut(2) {
                if chunk.len() == 2 {
                    let mut fighter1 = chunk[0].clone();
                    let mut fighter2 = chunk[1].clone();

                    let result = CombatSystem::battle(&mut fighter1, &mut fighter2);

                    match result {
                        BattleResult::Winner1 => {
                            println!("{} advances to the next round!", fighter1.get_name());
                            fighter1.revive(); // Heal for next round
                            next_round.push(fighter1);
                        }
                        BattleResult::Winner2 => {
                            println!("{} advances to the next round!", fighter2.get_name());
                            fighter2.revive(); // Heal for next round
                            next_round.push(fighter2);
                        }
                        BattleResult::Draw => {
                            // In case of draw, the one with higher health advances
                            if fighter1.get_health() >= fighter2.get_health() {
                                fighter1.revive();
                                next_round.push(fighter1);
                            } else {
                                fighter2.revive();
                                next_round.push(fighter2);
                            }
                        }
                    }
                } else {
                    // Odd number, last fighter gets a bye
                    println!("{} gets a bye to the next round!", chunk[0].get_name());
                    let mut bye_fighter = chunk[0].clone();
                    bye_fighter.revive();
                    next_round.push(bye_fighter);
                }
            }

            participants = next_round;
            round += 1;
        }

        participants.into_iter().next()
    }
}

pub fn calculate_damage_with_bonus<A, D>(attacker: &A, defender: &D) -> u32
where
    A: Character,
    D: Character,
{
    let base_damage = attacker.get_calculated_damage();

    // Type advantage system
    let type_multiplier = match (attacker.get_damage_type(), defender.get_damage_type()) {
        (crate::classes::DamageType::Physical, crate::classes::DamageType::Magical) => 1.2, // Physical vs Magical
        (crate::classes::DamageType::Magical, crate::classes::DamageType::Physical) => 0.8, // Magical vs Physical
        _ => 1.0, // Same types
    };

    // Attack type vs defense consideration
    let attack_bonus = match (attacker.get_attack_type(), defender.get_attack_type()) {
        (crate::classes::AttackType::Ranged, crate::classes::AttackType::Melee) => 1.15, // Ranged vs Melee
        (crate::classes::AttackType::Melee, crate::classes::AttackType::Ranged) => 0.9, // Melee vs Ranged
        _ => 1.0,
    };

    (base_damage as f32 * type_multiplier * attack_bonus) as u32
}
