use std::collections::HashMap;

#[derive(Debug)]
struct Vec2(f32, f32);

type EntId = u64;

struct World {
    positions: HashMap<EntId, Vec2>,
    velocities: HashMap<EntId, Vec2>,
    ents: Vec<EntId>,
    next: EntId,
}

impl World {
    fn new() -> Self {
        World {
            positions: HashMap::new(),
            velocities: HashMap::new(),
            ents: Vec::new(),
            next: 0,
        }
    }

    fn upd(&mut self) {
        for i in &self.ents {
            if let (Some(p), Some(v)) = (self.positions.get_mut(&i), self.velocities.get_mut(&i)) {
                p.0 += v.0;
                p.1 += v.1;
            }
        }
    }

    fn new_ent(&mut self) -> EntId {
        let ent = self.next;
        self.next += 1;
        self.ents.push(ent);
        ent
    }

    fn remove_ent(&mut self, e: EntId) {
        self.ents.retain(|t| *t != e);
        self.positions.remove(&e);
        self.velocities.remove(&e);
    }

    fn add_pos(&mut self, e: EntId, v: Vec2) {
        self.positions.insert(e, v);
    }

    fn add_vel(&mut self, e: EntId, v: Vec2) {
        self.velocities.insert(e, v);
    }

    fn print(&self) {
        for i in &self.ents {
            match (self.positions.get(&i), self.velocities.get(&i)) {
                (Some(p), Some(v)) => {
                    println!("Ent #{}: pos = {:?}, vel = {:?}", i, p, v);
                }
                (Some(p), None) => {
                    println!("Ent #{}: pos = {:?}", i, p);
                }
                (None, Some(v)) => {
                    println!("Ent #{}: vel = {:?}", i, v);
                }
                (None, None) => {
                    println!("Ent #{}", i);
                }
            }
        }
    }
}

fn main() {
    let mut w = World::new();
    for i in 0..4 {
        for j in 0..2 {
            let e = w.new_ent();
            w.add_pos(e,
                      Vec2(0.125 * (j + i * 8) as f32, 0.25 * (j + i * 8) as f32));
            w.add_vel(e,
                      Vec2(0.25 * (j + i * 8) as f32, 0.125 * (j + i * 8) as f32));
        }
        for j in 2..4 {
            let e = w.new_ent();
            w.add_pos(e,
                      Vec2(0.125 * (j + i * 8) as f32, 0.25 * (j + i * 8) as f32));
        }
        for j in 4..6 {
            let e = w.new_ent();
            w.add_vel(e,
                      Vec2(0.25 * (j + i * 8) as f32, 0.125 * (j + i * 8) as f32));
        }
        for j in 6..8 {
            let e = w.new_ent();
        }
    }
    w.print();
    w.upd();
    w.print();
}
