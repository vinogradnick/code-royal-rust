use std::collections::HashMap;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

const S_EMPTY: i32 = -1;

const S_TOWER: i32 = 1;
const S_BARRACKS: i32 = 2;
const S_MINE: i32 = 0;

const O_NO_OWNER: i32 = -1;
const O_FRIENDLY: i32 = 0;
const O_ENEMY: i32 = 1;

const U_QUEEN: i32 = -1;
const U_KNIGHT: i32 = 0;
const U_ARCHER: i32 = 1;
const U_GIANT: i32 = 2;

struct Unit {
    t_unit: i32,
    p: P,
    owner: i32,
    health: i32,
}

impl Unit {
    pub fn new(unit_type: i32, p: (i32, i32), owner: i32, health: i32) -> Unit {
        Unit {
            t_unit: unit_type,
            p: P(p.0, p.1),
            owner,
            health,
        }
    }
}

#[derive(Debug)]
struct P(i32, i32);

impl P {
    fn x(&self) -> i32 {
        return self.0;
    }
    fn y(&self) -> i32 {
        return self.1;
    }
    #[inline]
    fn near(&self, other: &P) -> i32 {
        let px = other.0 - self.0;
        let py = other.1 - self.1;

        return f32::sqrt(((px * px) + (py * py)) as f32) as i32;
    }
}

#[derive(Debug)]
struct Site {
    site_id: usize,
    radius: i32,
    p: P,
    t_site: i32,
    owner: i32,
}

impl Site {
    pub fn new(site_id: usize, radius: i32, p: (i32, i32)) -> Site {
        return Site {
            site_id,
            radius,
            t_site: S_EMPTY,
            owner: O_NO_OWNER,
            p: P(p.0, p.1),
        };
    }
}

struct Manager {
    sites: HashMap<usize, Site>,
    untis: Vec<Unit>,
    built_step: usize,
    gold: Box<i32>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            sites: HashMap::new(),
            untis: Vec::new(),
            built_step: 2,
            gold: Box::new(0),
        }
    }

    pub fn t_vec(&self, t_type: i32) -> Vec<&Site> {
        self.sites
            .values()
            .filter(|&x| x.owner == O_FRIENDLY && x.t_site == t_type)
            .collect::<Vec<_>>()
    }

    pub fn queen(&self) -> &Unit {
        return self
            .untis
            .iter()
            .find(|&u| u.owner == O_FRIENDLY && u.t_unit == U_QUEEN)
            .unwrap();
    }

    pub fn near_option<'a>(&'a mut self, ttype: i32, owner: i32) -> Option<&'a Site> {
        let queen = self.queen();
        let p = &queen.p;

        return self
            .sites
            .values()
            .filter(|&x| x.t_site == ttype && x.owner == owner)
            .min_by(|&x, &y| {
                let mx = x.p.near(p);
                let my = y.p.near(p);
                return mx.cmp(&my);
            });
    }

    /**
    proccess function
     */
    pub fn process(&mut self, near_site: i32) {
        if !self.queen_in_attack() {
            let near = self.near_option(S_EMPTY, O_NO_OWNER);
            if let Some(t) = near {
                let sid = t.site_id;
                self.process_build(&sid);
            } else {
                self.go_tower();
            }
        }
        self.process_train();
    }
    pub fn go_tower(&mut self) -> bool {
        let near_tower = self.near_option(S_TOWER, O_FRIENDLY);

        return match near_tower {
            None => false,
            Some(t) => {
                println!("MOVE {} {}", t.p.0, t.p.1);
                true
            }
        };
    }

    pub fn queen_in_attack(&mut self) -> bool {
        let q = self.queen();

        return q.health < 70;
    }
    pub fn build_predict(&self) -> i32 {}

    pub fn process_build(&mut self, near_id: &usize) {
        let site_v = self.sites.get(near_id).unwrap();

        let sites = self.t_vec(S_BARRACKS);

        make_built(
            &site_v.site_id,
            if sites.len() > 0 { S_TOWER } else { S_BARRACKS },
            U_KNIGHT,
        );
    }
    pub fn process_train(&mut self) -> usize {
        let train_summary = *self.gold / 80;
        let trainable_sites = self.t_vec(S_BARRACKS);
        let sites_ids = trainable_sites
            .iter()
            .take(train_summary as usize)
            .map(|x| x.site_id.to_string())
            .collect::<Vec<_>>();

        if sites_ids.len() == 0 {
            println!("TRAIN");
        } else {
            println!("TRAIN {}", sites_ids.join(" "));
        }

        return trainable_sites.len();
    }
}

fn make_built(site_id: &usize, t_site: i32, t_unit: i32) {
    let unit = match t_unit {
        U_ARCHER => "ARCHER".to_string(),
        U_KNIGHT => "KNIGHT".to_string(),
        U_GIANT => "GIANT".to_string(),
        _ => "".to_string(),
    };
    let s = match t_site {
        S_BARRACKS => "BARRACKS".to_string(),
        S_TOWER => "TOWER".to_string(),
        _ => "".to_string(),
    };

    let rb = if s == "BARRACKS".to_string() {
        format!("{}-{}", s, unit)
    } else {
        format!("{}", s)
    };

    println!("BUILD {} {}", site_id, rb);
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut manager = Manager::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let num_sites = parse_input!(input_line, i32);
    for i in 0..num_sites as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let site_id = parse_input!(inputs[0], usize);
        let x = parse_input!(inputs[1], i32);
        let y = parse_input!(inputs[2], i32);
        let radius = parse_input!(inputs[3], i32);

        let res = manager
            .sites
            .insert(site_id, Site::new(site_id, radius, (x, y)));

        match &res {
            _ => {}
        }
    }

    let mut round = 0 as usize;
    // game loop
    loop {
        round += 1;
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let gold = parse_input!(inputs[0], i32);
        manager.gold = Box::new(gold);
        let touched_site = parse_input!(inputs[1], i32); // -1 if none
        for i in 0..num_sites as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let site_id = parse_input!(inputs[0], usize);
            let ignore_1 = parse_input!(inputs[1], i32); // used in future leagues
            let ignore_2 = parse_input!(inputs[2], i32); // used in future leagues
            let structure_type = parse_input!(inputs[3], i32); // -1 = No structure, 1 = Tower, 2 = Barracks
            let owner = parse_input!(inputs[4], i32); // -1 = No structure, 0 = Friendly, 1 = Enemy
            let param_1 = parse_input!(inputs[5], i32);
            let param_2 = parse_input!(inputs[6], i32);

            let site = manager.sites.get_mut(&site_id).unwrap();
            site.t_site = structure_type;
            site.owner = owner;
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_units = parse_input!(input_line, i32);
        for i in 0..num_units as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let x = parse_input!(inputs[0], i32);
            let y = parse_input!(inputs[1], i32);
            let owner = parse_input!(inputs[2], i32);
            let unit_type = parse_input!(inputs[3], i32); // -1 = QUEEN, 0 = KNIGHT, 1 = ARCHER, 2 = GIANT
            let health = parse_input!(inputs[4], i32);
            manager
                .untis
                .push(Unit::new(unit_type, (x, y), owner, health));
        }

        manager.process(touched_site);
    }
}
