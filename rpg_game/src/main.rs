mod character;
mod classes;
mod combat;
mod enemies;
mod npcs;
mod players;

use character::{
    Character, CombatSystem, attack, compare_characters, find_strongest, find_tankiest, heal,
    heal_party, party_total_health,
};
use classes::{Class, ClassType};
use combat::Arena;
use enemies::{DragonBoss, Enemy, GoblinMage, GoblinWarrior};
use npcs::{LegendaryNPC, Merchant, NPC, QuestGiver, Villager};
use players::{Player, PlayerMage, PlayerWarrior};

fn main() {
    println!("=== RPG GAME SYSTEM ===\n");

    // Character creation and class system demonstration
    let mut warrior = PlayerWarrior::new("Aragorn");
    let mut mage = PlayerMage::new("Gandalf");
    let mut goblin1 = GoblinWarrior::new("Azog");
    let mut goblin2 = GoblinMage::new("Saruman's Lieutenant");
    let dragon = DragonBoss::new("Smaug");

    println!("=== CLASS SYSTEM EXPLORATION ===");

    // Demonstrate class type usage and internal class access
    match warrior.get_class_type() {
        ClassType::Warrior(warrior_class) => {
            println!("Warrior Class Stats:");
            println!("  Base Health: {}", warrior_class.get_base_health());
            println!("  Damage Type: {:?}", warrior_class.get_damage_type());
            println!("  Attack Type: {:?}", warrior_class.get_attack_type());
            println!("  Strength: {}", warrior_class.get_strength());
        }
        _ => {}
    }

    match mage.get_class_type() {
        ClassType::Mage(mage_class) => {
            println!("\nMage Class Stats:");
            println!("  Base Health: {}", mage_class.get_base_health());
            println!("  Damage Type: {:?}", mage_class.get_damage_type());
            println!("  Attack Type: {:?}", mage_class.get_attack_type());
            println!("  Mana: {}", mage_class.get_mana());
            println!("  Power: {}", mage_class.get_power());
        }
        _ => {}
    }

    println!("\n=== CLASS TYPE UNIFIED INTERFACE ===");
    let class_types = vec![
        ("Warrior", warrior.get_class_type()),
        ("Mage", mage.get_class_type()),
    ];

    for (name, class) in &class_types {
        println!("{} via ClassType:", name);
        println!("  Base Health: {}", class.get_base_health());
        println!("  Damage Type: {:?}", class.get_damage_type());
        println!("  Attack Type: {:?}", class.get_attack_type());

        if let Some(strength) = class.get_strength() {
            println!("  Strength: {}", strength);
        }
        if let Some(mana) = class.get_mana() {
            println!("  Mana: {}", mana);
        }
        if let Some(power) = class.get_power() {
            println!("  Power: {}", power);
        }
        println!();
    }

    println!("=== ENEMY SPECIAL ABILITIES ===");

    // Goblin warrior special abilities
    println!("Goblin Warrior Stats:");
    println!("  Strength: {}", goblin1.get_strength());
    println!("  Aggro Level: {}", goblin1.get_aggro());
    println!("  Threat Level: {}", goblin1.get_threat_level());

    let berserker_damage = goblin1.berserker_rage();
    println!("  Berserker Rage Bonus: {}", berserker_damage);

    println!("\nGoblin Mage Stats:");
    println!("  Aggro Level: {}", goblin2.get_aggro());
    println!("  Threat Level: {}", goblin2.get_threat_level());

    println!("\n=== NPC INTERACTION SYSTEM ===");

    let merchant = Merchant::new("Barliman Butterbur");
    let quest_giver = QuestGiver::new("Elrond");
    let legendary_npc = LegendaryNPC::new("Tom Bombadil");
    let villager = Villager::new("Farmer Maggot");

    let npcs = vec![
        ("Merchant", &merchant as &dyn NPC),
        ("Quest Giver", &quest_giver as &dyn NPC),
        ("Legendary NPC", &legendary_npc as &dyn NPC),
        ("Villager", &villager as &dyn NPC),
    ];

    for (npc_type, npc) in &npcs {
        println!("\n--- {} ({}) ---", npc.get_name(), npc_type);
        println!("Dialogue: \"{}\"", npc.get_dialogue());
        println!("Importance: {:?}", npc.get_importance());
        println!("Can Trade: {}", npc.can_trade());
        println!("Can Give Quests: {}", npc.can_give_quests());
        println!("{}", npc.interact());
    }

    println!("\n=== MERCHANT TRADING SYSTEM ===");
    let mut active_merchant = Merchant::new("Bree Trader");
    println!(
        "Merchant {} has {} gold",
        active_merchant.get_name(),
        active_merchant.get_gold()
    );

    println!("Attempting to spend 150 gold...");
    if active_merchant.spend_gold(150) {
        println!(
            "Trade successful! Remaining gold: {}",
            active_merchant.get_gold()
        );
    } else {
        println!("Insufficient funds!");
    }

    active_merchant.earn_gold(500);
    println!(
        "Merchant earned 500 gold! Total: {}",
        active_merchant.get_gold()
    );

    println!("\n=== PLAYER PROGRESSION SYSTEM ===");

    println!("Initial Player Stats:");
    println!(
        "  {} - Level: {}, Experience: {}",
        warrior.get_name(),
        warrior.get_level(),
        warrior.get_experience()
    );
    println!(
        "  {} - Level: {}, Experience: {}",
        mage.get_name(),
        mage.get_level(),
        mage.get_experience()
    );

    // Simulate gaining experience
    warrior.add_experience(50);
    mage.add_experience(150); // This should trigger a level up

    println!("\nAfter gaining experience:");
    println!(
        "  {} - Level: {}, Experience: {}",
        warrior.get_name(),
        warrior.get_level(),
        warrior.get_experience()
    );
    println!(
        "  {} - Level: {}, Experience: {}",
        mage.get_name(),
        mage.get_level(),
        mage.get_experience()
    );

    println!("\n=== COMBAT SYSTEM DEMONSTRATIONS ===");

    println!("Basic Attack System:");
    let damage1 = attack(&warrior, &mut goblin1);
    println!(
        "{} attacks {} for {} damage!",
        warrior.get_name(),
        goblin1.get_name(),
        damage1
    );

    let damage2 = attack(&mage, &mut goblin2);
    println!(
        "{} attacks {} for {} damage!",
        mage.get_name(),
        goblin2.get_name(),
        damage2
    );

    println!("\nHealing System:");
    println!(
        "Before healing: {} has {} HP",
        goblin1.get_name(),
        goblin1.get_health()
    );
    heal(&mut goblin1, 20);
    println!(
        "After healing: {} has {} HP",
        goblin1.get_name(),
        goblin1.get_health()
    );

    println!("\nCharacter Comparisons:");
    println!("{}", compare_characters(&warrior, &goblin1));
    println!("{}", compare_characters(&mage, &goblin2));

    println!("\n=== SPECIALIZED COMBAT WITH REWARDS ===");
    let mut test_player = PlayerWarrior::new("Boromir");
    let mut test_enemy = GoblinWarrior::new("Orc Captain");

    println!("Before combat:");
    println!(
        "  Player Level: {}, Experience: {}",
        test_player.get_level(),
        test_player.get_experience()
    );

    Arena::player_vs_enemy(&mut test_player, &mut test_enemy);

    println!("After combat:");
    println!(
        "  Player Level: {}, Experience: {}",
        test_player.get_level(),
        test_player.get_experience()
    );

    println!("\n=== COMBAT TYPE EFFECTIVENESS ===");
    let warrior_damage_vs_mage = combat::calculate_damage_with_bonus(&warrior, &mage);
    let mage_damage_vs_warrior = combat::calculate_damage_with_bonus(&mage, &warrior);

    println!("Type advantage system:");
    println!(
        "  Warrior vs Mage: {} damage (Physical vs Magical)",
        warrior_damage_vs_mage
    );
    println!(
        "  Mage vs Warrior: {} damage (Magical vs Physical)",
        mage_damage_vs_warrior
    );

    println!("\n=== PARTY OPERATIONS ===");

    let mut party = vec![
        PlayerWarrior::new("Gimli"),
        PlayerWarrior::new("Legolas"),
        PlayerWarrior::new("Faramir"),
    ];

    // Damage the party
    for member in &mut party {
        member.set_health(member.get_health() / 2);
    }

    println!(
        "Party total health before healing: {}",
        party_total_health(&party)
    );
    heal_party(&mut party, 25);
    println!(
        "Party total health after healing: {}",
        party_total_health(&party)
    );

    println!("\n=== FINDING STRONGEST AND TANKIEST ===");

    let warrior_enemies = vec![
        GoblinWarrior::new("Grunt Orc"),
        GoblinWarrior::new("Uruk-hai"),
        GoblinWarrior::new("Mordor Elite"),
    ];

    if let Some(strongest_warrior) = find_strongest(&warrior_enemies) {
        println!(
            "Strongest warrior enemy: {} with {} damage",
            strongest_warrior.get_name(),
            strongest_warrior.get_calculated_damage()
        );
    }

    if let Some(tankiest_warrior) = find_tankiest(&warrior_enemies) {
        println!(
            "Tankiest warrior enemy: {} with {} HP",
            tankiest_warrior.get_name(),
            tankiest_warrior.get_calculated_max_health()
        );
    }

    let mage_enemies = vec![
        GoblinMage::new("Witch-king"),
        GoblinMage::new("Mouth of Sauron"),
    ];

    if let Some(strongest_mage) = find_strongest(&mage_enemies) {
        println!(
            "Strongest mage enemy: {} with {} damage",
            strongest_mage.get_name(),
            strongest_mage.get_calculated_damage()
        );
    }

    println!("\n=== TOURNAMENT SYSTEM ===");

    let tournament_fighters = vec![
        GoblinWarrior::new("Gothmog"),
        GoblinWarrior::new("Bolg"),
        GoblinWarrior::new("Lurtz"),
        GoblinWarrior::new("Grishnakh"),
    ];

    if let Some(winner) = Arena::tournament(tournament_fighters) {
        println!("Tournament victor: {}!", winner.get_name());
    }

    let mage_tournament = vec![
        GoblinMage::new("Radagast"),
        GoblinMage::new("Saruman"),
        GoblinMage::new("Necromancer"),
    ];

    if let Some(mage_winner) = Arena::tournament(mage_tournament) {
        println!("Mage tournament champion: {}!", mage_winner.get_name());
    }

    println!("\n=== COMBAT SYSTEM BATTLES ===");

    let mut test_warrior1 = PlayerWarrior::new("Eomer");
    let mut test_warrior2 = PlayerWarrior::new("Theoden");
    let result1 = CombatSystem::battle(&mut test_warrior1, &mut test_warrior2);
    println!("Warrior vs Warrior: {:?}", result1);

    let mut combat_merchant = Merchant::new("Dale Merchant");
    let mut fresh_warrior = PlayerWarrior::new("Denethor");
    let result2 = CombatSystem::battle(&mut fresh_warrior, &mut combat_merchant);
    println!("Warrior vs Merchant: {:?}", result2);

    println!("\n=== FINAL BOSS BATTLE ===");
    let mut final_hero = PlayerWarrior::new("Frodo");
    let mut final_boss = DragonBoss::new("Balrog");

    // Give the hero some experience to make it interesting
    final_hero.add_experience(250);

    println!("Epic showdown begins!");
    println!(
        "Hero: Level {}, {} HP, {} Damage",
        final_hero.get_level(),
        final_hero.get_calculated_max_health(),
        final_hero.get_calculated_damage()
    );
    println!(
        "Boss: Threat {}, {} HP, {} Damage, Invincible: {}",
        final_boss.get_threat_level(),
        final_boss.get_calculated_max_health(),
        final_boss.get_calculated_damage(),
        final_boss.is_invincible()
    );

    Arena::player_vs_enemy(&mut final_hero, &mut final_boss);

    println!("\n=== ADDITIONAL FEATURES ===");

    println!("Mixed character analysis:");

    fn print_character_info<T: Character>(character: &T) {
        println!(
            "{}: {} HP, {} damage, {:?} type",
            character.get_name(),
            character.get_calculated_max_health(),
            character.get_calculated_damage(),
            character.get_damage_type()
        );
    }

    print_character_info(&warrior);
    print_character_info(&dragon);
    print_character_info(&merchant);

    println!("\nEnemy vs Enemy Combat:");
    let mut orc1 = GoblinWarrior::new("Shagrat");
    let mut orc2 = GoblinMage::new("Gorbag");
    Arena::enemy_vs_enemy(&mut orc1, &mut orc2);
}
