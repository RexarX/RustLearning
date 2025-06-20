use crate::character::{Character, CharacterType, NPCImportance};
use crate::classes::{AttackType, ClassType, DamageType, Mage, Warrior};

pub trait NPC: Character {
    fn get_dialogue(&self) -> &str;
    fn get_importance(&self) -> &NPCImportance;
    fn can_trade(&self) -> bool {
        false
    }
    fn can_give_quests(&self) -> bool {
        false
    }
    fn interact(&self) -> String {
        format!("{} says: \"{}\"", self.get_name(), self.get_dialogue())
    }
}

#[derive(Debug, Clone)]
pub struct Merchant {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub class: ClassType,
    pub character_type: CharacterType,
    pub dialogue: String,
    pub gold: u32,
}

impl Character for Merchant {
    fn new(name: &str) -> Self {
        let character_type = CharacterType::NPC {
            importance: NPCImportance::Normal,
        };
        let class = ClassType::Warrior(Warrior::new());
        let base_health = 60;
        let max_health = (base_health as f32 * character_type.get_health_coefficient()) as u32;

        Merchant {
            name: name.to_string(),
            health: max_health,
            max_health,
            class,
            character_type,
            dialogue: "Welcome to my shop! What can I get for you?".to_string(),
            gold: 1000,
        }
    }

