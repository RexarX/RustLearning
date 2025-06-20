use crate::character::{Character, CharacterType};
use crate::classes::{AttackType, ClassType, DamageType, Mage, Warrior};

pub trait Enemy: Character {
    fn get_aggro(&self) -> u32;
    fn get_threat_level(&self) -> u32;
}

#[derive(Debug, Clone)]
pub struct GoblinWarrior {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub strength: u32,
    pub class: ClassType,
    pub character_type: CharacterType,
}

impl Character for GoblinWarrior {
    fn new(name: &str) -> Self {
        let character_type = CharacterType::Enemy { threat_level: 3 };
        let class = ClassType::Warrior(Warrior::new());
        let base_health = 50;
        let max_health = (base_health as f32 * character_type.get_health_coefficient()) as u32;

        GoblinWarrior {
            name: name.to_string(),
            health: max_health,
            max_health,
            strength: 5,
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
        50
    }

    fn get_damage_type(&self) -> DamageType {
        DamageType::Physical
    }

    fn get_attack_type(&self) -> AttackType {
        AttackType::Melee
    }

    fn get_base_damage(&self) -> u32 {
        15
    }
}

impl Enemy for GoblinWarrior {
    fn get_aggro(&self) -> u32 {
        75
    }

    fn get_threat_level(&self) -> u32 {
        if let CharacterType::Enemy { threat_level } = &self.character_type {
            *threat_level
        } else {
            1
        }
    }
}

impl GoblinWarrior {
    pub fn get_strength(&self) -> u32 {
        self.strength
    }

    pub fn berserker_rage(&mut self) -> u32 {
        // Use strength for special ability
        let bonus_damage = self.strength * 2;
        println!(
            "{} enters berserker rage! Bonus damage: {}",
            self.name, bonus_damage
        );
        bonus_damage
    }
}

#[derive(Debug, Clone)]
pub struct GoblinMage {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub power: u32,
    pub mana: u32,
    pub class: ClassType,
    pub character_type: CharacterType,
}

impl Character for GoblinMage {
    fn new(name: &str) -> Self {
        let character_type = CharacterType::Enemy { threat_level: 4 };
        let class = ClassType::Mage(Mage::new());
        let base_health = 25;
        let max_health = (base_health as f32 * character_type.get_health_coefficient()) as u32;

        GoblinMage {
            name: name.to_string(),
            health: max_health,
            max_health,
            power: 5,
            mana: 50,
            class,
            character_type,
        }
    }

    fn revive(&mut self) {
        if self.health == 0 {
            self.health = self.get_calculated_max_health();
            self.mana = 50;
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
        25
    }

    fn get_damage_type(&self) -> DamageType {
        DamageType::Magical
    }

    fn get_attack_type(&self) -> AttackType {
        AttackType::Ranged
    }

    fn get_base_damage(&self) -> u32 {
        20
    }

    fn can_attack(&self) -> bool {
        self.get_health() > 0 && self.mana >= 10
    }
}

impl Enemy for GoblinMage {
    fn get_aggro(&self) -> u32 {
        25
    }

    fn get_threat_level(&self) -> u32 {
        if let CharacterType::Enemy { threat_level } = &self.character_type {
            *threat_level
        } else {
            1
        }
    }
}

#[derive(Debug, Clone)]
pub struct DragonBoss {
    pub name: String,
    pub health: u32,
    pub max_health: u32,
    pub class: ClassType,
    pub character_type: CharacterType,
}

impl Character for DragonBoss {
    fn new(name: &str) -> Self {
        let character_type = CharacterType::Enemy { threat_level: 15 };
        let class = ClassType::Warrior(Warrior::new());
        let base_health = 200;
        let max_health = (base_health as f32 * character_type.get_health_coefficient()) as u32;

        DragonBoss {
            name: name.to_string(),
            health: max_health,
            max_health,
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
        200
    }

    fn get_damage_type(&self) -> DamageType {
        DamageType::Physical
    }

    fn get_attack_type(&self) -> AttackType {
        AttackType::Melee
    }

    fn get_base_damage(&self) -> u32 {
        50
    }

    fn is_invincible(&self) -> bool {
        self.health > self.max_health / 2
    }
}

impl Enemy for DragonBoss {
    fn get_aggro(&self) -> u32 {
        100
    }

    fn get_threat_level(&self) -> u32 {
        if let CharacterType::Enemy { threat_level } = &self.character_type {
            *threat_level
        } else {
            10
        }
    }
}
