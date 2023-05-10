mod area;
use area::Area;

mod util;
use util::*;

mod maptype;
use maptype::*;

use rand::Rng;

pub struct Field {
    size: Point,
    field: Vec<MapType>
}

impl Field {
    pub fn create(size_x: usize, size_y: usize) -> Field {
        assert!(size_x >= 8, "x size is too small.");
        assert!(size_y >= 8, "y size is too small.");
        return Self {size: Point::create(size_x, size_y), field: vec![MapType::Wall; size_x * size_y]};
    }
  
    pub fn print(&self) {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                print!("{}", self.at(x, y));
            }
            print!("\n");
        }
    }

    pub fn gen_map(&mut self, num_of_areas: usize) {
        let mut rng = rand::thread_rng();

        let root = Area::create(Rect::create(Point::create(0, 0), Point::create(self.size.x - 1, self.size.y - 1)));
        let mut areas = vec![root; 1];
        let mut area_dividers = vec![Rect::new(); 0];

        let max_area_size_x = 8;
        let max_area_size_y = 8;

        // divide field to areas
        for _ in 0..num_of_areas {
            let Some(target_area) = areas.iter_mut()
                .filter(|area| area.size().x >= 2 * max_area_size_x + 1 && area.size().y >= 2 * max_area_size_y + 1)
                .max_by_key(|area| area.space())
            else {
                break;
            };


            if target_area.size().x > target_area.size().y {
                let divide_x = rng.gen_range((target_area.s().x + max_area_size_x)..=(target_area.g().x - max_area_size_x));
                area_dividers.push(Rect::create(Point::create(divide_x, target_area.s().y), Point::create(divide_x, target_area.g().y)));

                let rec = Rect::create(Point::create(divide_x + 1, target_area.s().y), target_area.g());
                *target_area = Area::create(Rect::create(target_area.s(), Point::create(divide_x - 1, target_area.g().y)));
                areas.push(Area::create(rec));
            } else {
                let divide_y = rng.gen_range((target_area.s().y + max_area_size_y)..=(target_area.g().y - max_area_size_x));
                area_dividers.push(Rect::create(Point::create(target_area.s().x, divide_y), Point::create(target_area.g().x, divide_y)));

                let rec = Rect::create(Point::create(target_area.s().x, divide_y + 1), target_area.g());
                *target_area = Area::create(Rect::create(target_area.s(), Point::create(target_area.g().x, divide_y - 1)));
                areas.push(Area::create(rec));
            }
        }

        // generate room
        for area in areas.iter_mut() { 
            area.gen_room();

            if area.rect.s.x != 0 { area.gen_arms(Direction::LEFT); }
            if area.rect.g.x != self.size.x - 1 { area.gen_arms(Direction::RIGHT); }
            if area.rect.s.y != 0 { area.gen_arms(Direction::UP); }
            if area.rect.g.y != self.size.y - 1 { area.gen_arms(Direction::DOWN); }
        }

        for area in areas.iter() {
            let room = area.room;
            for x in room.x_iter() {
                for y in room.y_iter() {
                    *self.at_im(x, y) = MapType::Road;
                }
            }

            for arm in area.arms.iter() {
                for x in arm.x_iter() {
                    for y in arm.y_iter() {
                        *self.at_im(x, y) = MapType::Road;
                    }
                }
            }
        }

        // generate roads
        for area_divider in area_dividers {
            if area_divider.s.x == area_divider.g.x {
                // divide_x
                let divide_x = area_divider.s.x;
                let adjacents = area_divider.y_iter().filter(|&y| self.at(divide_x, y) == MapType::Road);
                let Some(up) = adjacents.clone().min_by_key(|&y| y) else { panic!("cannot generate roads"); };
                let Some(down) = adjacents.max_by_key(|&y| y) else { panic!("cannot generate roads"); };
                for y in up..=down { *self.at_im(divide_x, y) = MapType::Road; }
            } else {
                // divide_y
                let divide_y = area_divider.s.y;
                let adjacents = area_divider.x_iter().filter(|&x| self.at(x, divide_y) == MapType::Road);
                let Some(left) = adjacents.clone().min_by_key(|&x| x) else { panic!("cannot generate roads"); };
                let Some(right) = adjacents.max_by_key(|&x| x) else { panic!("cannot generate roads"); };
                for x in left..=right { *self.at_im(x, divide_y) = MapType::Road; }
            }
        }
    }

    fn at_im(&mut self, x: usize, y: usize) -> &mut MapType {
        return &mut self.field[y * self.size.x + x];
    }

    fn at(&self, x: usize, y: usize) -> MapType {
        return self.field[y * self.size.x + x];
    }
}
