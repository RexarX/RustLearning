use crate::character::{Character, CharacterType};
use crate::classes::{
    AttackType, ClassType, DamageType, Mage as MageClass, Warrior as WarriorClass,
};

pub trait Player: Character {
    fn level_up(&mut self);
    fn get_experience(&self) -> u32;
    fn get_level(&self) -> u32;
    fn add_experience(&mut self, exp: u32);
}

#[derive(Debug, Clone)]
pub struct PlayerWarrior {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub experience: u32,
    pub level: u32,
    pub class: ClassType,
    pub character_type: CharacterType,
}

impl Character for PlayerWarrior {
    fn new(name: &str) -> Self {
        let level = 1;
        let character_type = CharacterType::Player { level };
        let class = ClassType::Warrior(WarriorClass::new());
        let base_health = 100;
        let max_health = (base_health as f32 * character_type.get_health_coefficient()) as u32;

        PlayerWarrior {
            name: name.to_string(),
            health: max_health,
            max_health,
            experience: 0,
            level,
            class,
            character_type,
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
        100
    }

    fn get_damage_type(&self) -> DamageType {
        DamageType::Physical
    }

    fn get_attack_type(&self) -> AttackType {
        AttackType::Melee
    }

    fn get_base_damage(&self) -> u32 {
        25
    }
}

impl Player for PlayerWarrior {
    fn level_up(&mut self) {
        self.level += 1;
        self.character_type = CharacterType::Player { level: self.level };
        let new_max_health = self.get_calculated_max_health();
        self.max_health = new_max_health;
        self.health = new_max_health; // Full heal on level up
    }

    fn get_experience(&self) -> u32 {
        self.experience
    }

    fn get_level(&self) -> u32 {
        self.level
    }

    fn add_experience(&mut self, exp: u32) {
        self.experience += exp;
        let exp_needed = self.level * 100; // 100 exp per level
        if self.experience >= exp_needed {
            self.experience -= exp_needed;
            self.level_up();
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerMage {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub mana: u32,
    pub max_mana: u32,
    pub experience: u32,
    pub level: u32,
    pub class: ClassType,
    pub character_type: CharacterType,
}

impl Character for PlayerMage {
    fn new(name: &str) -> Self {
        let level = 1;
        let character_type = CharacterType::Player { level };
        let class = ClassType::Mage(MageClass::new());
        let base_health = 75;
        let max_health = (base_health as f32 * character_type.get_health_coefficient()) as u32;

        PlayerMage {
            name: name.to_string(),
            health: max_health,
            max_health,
            mana: 100,
            max_mana: 100,
            experience: 0,
            level,
            class,
            character_type,
        }
    }

    fn revive(&mut self) {
        if self.health == 0 {
            self.health = self.get_calculated_max_health();
            self.mana = self.max_mana;
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
        75
    }

    fn get_damage_type(&self) -> DamageType {
        DamageType::Magical
    }

    fn get_attack_type(&self) -> AttackType {
        AttackType::Ranged
    }

    fn get_base_damage(&self) -> u32 {
        30
    }
}

impl Player for PlayerMage {
    fn level_up(&mut self) {
        self.level += 1;
        self.character_type = CharacterType::Player { level: self.level };
        let new_max_health = self.get_calculated_max_health();
        self.max_health = new_max_health;
        self.health = new_max_health;
        self.max_mana += 10; // Increase mana with level
        self.mana = self.max_mana;
    }

    fn get_experience(&self) -> u32 {
        self.experience
    }

    fn get_level(&self) -> u32 {
        self.level
    }

    fn add_experience(&mut self, exp: u32) {
        self.experience += exp;
        let exp_needed = self.level * 100;
        if self.experience >= exp_needed {
            self.experience -= exp_needed;
            self.level_up();
        }
    }
}
