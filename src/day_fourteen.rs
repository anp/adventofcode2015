#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    sprinting: bool,
    sprint_time: u32,
    rest_time: u32,
    sprint_remaining: u32,
    rest_remaining: u32,
    distance: u32,
    score: u32,
}

pub fn solve_part_one(input: &str) {
    let mut reindeer = parse_reindeer_stats(input);

    for _ in 0..2503 {
        tick_race(&mut reindeer);
    }

    {
        reindeer.sort_by(|a, b| b.distance.cmp(&a.distance));
        let fastest = &reindeer[0];
        println!("The fastest reindeer ({}) covered {} km in the race.",
                 fastest.name,
                 fastest.distance);
    }

    {
        reindeer.sort_by(|a, b| b.score.cmp(&a.score));
        let winner = &reindeer[0];
        println!("The winning reindeer ({}) won with {} points.",
                 winner.name,
                 winner.score);
    }
}

fn parse_reindeer_stats(input: &str) -> Vec<Reindeer> {
    let mut reindeer = Vec::new();

    for line in input.lines() {
        let tokens = line.split(' ').collect::<Vec<_>>();
        let speed = tokens[3].parse::<u32>().unwrap();
        let sprint_time = tokens[6].parse::<u32>().unwrap();
        let rest_time = tokens[13].parse::<u32>().unwrap();

        reindeer.push(Reindeer {
            name: tokens[0].to_string(),
            speed: speed,
            sprint_time: sprint_time,
            rest_time: rest_time,
            sprinting: true,
            sprint_remaining: sprint_time,
            rest_remaining: rest_time,
            distance: 0,
            score: 0,
        });
    }
    reindeer
}

fn tick_race(reindeer: &mut Vec<Reindeer>) {
    for deer in reindeer.iter_mut() {
        if deer.sprint_remaining > 0 {
            deer.distance += deer.speed;
            deer.sprint_remaining -= 1;
            deer.rest_remaining = deer.rest_time;
        } else {
            deer.rest_remaining -= 1;
            if deer.rest_remaining == 0 {
                deer.sprint_remaining = deer.sprint_time;
            }
        }
    }

    reindeer.sort_by(|a, b| b.distance.cmp(&a.distance));
    reindeer[0].score += 1;
    for i in 1..reindeer.len() {
        if reindeer[i].distance == reindeer[0].distance {
            reindeer[i].score += 1;
        } else {
            break;
        }
    }
}

#[test]
fn test_tick_race() {
    let comet = Reindeer {
        name: "Comet".to_string(),
        speed: 14,
        sprint_time: 10,
        rest_time: 127,
        sprinting: true,
        sprint_remaining: 10,
        rest_remaining: 127,
        distance: 0,
        score: 0,
    };

    let dancer = Reindeer {
        name: "Dancer".to_string(),
        speed: 16,
        sprint_time: 11,
        rest_time: 162,
        sprinting: true,
        sprint_remaining: 11,
        rest_remaining: 162,
        distance: 0,
        score: 0,
    };

    let mut reindeer = vec![comet, dancer];

    for _ in 0..1000 {
        tick_race(&mut reindeer);
    }

    assert_eq!(1120, reindeer[0].distance);
    assert_eq!(1056, reindeer[1].distance);
    assert_eq!(312, reindeer[0].score);
    assert_eq!(689, reindeer[1].score);
    
}