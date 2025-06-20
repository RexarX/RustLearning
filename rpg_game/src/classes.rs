#[derive(Debug, Clone)]
pub enum DamageType {
    Physical,
    Magical,
}

#[derive(Debug, Clone)]
pub enum AttackType {
    Melee,
    Ranged,
}

pub trait Class {
    fn get_base_health(&self) -> u32;
    fn get_damage_type(&self) -> DamageType;
    fn get_attack_type(&self) -> AttackType;
}

#[derive(Debug, Clone)]
pub struct Warrior {
    strength: u32,
}

impl Warrior {
    pub fn new() -> Self {
        Warrior { strength: 10 }
    }

    pub fn get_strength(&self) -> u32 {
        self.strength
    }
}

impl Class for Warrior {
    fn get_base_health(&self) -> u32 {
        100
    }

    fn get_damage_type(&self) -> DamageType {
        DamageType::Physical
    }

    fn get_attack_type(&self) -> AttackType {
        AttackType::Melee
    }
}

#[derive(Debug, Clone)]
pub struct Mage {
    mana: u32,
    power: u32,
}

impl Mage {
    pub fn new() -> Self {
        Mage {
            mana: 100,
            power: 10,
        }
    }

    pub fn get_mana(&self) -> u32 {
        self.mana
    }

    pub fn get_power(&self) -> u32 {
        self.power
    }
}

impl Class for Mage {
    fn get_base_health(&self) -> u32 {
        75
    }

    fn get_damage_type(&self) -> DamageType {
        DamageType::Magical
    }

    fn get_attack_type(&self) -> AttackType {
        AttackType::Ranged
    }
}

#[derive(Debug, Clone)]
pub enum ClassType {
    Warrior(Warrior),
    Mage(Mage),
}

impl ClassType {
    pub fn get_base_health(&self) -> u32 {
        match self {
            ClassType::Warrior(warrior) => warrior.get_base_health(),
            ClassType::Mage(mage) => mage.get_base_health(),
        }
    }

    pub fn get_damage_type(&self) -> DamageType {
        match self {
            ClassType::Warrior(warrior) => warrior.get_damage_type(),
            ClassType::Mage(mage) => mage.get_damage_type(),
        }
    }

    pub fn get_attack_type(&self) -> AttackType {
        match self {
            ClassType::Warrior(warrior) => warrior.get_attack_type(),
            ClassType::Mage(mage) => mage.get_attack_type(),
        }
    }

    pub fn get_strength(&self) -> Option<u32> {
        match self {
            ClassType::Warrior(warrior) => Some(warrior.get_strength()),
            ClassType::Mage(_) => None,
        }
    }

    pub fn get_mana(&self) -> Option<u32> {
        match self {
            ClassType::Warrior(_) => None,
            ClassType::Mage(mage) => Some(mage.get_mana()),
        }
    }

    pub fn get_power(&self) -> Option<u32> {
        match self {
            ClassType::Warrior(_) => None,
            ClassType::Mage(mage) => Some(mage.get_power()),
        }
    }
}
