#[derive(Debug)]
#[allow(dead_code)]
enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

#[allow(dead_code)]
enum PlayerAction {
    Move {
        direction: Direction,
        speed: u8,
    },
    Wait,
    Attack(Direction),
}

fn main() {
    let actions = vec![
        PlayerAction::Wait,
        PlayerAction::Attack(Direction::North),
        PlayerAction::Move {
            direction: Direction::Northeast,
            speed: 2,
        },
    ];

    for action in actions {
        match action {
            PlayerAction::Wait => println!("Player wants to wait"),
            PlayerAction::Move { direction, speed } => {
                println!("Player wants to move in direction {:?} with speed {}",
                         direction,
                         speed)
            }
            PlayerAction::Attack(direction) => {
                println!("Player wants to attack direction {:?}", direction)
            }
        };
    }
}
