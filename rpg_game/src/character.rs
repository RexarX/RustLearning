use crate::classes::{AttackType, ClassType, DamageType};

#[derive(Debug, Clone)]
pub enum CharacterType {
    Player { level: u32 },
    Enemy { threat_level: u32 },
    NPC { importance: NPCImportance },
}

#[derive(Debug, Clone)]
pub enum NPCImportance {
    Minor,     // 0.5x coefficient
    Normal,    // 1.0x coefficient
    Important, // 1.5x coefficient
    Legendary, // 2.0x coefficient
}

impl CharacterType {
    pub fn get_health_coefficient(&self) -> f32 {
        match self {
            CharacterType::Player { level } => {
                1.0 + (*level as f32 - 1.0) * 0.1 // +10% per level above 1
            }
            CharacterType::Enemy { threat_level } => {
                match threat_level {
                    1..=3 => 0.8, // Weak enemies
                    4..=6 => 1.0, // Normal enemies
                    7..=9 => 1.3, // Strong enemies
                    10.. => 1.8,  // Boss enemies
                    _ => 0.5,     // Very weak
                }
            }
            CharacterType::NPC { importance } => match importance {
                NPCImportance::Minor => 0.5,
                NPCImportance::Normal => 1.0,
                NPCImportance::Important => 1.5,
                NPCImportance::Legendary => 2.0,
            },
        }
    }

    pub fn get_damage_coefficient(&self) -> f32 {
        match self {
            CharacterType::Player { level } => {
                1.0 + (*level as f32 - 1.0) * 0.15 // +15% damage per level
            }
            CharacterType::Enemy { threat_level } => 1.0 + (*threat_level as f32 - 1.0) * 0.1,
            CharacterType::NPC { importance } => match importance {
                NPCImportance::Minor => 0.3,
                NPCImportance::Normal => 0.7,
                NPCImportance::Important => 1.2,
                NPCImportance::Legendary => 2.5,
            },
        }
    }
}

pub trait Character {
    fn new(name: &str) -> Self
    where
        Self: Sized;

    fn revive(&mut self);
    fn get_name(&self) -> &str;
    fn get_health(&self) -> u32;
    fn set_health(&mut self, health: u32);
    fn is_invincible(&self) -> bool {
        false
    }

    fn get_class_type(&self) -> &ClassType;
    fn get_character_type(&self) -> &CharacterType;
    fn get_base_health(&self) -> u32;
    fn get_calculated_max_health(&self) -> u32 {
        let base = self.get_base_health() as f32;
        let coefficient = self.get_character_type().get_health_coefficient();
        (base * coefficient) as u32
    }

    fn get_damage_type(&self) -> DamageType;
    fn get_attack_type(&self) -> AttackType;
    fn get_base_damage(&self) -> u32;
    fn get_calculated_damage(&self) -> u32 {
        let base = self.get_base_damage() as f32;
        let coefficient = self.get_character_type().get_damage_coefficient();
        (base * coefficient) as u32
    }

    fn can_attack(&self) -> bool {
        self.get_health() > 0 && !self.is_invincible()
    }

    fn is_alive(&self) -> bool {
        self.get_health() > 0
    }
}

pub fn attack<A, D>(attacker: &A, defender: &mut D) -> u32
where
    A: Character,
    D: Character,
{
    if !attacker.can_attack() {
        return 0;
    }

    let damage = attacker.get_calculated_damage();
    if defender.get_health() > damage {
        defender.set_health(defender.get_health() - damage);
    } else {
        defender.set_health(0);
    }
    damage
}

pub struct CombatSystem;

impl CombatSystem {
    pub fn battle<T1, T2>(fighter1: &mut T1, fighter2: &mut T2) -> BattleResult
    where
        T1: Character,
        T2: Character,
    {
        let mut rounds = 0;
        let max_rounds = 100; // Prevent infinite loops

        while fighter1.is_alive() && fighter2.is_alive() && rounds < max_rounds {
            rounds += 1;

            // Fighter 1 attacks Fighter 2
            if fighter1.can_attack() {
                let damage = attack(fighter1, fighter2);
                println!(
                    "Round {}: {} attacks {} for {} damage! ({} HP remaining)",
                    rounds,
                    fighter1.get_name(),
                    fighter2.get_name(),
                    damage,
                    fighter2.get_health()
                );
            }

            if !fighter2.is_alive() {
                return BattleResult::Winner1;
            }

            // Fighter 2 attacks Fighter 1
            if fighter2.can_attack() {
                let damage = attack(fighter2, fighter1);
                println!(
                    "Round {}: {} attacks {} for {} damage! ({} HP remaining)",
                    rounds,
                    fighter2.get_name(),
                    fighter1.get_name(),
                    damage,
                    fighter1.get_health()
                );
            }

            if !fighter1.is_alive() {
                return BattleResult::Winner2;
            }
        }

        if rounds >= max_rounds {
            BattleResult::Draw
        } else if fighter1.get_health() > fighter2.get_health() {
            BattleResult::Winner1
        } else if fighter2.get_health() > fighter1.get_health() {
            BattleResult::Winner2
        } else {
            BattleResult::Draw
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BattleResult {
    Winner1,
    Winner2,
    Draw,
}

pub fn heal<T: Character>(character: &mut T, amount: u32) {
    let new_health = character.get_health() + amount;
    let max_health = character.get_calculated_max_health();
    character.set_health(new_health.min(max_health));
}

pub fn compare_characters<T1, T2>(char1: &T1, char2: &T2) -> String
where
    T1: Character,
    T2: Character,
{
    format!(
        "{} vs {}: HP ({} vs {}), Damage ({} vs {}), Type ({:?} vs {:?})",
        char1.get_name(),
        char2.get_name(),
        char1.get_calculated_max_health(),
        char2.get_calculated_max_health(),
        char1.get_calculated_damage(),
        char2.get_calculated_damage(),
        char1.get_damage_type(),
        char2.get_damage_type()
    )
}

pub fn heal_party<T: Character>(party: &mut [T], amount: u32) {
    for character in party.iter_mut() {
        heal(character, amount);
        println!(
            "{} healed for {} HP! Current HP: {}",
            character.get_name(),
            amount,
            character.get_health()
        );
    }
}

pub fn party_total_health<T: Character>(party: &[T]) -> u32 {
    party.iter().map(|c| c.get_health()).sum()
}

pub fn find_strongest<'a, T: Character>(characters: &'a [T]) -> Option<&'a T> {
    characters.iter().max_by_key(|c| c.get_calculated_damage())
}

pub fn find_tankiest<'a, T: Character>(characters: &'a [T]) -> Option<&'a T> {
    characters
        .iter()
        .max_by_key(|c| c.get_calculated_max_health())
}
