use bracket_lib::{
    color::{BLACK, RED},
    random::RandomNumberGenerator,
    terminal::{to_cp437, BTerm},
};

use crate::constans::SCREEN_HEIGHT;
use crate::player::Player;

pub struct Obstacle {
    pub x: i32,
    pub gap_y: i32,
    pub size: i32,
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Obstacle {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;
        // Draw the top half of the obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        // Draw the bottom half of the obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    pub fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;

        does_x_match && (player_below_gap || player_above_gap)
    }
}
