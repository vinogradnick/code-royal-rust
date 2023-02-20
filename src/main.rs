use crate::OwnerType::Friendly;
use crate::SiteType::{Barracks, Mine, Tower};
use std::collections::{HashMap, HashSet};
use std::io;
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

const QUEEN_GUARD_HEALTH: i32 = 24;

const COUNT_BARRACKS: usize = 0;
const GOLD_INC: i32 = 2;

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
pub struct SiteOptions {
    gold_remaining: i32,
    max_mine_size: i32,
}

impl SiteOptions {
    pub fn new(g: i32, m: i32) -> SiteOptions {
        return SiteOptions {
            gold_remaining: g,
            max_mine_size: m,
        };
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum OwnerType {
    None = -1,
    Friendly = 0,
    Enemy = 1,
}

impl OwnerType {
    pub fn from(val: i32) -> OwnerType {
        return match val {
            0 => OwnerType::Friendly,
            1 => OwnerType::Enemy,
            _ => OwnerType::None,
        };
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SiteType {
    None = -1,
    Mine = 0,
    Barracks = 2,
    Tower = 1,
}

impl SiteType {
    pub fn from(val: i32) -> SiteType {
        match val {
            0 => SiteType::Mine,
            1 => SiteType::Tower,
            2 => SiteType::Barracks,
            _ => SiteType::None,
        }
    }
}

#[derive(Debug)]
pub struct CommonSite {
    site_id: i32,
    p: P,
    radius: i32,
    options: Option<SiteOptions>,
    s_type: SiteType,
    s_owner: OwnerType,
}

impl CommonSite {
    pub fn new(id: i32, x: i32, y: i32) -> CommonSite {
        CommonSite {
            site_id: id,
            p: P(x, y),
            radius: 0,
            options: None,
            s_type: SiteType::None,
            s_owner: OwnerType::None,
        }
    }
    pub fn update(&mut self, s_type: i32, s_owner: i32, options: SiteOptions) {
        self.s_owner = OwnerType::from(s_owner);
        self.s_type = SiteType::from(s_type);
        self.options = Option::from(options);
    }
    pub fn is_barracks(&self) -> bool {
        return self.s_type == Barracks;
    }
    pub fn is_tower(&self) -> bool {
        return self.s_type == Tower;
    }
    pub fn is_mine(&self) -> bool {
        return self.s_type == Mine;
    }
    pub fn is_own(&self) -> bool {
        return self.s_owner == OwnerType::Friendly;
    }
    pub fn is(&self, s: SiteType, owner: OwnerType) -> bool {
        return self.s_type == s && self.s_owner == owner;
    }
}

#[derive(Debug)]
struct Unit {
    p: P,
    health: i32,
    u_type: i32, //-1 queen
}

#[derive(Debug)]
struct ResourceManager {
    sites: HashMap<i32, CommonSite>,
    queen: Option<Box<Unit>>,
    gold: Box<i32>,
}

fn built(site: i32, s_type: SiteType) -> String {
    let t = match s_type {
        SiteType::Mine => String::from("MINE"),
        SiteType::Barracks => String::from("BARRACKS-KNIGHT"),
        SiteType::Tower => String::from("TOWER"),
        SiteType::None => String::from(""),
    };
    format!("BUILD {} {}", &site, t)
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            sites: HashMap::new(),
            queen: None,
            gold: Box::new(0),
        }
    }
    pub fn gold_inc(&mut self) -> i32 {
        return self
            .sites
            .values()
            .filter(|&x| x.is(Mine, OwnerType::Friendly))
            .count() as i32;
    }

    pub fn near_site(&self) -> Option<&CommonSite> {
        let t = self.queen.as_ref().unwrap();

        self.sites
            .values()
            .filter(|&x| x.is(SiteType::None, OwnerType::None))
            .min_by(|&x, &y| {
                let xp = x.p.near(&t.p);
                let yp = y.p.near(&t.p);
                return xp.cmp(&yp);
            })
    }

    // select building type from necessaries
    pub fn site_predict(&mut self) -> SiteType {
        // если притока золота хватит на еще то строим барак
        // иначе строим шахты f
        let barracks = self
            .sites
            .values()
            .filter(|&x| x.is(Barracks, Friendly))
            .collect::<Vec<_>>()
            .len();

        let gold_inc = self.gold_inc();

        eprintln!("GOLD INC {gold_inc}");
        if gold_inc < GOLD_INC {
            return SiteType::Mine;
        }
        if barracks > COUNT_BARRACKS {
            return SiteType::Tower;
        }
        return SiteType::Barracks;
    }
    pub fn process_build(&mut self) {
        eprintln!("process_build");
        let st = self.site_predict();
        let near = self.near_site();
        match near {
            Some(t) => {
                println!("{}", built(t.site_id, st));
            }
            None => {
                println!("WAIT");
            }
        }
    }
    pub fn queen_guard(&mut self) -> bool {
        eprintln!("queen_guard");
        let t = self.queen.as_ref().unwrap();

        if t.health < QUEEN_GUARD_HEALTH {
            let nearTower = self
                .sites
                .values()
                .filter(|&x| x.is(Tower, Friendly))
                .max_by(|&x, &y| {
                    let xp = x.p.near(&t.p);
                    let yp = y.p.near(&t.p);
                    return xp.cmp(&yp);
                });
            return match nearTower {
                Some(t) => {
                    println!("MOVE {} {}", t.p.x(), t.p.y());
                    return true;
                }
                _ => false,
            };
        }
        return false;
    }
    pub fn process_train(&mut self) {
        eprintln!("process_train");
        let gl = self.gold.as_ref() / 80;

        if gl == 0 {
            println!("TRAIN");
            return;
        }
        let barracks = self
            .sites
            .values()
            .filter(|&x| x.is(Barracks, Friendly))
            .take(gl as usize)
            .map(|x| x.site_id.to_string())
            .collect::<Vec<_>>();

        eprintln!("{}", barracks.len());
        if barracks.len() > 0 {
            println!("TRAIN {}", barracks.join(" "));
            return;
        }
        println!("TRAIN");
    }
    // move or build
    pub fn process_predict(&mut self) {
        self.process_build();
    }
    pub fn process(&mut self, tc: i32) {
        eprintln!("tc:{tc}");
        if !self.queen_guard() {
            match tc {
                -1 => self.process_predict(),
                _ => self.process_build(),
            }
        }

        self.process_train();
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut manager = ResourceManager::new();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let num_sites = parse_input!(input_line, i32);
    for i in 0..num_sites as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let site_id = parse_input!(inputs[0], i32);
        let x = parse_input!(inputs[1], i32);
        let y = parse_input!(inputs[2], i32);
        let radius = parse_input!(inputs[3], i32);
        manager
            .sites
            .insert(site_id, CommonSite::new(site_id, x, y));
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let gold = parse_input!(inputs[0], i32);
        manager.gold = Box::from(gold);
        let touched_site = parse_input!(inputs[1], i32); // -1 if none
        for i in 0..num_sites as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let site_id = parse_input!(inputs[0], i32);
            let gold_remaining = parse_input!(inputs[1], i32); // -1 if unknown
            let max_mine_size = parse_input!(inputs[2], i32); // -1 if unknown
            let structure_type = parse_input!(inputs[3], i32); // -1 = No structure, 0 = Goldmine, 1 = Tower, 2 = Barracks
            let owner = parse_input!(inputs[4], i32); // -1 = No structure, 0 = Friendly, 1 = Enemy
            let param_1 = parse_input!(inputs[5], i32);
            let param_2 = parse_input!(inputs[6], i32);
            manager.sites.get_mut(&site_id).unwrap().update(
                structure_type,
                owner,
                SiteOptions::new(gold_remaining, max_mine_size),
            );
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
            if unit_type == -1 && owner == OwnerType::Friendly as i32 {
                manager.queen = Some(Box::new(Unit {
                    health,
                    p: P(x, y),
                    u_type: -1,
                }));
            }
        }

        manager.process(touched_site);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // First line: A valid queen action
        // Second line: A set of training instructions
    }
}
