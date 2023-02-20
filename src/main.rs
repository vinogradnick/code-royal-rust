use std::collections::{HashMap, HashSet};
use std::io;
macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
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
enum OwnerType {
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
enum SiteType {
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
            .filter(|&x| x.s_type == SiteType::Mine && x.s_owner == OwnerType::Friendly)
            .map(|x| match &x.options {
                Some(opt) => &opt.gold_remaining,
                None => &0,
            })
            .sum();
    }

    pub fn near_site(&self) -> Option<&CommonSite> {
        let t = self.queen.as_ref().unwrap();

        self.sites.values().min_by(|&x, &y| {
            let xp = x.p.near(&t.p);
            let yp = y.p.near(&t.p);
            return xp.cmp(&yp);
        })
    }
    pub fn site_predict(&mut self) -> SiteType {
        let inc = self.gold_inc();
        let barracks = self
            .sites
            .values()
            .filter(|&x| x.s_type == SiteType::Barracks && x.s_owner == OwnerType::Friendly)
            .count();

        if gold > (barracks * 40) as i32 {
            return SiteType::Barracks;
        }
        return SiteType::Mine;
    }
    pub fn process_build(&mut self) {
        let st = self.site_predict();
        let near = self.near_site();
        match near {
            Some(t) => {
                built(t.site_id, st);
            }
            None => {}
        }
    }
    pub fn process_train(&mut self) {
        let gl = self.gold.as_ref() / 80;

        if gl == 0 {
            println!("TRAIN");
            return;
        }
        let barracks = self
            .sites
            .values()
            .filter(|&x| x.s_type == SiteType::Barracks && x.s_owner == OwnerType::Friendly)
            .take(gl as usize)
            .map(|x| x.site_id.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        println!("TRAIN {}", barracks);
    }
    pub fn process_predict(&mut self) {}
    pub fn process(&mut self, tc: i32) {
        match tc {
            -1 => self.process_predict(),
            _ => self.process_build(),
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
