use crate::helpers::{get_lines, parse};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Item {
    cost: i64,
    dmg: i64,
    arm: i64,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Outcome {
    Lost(i64),
    Win(i64),
}

impl Item {
    pub fn new(cost: i64, dmg: i64, arm: i64) -> Self {
        Self { cost, dmg, arm }
    }
    pub fn from(nums: Vec<i64>) -> Self {
        Self {
            cost: nums[0],
            dmg: nums[1],
            arm: nums[2],
        }
    }
}

fn win(boss_hp: i64, boss_dmg: i64, boss_arm: i64, dmg: i64, armor: i64) -> bool {
    let mut boss_hp = boss_hp;
    let mut player_hp = 100;
    loop {
        boss_hp -= if dmg > boss_arm { dmg - boss_arm } else { 1 };
        if boss_hp <= 0 {
            return true;
        }
        player_hp -= if boss_dmg > armor {
            boss_dmg - armor
        } else {
            1
        };
        if player_hp <= 0 {
            return false;
        }
    }
}

fn test_items(
    boss_hp: i64,
    boss_dmg: i64,
    boss_arm: i64,
    weapon: &Item,
    armor: Option<&Item>,
    ring1: Option<&Item>,
    ring2: Option<&Item>,
) -> Outcome {
    let mut cost = weapon.cost;
    let mut dmg = weapon.dmg;
    let mut arm = weapon.arm;
    if let Some(armor) = armor {
        cost += armor.cost;
        dmg += armor.dmg;
        arm += armor.arm;
    }
    if let Some(ring1) = ring1 {
        cost += ring1.cost;
        dmg += ring1.dmg;
        arm += ring1.arm;
    }
    if let Some(ring2) = ring2 {
        cost += ring2.cost;
        dmg += ring2.dmg;
        arm += ring2.arm;
    }
    if win(boss_hp, boss_dmg, boss_arm, dmg, arm) {
        Outcome::Win(cost)
    } else {
        Outcome::Lost(cost)
    }
}

pub fn solve() {
    println!("Day 21 of 2015");
    let lines = get_lines();
    let weapons = vec![
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0),
    ];
    let armors = vec![
        Item::new(13, 0, 1),
        Item::new(31, 0, 2),
        Item::new(53, 0, 3),
        Item::new(75, 0, 4),
        Item::new(102, 0, 5),
    ];
    let rings = vec![
        Item::new(25, 1, 0),
        Item::new(50, 2, 0),
        Item::new(100, 3, 0),
        Item::new(20, 0, 1),
        Item::new(40, 0, 2),
        Item::new(80, 0, 3),
    ];
    let boss_hp = parse(&lines[0]).1[0];
    let boss_dmg = parse(&lines[1]).1[0];
    let boss_arm = parse(&lines[2]).1[0];
    let mut s1 = i64::MAX;
    let mut s2 = 0;
    for weapon in &weapons {
        for armor in &armors {
            match test_items(boss_hp, boss_dmg, boss_arm, weapon, Some(armor), None, None) {
                Outcome::Win(cost) => s1 = s1.min(cost),
                Outcome::Lost(cost) => s2 = s2.max(cost),
            }
            for ring1 in &rings {
                match test_items(
                    boss_hp,
                    boss_dmg,
                    boss_arm,
                    weapon,
                    Some(armor),
                    Some(ring1),
                    None,
                ) {
                    Outcome::Win(cost) => s1 = s1.min(cost),
                    Outcome::Lost(cost) => s2 = s2.max(cost),
                }
                for ring2 in &rings {
                    if ring1 == ring2 {
                        continue;
                    }
                    match test_items(
                        boss_hp,
                        boss_dmg,
                        boss_arm,
                        weapon,
                        Some(armor),
                        Some(ring1),
                        Some(ring2),
                    ) {
                        Outcome::Win(cost) => s1 = s1.min(cost),
                        Outcome::Lost(cost) => s2 = s2.max(cost),
                    }
                }
            }
        }
    }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