    fn revive(&mut self) {
        if self.health == 0 {
            self.health = self.get_calculated_max_health();
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_health(&self) -> u32 {
        self.health
    }

    fn set_health(&mut self, health: u32) {
        self.health = health.min(self.max_health);
    }

    fn get_class_type(&self) -> &ClassType {
        &self.class
    }

    fn get_character_type(&self) -> &CharacterType {
        &self.character_type
    }

    fn get_base_health(&self) -> u32 {
        60
    }

    fn get_damage_type(&self) -> DamageType {
        DamageType::Physical
    }

    fn get_attack_type(&self) -> AttackType {
        AttackType::Melee
    }

    fn get_base_damage(&self) -> u32 {
        10
    }

    fn can_attack(&self) -> bool {
        false // Merchants typically don't attack
    }
}

impl NPC for Merchant {
    fn get_dialogue(&self) -> &str {
        &self.dialogue
    }

    fn get_importance(&self) -> &NPCImportance {
        if let CharacterType::NPC { importance } = &self.character_type {
            importance
        } else {
            &NPCImportance::Normal
        }
    }

    fn can_trade(&self) -> bool {
        true
    }
}

impl Merchant {
    pub fn get_gold(&self) -> u32 {
        self.gold
    }

    pub fn spend_gold(&mut self, amount: u32) -> bool {
        if self.gold >= amount {
            self.gold -= amount;
            true
        } else {
            false
        }
    }

    pub fn earn_gold(&mut self, amount: u32) {
        self.gold += amount;
    }
}

#[derive(Debug, Clone)]
pub struct QuestGiver {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub class: ClassType,
    pub character_type: CharacterType,
    pub dialogue: String,
    pub available_quests: u32,
}

impl Character for QuestGiver {
    fn new(name: &str) -> Self {
        let character_type = CharacterType::NPC {
            importance: NPCImportance::Important,
        };
        let class = ClassType::Mage(Mage::new());
        let base_health = 80;
        let max_health = (base_health as f32 * character_type.get_health_coefficient()) as u32;

        QuestGiver {
            name: name.to_string(),
            health: max_health,
            max_health,
            class,
            character_type,
            dialogue: "I have important tasks for brave adventurers!".to_string(),
            available_quests: 3,
        }
    }

    fn revive(&mut self) {
        if self.health == 0 {
            self.health = self.get_calculated_max_health();
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_health(&self) -> u32 {
        self.health
    }

    fn set_health(&mut self, health: u32) {
        self.health = health.min(self.max_health);
    }

    fn get_class_type(&self) -> &ClassType {
        &self.class
    }

    fn get_character_type(&self) -> &CharacterType {
        &self.character_type
    }

    fn get_base_health(&self) -> u32 {
        80
    }

    fn get_damage_type(&self) -> DamageType {
        DamageType::Magical
    }

    fn get_attack_type(&self) -> AttackType {
        AttackType::Ranged
    }

    fn get_base_damage(&self) -> u32 {
        25
    }

    fn can_attack(&self) -> bool {
        false // Quest givers typically don't attack unless threatened
    }
}

impl NPC for QuestGiver {
    fn get_dialogue(&self) -> &str {
        &self.dialogue
    }

    fn get_importance(&self) -> &NPCImportance {
        if let CharacterType::NPC { importance } = &self.character_type {
            importance
        } else {
            &NPCImportance::Important
        }
    }

    fn can_give_quests(&self) -> bool {
        self.available_quests > 0
    }
}

#[derive(Debug, Clone)]
pub struct LegendaryNPC {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub class: ClassType,
    pub character_type: CharacterType,
    pub dialogue: String,
}

impl Character for LegendaryNPC {
    fn new(name: &str) -> Self {
        let character_type = CharacterType::NPC {
            importance: NPCImportance::Legendary,
        };
        let class = ClassType::Mage(Mage::new());
        let base_health = 150;
        let max_health = (base_health as f32 * character_type.get_health_coefficient()) as u32;

        LegendaryNPC {
            name: name.to_string(),
            health: max_health,
            max_health,
            class,
            character_type,
            dialogue: "The winds of fate have brought you to me, young one...".to_string(),
        }
    }

    fn revive(&mut self) {
        if self.health == 0 {
            self.health = self.get_calculated_max_health();
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_health(&self) -> u32 {
        self.health
    }

    fn set_health(&mut self, health: u32) {
        self.health = health.min(self.max_health);
    }

    fn get_class_type(&self) -> &ClassType {
        &self.class
    }

    fn get_character_type(&self) -> &CharacterType {
        &self.character_type
    }

    fn get_base_health(&self) -> u32 {
        150
    }

    fn get_damage_type(&self) -> DamageType {
        DamageType::Magical
    }

    fn get_attack_type(&self) -> AttackType {
        AttackType::Ranged
    }

    fn get_base_damage(&self) -> u32 {
        75
    }

    fn is_invincible(&self) -> bool {
        true
    }

    fn can_attack(&self) -> bool {
        true // Legendary NPCs can defend themselves
    }
}

impl NPC for LegendaryNPC {
    fn get_dialogue(&self) -> &str {
        &self.dialogue
    }

    fn get_importance(&self) -> &NPCImportance {
        if let CharacterType::NPC { importance } = &self.character_type {
            importance
        } else {
            &NPCImportance::Legendary
        }
    }

    fn can_trade(&self) -> bool {
        true
    }

    fn can_give_quests(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct Villager {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub class: ClassType,
    pub character_type: CharacterType,
    pub dialogue: String,
}

impl Character for Villager {
    fn new(name: &str) -> Self {
        let character_type = CharacterType::NPC {
            importance: NPCImportance::Minor,
        };
        let class = ClassType::Warrior(Warrior::new());
        let base_health = 30;
        let max_health = (base_health as f32 * character_type.get_health_coefficient()) as u32;

        Villager {
            name: name.to_string(),
            health: max_health,
            max_health,
            class,
            character_type,
            dialogue: "Hello there, traveler!".to_string(),
        }
    }

    fn revive(&mut self) {
        if self.health == 0 {
            self.health = self.get_calculated_max_health();
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_health(&self) -> u32 {
        self.health
    }

    fn set_health(&mut self, health: u32) {
        self.health = health.min(self.max_health);
    }

    fn get_class_type(&self) -> &ClassType {
        &self.class
    }

    fn get_character_type(&self) -> &CharacterType {
        &self.character_type
    }

    fn get_base_health(&self) -> u32 {
        30
    }

    fn get_damage_type(&self) -> DamageType {
        DamageType::Physical
    }

    fn get_attack_type(&self) -> AttackType {
        AttackType::Melee
    }

    fn get_base_damage(&self) -> u32 {
        5
    }

    fn can_attack(&self) -> bool {
        false // Villagers are peaceful
    }
}

impl NPC for Villager {
    fn get_dialogue(&self) -> &str {
        &self.dialogue
    }

    fn get_importance(&self) -> &NPCImportance {
        if let CharacterType::NPC { importance } = &self.character_type {
            importance
        } else {
            &NPCImportance::Minor
        }
    }
}
