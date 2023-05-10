use std::ops::Range;
use rand::Rng;

use crate::field::util::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub rect: Rect,
    pub room: Rect,
    pub arms: Vec<Rect>
}

impl Area {
    pub fn create(rect: Rect) -> Self {
        return Self {rect: rect, room: Rect::new(), arms: vec![Rect::new(); 0]};
    }

    pub fn x_iter(&self) -> Range<usize> {
        self.rect.x_iter()
    }

    pub fn y_iter(&self) -> Range<usize> {
        self.rect.y_iter()
    }

    pub fn size(&self) -> Point {
        self.rect.size()
    }

    pub fn space(&self) -> usize {
        self.rect.space()
    }

    pub fn s(&self) -> Point {
        self.rect.s
    }

    pub fn g(&self) -> Point {
        self.rect.g
    }

    pub fn gen_room(&mut self) {
        let mut rng = rand::thread_rng();

        let size_x = rng.gen_range((self.size().x / 2)..=(self.size().x - 2));
        let size_y = rng.gen_range((self.size().y / 2)..=(self.size().y - 2));
        
        let start_x = rng.gen_range((self.s().x + 1)..=(self.g().x - size_x));
        let start_y = rng.gen_range((self.s().y + 1)..=(self.g().y - size_y));

        self.room = Rect::create(Point::create(start_x, start_y), Point::create(start_x + size_x - 1, start_y + size_y - 1));
    }

    pub fn gen_arms(&mut self, direction: Direction) {
        if self.room.size().x == 0 || self.room.size().y == 0 { panic!("No room exists in the area"); } 

        let mut rng = rand::thread_rng();
        let max_arm_num = 3;
        let arm_num = rng.gen_range(1..=max_arm_num);

        for _ in 0..arm_num {
            let start_x = match direction {
                Direction::LEFT => self.room.s.x,
                Direction::RIGHT => self.room.g.x,
                _ => rng.gen_range(self.room.s.x..=self.room.g.x)
            };

            let start_y = match direction {
                Direction::UP => self.room.s.y,
                Direction::DOWN => self.room.g.y,
                _ => rng.gen_range(self.room.s.y..=self.room.g.y)
            };

            let end_x = match direction {
                Direction::LEFT => self.rect.s.x - 1,
                Direction::RIGHT => self.rect.g.x + 1,
                _ => start_x
            };

            let end_y = match direction {
                Direction::UP => self.rect.s.y - 1,
                Direction::DOWN => self.rect.g.y + 1,
                _ => start_y
            };

            // check adjacent arms
            if match direction {
                Direction::DOWN     => self.arms.iter().find(|arm| start_y == arm.s.y && (arm.s.x == start_x + 1 || arm.s.x + 1 == start_x)),
                Direction::UP       => self.arms.iter().find(|arm| start_y == arm.g.y && (arm.s.x == start_x + 1 || arm.s.x + 1 == start_x)),
                Direction::RIGHT    => self.arms.iter().find(|arm| start_x == arm.s.x && (arm.s.y == start_y + 1 || arm.s.y + 1 == start_y)),
                _                    => self.arms.iter().find(|arm| start_x == arm.g.x && (arm.s.y == start_y + 1 || arm.s.y + 1 == start_y))
            }.is_some() { continue; };

            let (arm_start, arm_end) = match direction {
                Direction::LEFT | Direction::UP => (Point::create(end_x, end_y), Point::create(start_x, start_y)),
                _                               => (Point::create(start_x, start_y), Point::create(end_x, end_y))
            };

            self.arms.push(Rect::create(arm_start, arm_end));
        }
    }
}
