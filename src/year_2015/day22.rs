use std::{cmp::Reverse, collections::BinaryHeap};

use crate::helpers::{get_lines, parse};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct GameState {
    pub mana_spent: u64,
    pub player_hp: i64,
    pub player_mana: i64,
    pub boss_hp: i64,
    pub boss_dmg: i64,
    pub shield: u64,
    pub poison: u64,
    pub mana_regen: u64,
}

impl GameState {
    pub fn start(boss_hp: i64, boss_dmg: i64) -> Self {
        Self {
            mana_spent: 0,
            player_hp: 50,
            player_mana: 500,
            boss_hp,
            boss_dmg,
            shield: 0,
            poison: 0,
            mana_regen: 0,
        }
    }

    pub fn missile(&self) -> Self {
        Self {
            mana_spent: self.mana_spent + 53,
            player_hp: self.player_hp,
            player_mana: self.player_mana - 53,
            boss_hp: self.boss_hp - 4,
            boss_dmg: self.boss_dmg,
            shield: self.shield,
            poison: self.poison,
            mana_regen: self.mana_regen,
        }
    }

    pub fn drain(&self) -> Self {
        Self {
            mana_spent: self.mana_spent + 73,
            player_hp: self.player_hp + 2,
            player_mana: self.player_mana - 73,
            boss_hp: self.boss_hp - 2,
            boss_dmg: self.boss_dmg,
            shield: self.shield,
            poison: self.poison,
            mana_regen: self.mana_regen,
        }
    }

    pub fn shield(&self) -> Self {
        Self {
            mana_spent: self.mana_spent + 113,
            player_hp: self.player_hp,
            player_mana: self.player_mana - 113,
            boss_hp: self.boss_hp,
            boss_dmg: self.boss_dmg,
            shield: 6,
            poison: self.poison,
            mana_regen: self.mana_regen,
        }
    }

    pub fn poison(&self) -> Self {
        Self {
            mana_spent: self.mana_spent + 173,
            player_hp: self.player_hp,
            player_mana: self.player_mana - 173,
            boss_hp: self.boss_hp,
            boss_dmg: self.boss_dmg,
            shield: self.shield,
            poison: 6,
            mana_regen: self.mana_regen,
        }
    }

    pub fn recharge(&self) -> Self {
        Self {
            mana_spent: self.mana_spent + 229,
            player_hp: self.player_hp,
            player_mana: self.player_mana - 229,
            boss_hp: self.boss_hp,
            boss_dmg: self.boss_dmg,
            shield: self.shield,
            poison: self.poison,
            mana_regen: 5,
        }
    }

    pub fn boss(&self, hard: bool) -> Self {
        let armor = if self.shield > 0 { 7 } else { 0 };
        let boss_hp = if self.poison > 0 {
            self.boss_hp - 3
        } else {
            self.boss_hp
        };
        let player_mana = if self.mana_regen > 0 {
            self.player_mana + 101
        } else {
            self.player_mana
        };
        if boss_hp <= 0 {
            return Self {
                mana_spent: self.mana_spent,
                player_hp: if hard {
                    self.player_hp - 1
                } else {
                    self.player_hp
                },
                player_mana,
                boss_hp,
                boss_dmg: self.boss_dmg,
                shield: self.shield,
                poison: self.poison,
                mana_regen: self.mana_regen,
            };
        }
        let shield = self.shield.saturating_sub(1);
        let mana_regen = self.mana_regen.saturating_sub(1);
        let poison = self.poison.saturating_sub(1);
        Self {
            mana_spent: self.mana_spent,
            player_hp: self.player_hp
                - if self.boss_dmg > armor {
                    self.boss_dmg - armor
                } else {
                    1
                },
            player_mana,
            boss_hp,
            boss_dmg: self.boss_dmg,
            shield,
            poison,
            mana_regen,
        }
    }
}

pub fn player(state: GameState, hard: bool) -> Vec<GameState> {
    let state = GameState {
        mana_spent: state.mana_spent,
        player_hp: if hard {
            state.player_hp - 1
        } else {
            state.player_hp
        },
        player_mana: if state.mana_regen > 0 {
            state.player_mana + 101
        } else {
            state.player_mana
        },
        boss_hp: if state.poison > 0 {
            state.boss_hp - 3
        } else {
            state.boss_hp
        },
        boss_dmg: state.boss_dmg,
        shield: state.shield.saturating_sub(1),
        poison: state.poison.saturating_sub(1),
        mana_regen: state.mana_regen.saturating_sub(1),
    };
    if state.boss_hp <= 0 {
        return vec![state];
    }
    // Magic Missile
    let mut outcome = Vec::new();
    if state.player_mana >= 53 {
        outcome.push(state.missile());
    }
    // Drain
    if state.player_mana >= 73 {
        outcome.push(state.drain());
    }
    // Shield (6 turns)
    if state.player_mana >= 113 && state.shield == 0 {
        outcome.push(state.shield());
    }
    // Poison (6 turns)
    if state.player_mana >= 173 && state.poison == 0 {
        outcome.push(state.poison());
    }

    // Recharge (5 turns)
    if state.player_mana >= 229 && state.mana_regen == 0 {
        outcome.push(state.recharge());
    }
    outcome
        .iter()
        .map(|s| s.boss(hard))
        .filter(|s| s.player_hp > 0)
        .collect()
}

pub fn solve() {
    println!("Day 22 of 2015");
    let lines = get_lines();
    let hp = parse(&lines[0]).1[0];
    let dmg = parse(&lines[1]).1[0];
    let mut s1 = 0;
    let mut s2 = 0;
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(GameState::start(hp, dmg)));
    loop {
        let game_state = queue.pop().unwrap();
        for state in player(game_state.0, false) {
            queue.push(Reverse(state));
        }
        if game_state.0.player_hp <= 0 {
            continue;
        }
        if game_state.0.boss_hp <= 0 {
            s1 = game_state.0.mana_spent;
            break;
        }
    }
    println!("Part 1: {}", s1);
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(GameState::start(hp, dmg)));
    loop {
        let game_state = queue.pop().unwrap();
        for state in player(game_state.0, true) {
            queue.push(Reverse(state));
        }
        if game_state.0.player_hp <= 0 {
            continue;
        }
        if game_state.0.boss_hp <= 0 {
            s2 = game_state.0.mana_spent;
            break;
        }
    }
    println!("Part 2: {}", s2);
}
