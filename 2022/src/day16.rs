use std::collections::HashMap;

struct Valve {
    index: usize,
    name: String,
    flow: u64,
    tunnels: Vec<String>
}

fn main() {
    let input = include_str!("../input/day16.input");

    // Part one
    let without_elephant = first_puzzle(input);
    println!("Without an elephant helping you, you can release at most {} pressure in 30 minutes.", without_elephant);

    // Part two
    let with_elephant = second_puzzle(input);
    println!("With an elephant helping you, you can release at most {} pressure in 26 minutes.", with_elephant);
}

#[test]
fn sample() {
    let sample = include_str!("../sample/day16.input");

    assert_eq!(first_puzzle(sample), 1651);
    //assert_eq!(second_puzzle(sample), 1707);
}

fn first_puzzle(source: &str) -> u64 {
    let valves = parse_valves(source);

    let mut open = 0;
    for valve in valves.values() {
        if valve.flow == 0 {
            open |= 1 << valve.index;
        }
    }

    let mut cache = HashMap::<(String, u64, u64), u64>::new();
    let start = "AA".to_owned();

    dfs(&valves, &mut cache, &start, open, 0, 30)
}

fn second_puzzle(source: &str) -> u64 {
    let valves = parse_valves(source);

    let mut open = 0;
    for valve in valves.values() {
        if valve.flow == 0 {
            open |= 1 << valve.index;
        }
    }

    let mut cache = HashMap::<(String, String, u64, u64), u64>::new();
    let start = "AA".to_owned();

    dfs_with_elephant(&valves, &mut cache, &start, &start, open, 0, 26)
}

fn dfs(
    valves: &HashMap<String, Valve>, 
    cache: &mut HashMap<(String, u64, u64), u64>, 
    name: &String, 
    open: u64, 
    rate: u64, 
    minutes: u64
) -> u64 {
    if minutes == 0 {
        return 0;
    }

    if let Some(&released) = cache.get(&(name.to_owned(), open, minutes)) {
        return released;
    }

    let mut released = 0;

    let valve = &valves[name];

    let all_open = u64::MAX >> (64 - valves.len());
    if open != all_open {
        if open & (1 << valve.index) == 0 {
            released = released.max(dfs(valves, cache, name, open | 1 << valve.index, rate + valve.flow, minutes - 1));
        }

        for option in &valve.tunnels {
            released = released.max(dfs(valves, cache, option, open, rate, minutes - 1));
        }

        released += rate;
    } else {
        released += rate * minutes;
    }

    cache.insert((name.to_owned(), open, minutes), released);
    released
}

fn dfs_with_elephant(
    valves: &HashMap<String, Valve>, 
    cache: &mut HashMap<(String, String, u64, u64), u64>, 
    hu_at: &String, 
    el_at: &String,
    open: u64, 
    rate: u64, 
    minutes: u64
) -> u64 {
    if minutes == 0 {
        return 0;
    }

    if let Some(&released) = cache.get(&(hu_at.to_owned(), el_at.to_owned(), open, minutes)) {
        return released;
    }

    let mut released = 0;

    let hu_valve = &valves[hu_at];
    let el_valve = &valves[el_at];

    let hu_valve_open = open & (1 << hu_valve.index) != 0;
    let el_valve_open = open & (1 << el_valve.index) != 0 || hu_valve.index == el_valve.index;

    let all_open = u64::MAX >> (64 - valves.len());
    if open != all_open {
        match (hu_valve_open, el_valve_open) {
            (false, false) => {
                released = released.max(dfs_with_elephant(
                    valves, 
                    cache, 
                    hu_at, 
                    el_at, 
                    open | 1 << el_valve.index | 1 << hu_valve.index, 
                    rate + el_valve.flow + hu_valve.flow, 
                    minutes - 1
                ));
            }

            (false, true) => {
                for el_option in &el_valve.tunnels {
                    released = released.max(
                        dfs_with_elephant(
                            valves, 
                            cache, 
                            hu_at,
                            el_option,
                            open | 1 << hu_valve.index, 
                            rate + hu_valve.flow, 
                            minutes - 1
                        )
                    );
                }
            }

            (true, false) => {
                for hu_option in &hu_valve.tunnels {
                    released = released.max(
                        dfs_with_elephant(
                            valves, 
                            cache, 
                            hu_option,
                            el_at,
                            open | 1 << el_valve.index, 
                            rate + el_valve.flow, 
                            minutes - 1
                        )
                    );
                }
            }

            (true, true) => {
                for hu_option in &hu_valve.tunnels {
                    for el_option in &el_valve.tunnels {
                        released = released.max(
                            dfs_with_elephant(
                                valves, 
                                cache, 
                                hu_option,
                                el_option,
                                open, 
                                rate, 
                                minutes - 1
                            )
                        );
                    }
                }
            }
        }

        released += rate;
    } else {
        released += rate * minutes;
    }

    cache.insert((hu_at.to_owned(), el_at.to_owned(), open, minutes), released);
    released
}

fn parse_valves(source: &str) -> HashMap<String, Valve> {
    let mut valves = HashMap::<String, Valve>::new();
    let mut index = 0;

    source
        .lines()
        .map(parse_line)
        .for_each(|mut valve| { 
            valve.index = index;
            valves.insert(valve.name.clone(), valve);

            index += 1;
        });

    valves
}

fn parse_line(source: &str) -> Valve {
    let mut parts = source.split("; ");

    let mut no_valve_prefix = parts
        .next()
        .expect("String ended early.")
        .strip_prefix("Valve ")
        .expect("Invalid valve statement.")
        .split(" has flow rate=");

    let name = no_valve_prefix.next().expect("Couldn't find valve name.").to_owned();
    let flow = no_valve_prefix.next().expect("Couldn't find valve flow rate.").parse::<u64>().expect("Invalid integer.");

    let mut tunnels_part = parts.next().expect("String ended early");

    if tunnels_part.starts_with("tunnels lead to valves ") {
        tunnels_part = tunnels_part.strip_prefix("tunnels lead to valves ").unwrap();
    } else if tunnels_part.starts_with("tunnel leads to valve ") {
        tunnels_part = tunnels_part.strip_prefix("tunnel leads to valve ").unwrap();
    } else {
        panic!("Invalid tunnels statement.");
    }

    let tunnels: Vec<String> = tunnels_part
        .split(", ")
        .map(|slice| slice.to_owned())
        .collect();
    
    Valve { index: 0, name, flow, tunnels }
}