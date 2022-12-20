use std::collections::{HashMap};

#[derive(Copy, Clone)]
struct Blueprint {
    index: u64,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
    max_ore_cost: u64,
}

impl Blueprint {
    fn from_slice(source: &str) -> Self {
        let ended_early = "String ended early.";
        let invalid_format = "Invalid format.";
        let invalid_number = "Invalid number format.";

        let mut ore_robot_cost = Resources::default();
        let mut clay_robot_cost = Resources::default();
        let mut obsidian_robot_cost = Resources::default();
        let mut geode_robot_cost = Resources::default();

        let parts = source
            .split_once(": ")
            .expect("Invalid format.");

        let index = parts
            .0
            .strip_prefix("Blueprint ")
            .expect(invalid_format)
            .parse::<u64>()
            .expect(invalid_number);

        let mut parts = parts
            .1
            .split('.');

        ore_robot_cost.ore = parts
            .next()
            .expect(ended_early)
            .trim()
            .strip_prefix("Each ore robot costs ")
            .expect(invalid_format)
            .strip_suffix(" ore")
            .expect(invalid_format)
            .parse::<u64>()
            .expect(invalid_number);

        clay_robot_cost.ore = parts
            .next()
            .expect(ended_early)
            .trim()
            .strip_prefix("Each clay robot costs ")
            .expect(invalid_format)
            .strip_suffix(" ore")
            .expect(invalid_format)
            .parse::<u64>()
            .expect(invalid_number);

        let mut obsidian_robot_cost_parts = parts
            .next()
            .expect(ended_early)
            .trim()
            .strip_prefix("Each obsidian robot costs ")
            .expect(invalid_format)
            .strip_suffix(" clay")
            .expect(invalid_format)
            .split(" ore and ");

        obsidian_robot_cost.ore = obsidian_robot_cost_parts.next().expect(invalid_format).parse::<u64>().expect(invalid_number);
        obsidian_robot_cost.clay = obsidian_robot_cost_parts.next().expect(invalid_format).parse::<u64>().expect(invalid_number);

        let mut geode_robot_cost_parts = parts
            .next()
            .expect(ended_early)
            .trim()
            .strip_prefix("Each geode robot costs ")
            .expect(invalid_format)
            .strip_suffix(" obsidian")
            .expect(invalid_format)
            .split(" ore and ");

        geode_robot_cost.ore = geode_robot_cost_parts.next().expect(invalid_format).parse::<u64>().expect(invalid_number);
        geode_robot_cost.obsidian = geode_robot_cost_parts.next().expect(invalid_format).parse::<u64>().expect(invalid_number);

        let mut max_ore_cost = u64::MIN;
        max_ore_cost = max_ore_cost.max(ore_robot_cost.ore);
        max_ore_cost = max_ore_cost.max(clay_robot_cost.ore);
        max_ore_cost = max_ore_cost.max(obsidian_robot_cost.ore);
        max_ore_cost = max_ore_cost.max(geode_robot_cost.ore);

        Self {
            index,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
            max_ore_cost
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
struct Resources {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geodes: u64,
}

impl Resources {
    fn check(&self, cost: &Resources) -> bool {
        self.ore >= cost.ore && 
        self.clay >= cost.clay && 
        self.obsidian >= cost.obsidian &&
        self.geodes >= cost.geodes
    }

    fn spend(&mut self, cost: &Resources) {
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
        self.geodes -= cost.geodes;
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Robots {
    ore: u64,
    clay: u64,
    obsidian: u64,
    geodes: u64
}

impl Default for Robots {
    fn default() -> Self {
        Self {
            ore: 1,
            clay: 0,
            obsidian: 0,
            geodes: 0
        }
    }
}

fn main() {
    let input = include_str!("../input/day19.input");

    // Part one
    let sum = first_puzzle(input);
    println!("The quality level of all the blueprints is {}.", sum);

    // Part two
    let product = second_puzzle(input);
    println!("The product of the most geodes you can get with the first three blueprints is {}.", product);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day19.input");

    assert_eq!(first_puzzle(sample), 33);
    assert_eq!(second_puzzle(sample), 3472);
}

fn first_puzzle(source: &str) -> u64 {
    let resources = Resources::default();
    let robots = Robots::default();

    source
        .lines()
        .map(Blueprint::from_slice)
        .map(|blueprint| {
            let mut cache = HashMap::<(Resources, Robots, u64), Resources>::new();
            let mut upper_bound = u64::MIN;

            blueprint.index * simulate_blueprint(&blueprint, &mut cache, &mut upper_bound, resources, robots, 24).geodes
        })
        .sum()
}

fn second_puzzle(source: &str) -> u64 {
    let resources = Resources::default();
    let robots = Robots::default();

    source
        .lines()
        .take(3)
        .map(Blueprint::from_slice)
        .map(|blueprint| {
            let mut cache = HashMap::<(Resources, Robots, u64), Resources>::new();
            let mut upper_bound = u64::MIN;

            simulate_blueprint(&blueprint, &mut cache, &mut upper_bound, resources, robots, 32).geodes
        })
        .product()
}

fn simulate_blueprint(
    blueprint: &Blueprint,
    cache: &mut HashMap<(Resources, Robots, u64), Resources>,
    upper_bound: &mut u64,
    mut resources: Resources,
    robots: Robots,
    minutes: u64
) -> Resources {
    if minutes == 0 {
        if resources.geodes > *upper_bound {
            *upper_bound = resources.geodes;
        }

        return resources;
    }

    let extrapolate = resources.geodes + robots.geodes * minutes + (minutes * (minutes + 1)) / 2;
    if extrapolate <= *upper_bound {
        return resources;
    }

    let key = (resources, robots, minutes);
    if let Some(r) = cache.get(&key) {
        return *r;
    }

    let can_build_geode_robot = resources.check(&blueprint.geode_robot_cost);
    let can_build_obsidian_robot = resources.check(&blueprint.obsidian_robot_cost);
    let can_build_clay_robot = resources.check(&blueprint.clay_robot_cost);
    let can_build_ore_robot = resources.check(&blueprint.ore_robot_cost);

    resources.ore += robots.ore;
    resources.clay += robots.clay;
    resources.obsidian += robots.obsidian;
    resources.geodes += robots.geodes;

    let mut result = resources;

    if can_build_geode_robot {
        let mut resources_after = resources;
        let mut robots_after = robots;
        
        resources_after.spend(&blueprint.geode_robot_cost);
        robots_after.geodes += 1;

        let res = simulate_blueprint(blueprint, cache, upper_bound, resources_after, robots_after, minutes - 1);

        if res.geodes >= result.geodes {
            result = res;
        }
    } 
    
    if can_build_obsidian_robot && robots.obsidian < blueprint.geode_robot_cost.obsidian {
        let mut resources_after = resources;
        let mut robots_after = robots;

        resources_after.spend(&blueprint.obsidian_robot_cost);
        robots_after.obsidian += 1;

        let res = simulate_blueprint(blueprint, cache, upper_bound, resources_after, robots_after, minutes - 1);

        if res.geodes >= result.geodes {
            result = res;
        }
    } 
    
    if can_build_clay_robot && robots.clay < blueprint.obsidian_robot_cost.clay {
        let mut resources_after = resources;
        let mut robots_after = robots;

        resources_after.spend(&blueprint.clay_robot_cost);
        robots_after.clay += 1;

        let res = simulate_blueprint(blueprint, cache, upper_bound, resources_after, robots_after, minutes - 1);

        if res.geodes >= result.geodes {
            result = res;
        }
    } 
    
    if can_build_ore_robot && robots.ore < blueprint.max_ore_cost {
        let mut resources_after = resources;
        let mut robots_after = robots;

        resources_after.spend(&blueprint.ore_robot_cost);
        robots_after.ore += 1;

        let res = simulate_blueprint(blueprint, cache, upper_bound, resources_after, robots_after, minutes - 1);

        if res.geodes >= result.geodes {
            result = res;
        }
    }

    let res = simulate_blueprint(blueprint, cache, upper_bound, resources, robots, minutes - 1);
    if res.geodes >= result.geodes {
        result = res;
    }

    cache.insert(key, result);

    result
}