mod btree;
mod btree2;

pub use btree::{part_1, part_2};
pub use btree2::part_3;

#[derive(Clone, Copy, Debug)]
enum Command<ID, R, S> {
    Add(ID, (R, S), (R, S)),
    Swap(ID),
}

fn parse_line(line: &str) -> Command<usize, usize, char> {
    let mut parts = line.split(' ');
    match parts.next().expect("invalid line") {
        "ADD" => {
            let id = parts
                .next()
                .expect("missing id")
                .split_once('=')
                .expect("missing id part")
                .1
                .parse()
                .expect("invalid id");
            let left = parse_node(parts.next().expect("missing left node"));
            let right = parse_node(parts.next().expect("missing right node"));
            Command::Add(id, left, right)
        }
        "SWAP" => Command::Swap(
            parts
                .next()
                .expect("missing id")
                .parse()
                .expect("invalid number"),
        ),
        _ => unreachable!("invalid command"),
    }
}

fn parse_node(node: &str) -> (usize, char) {
    let (rank, symbol) = node.split_once('=').unwrap().1.split_once(',').unwrap();

    (
        rank.split_once('[')
            .expect("Cannot find rank")
            .1
            .parse()
            .expect("Invalid rank"),
        symbol.chars().next().expect("Cannot find symbol"),
    )
}
