use aoc2021::Input;
use std::{
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    io::BufRead,
};

type Cost = u64;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn cost(&self) -> Cost {
        match self {
            Self::Amber => 1,
            Self::Bronze => 10,
            Self::Copper => 100,
            Self::Desert => 1000,
        }
    }

    fn parse(c: u8) -> Option<Self> {
        match c {
            b'A' => Some(Self::Amber),
            b'B' => Some(Self::Bronze),
            b'C' => Some(Self::Copper),
            b'D' => Some(Self::Desert),
            _ => None,
        }
    }

    fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::Amber),
            1 => Some(Self::Bronze),
            2 => Some(Self::Copper),
            3 => Some(Self::Desert),
            _ => None,
        }
    }
}

impl Display for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Amber => "A",
            Self::Bronze => "B",
            Self::Copper => "C",
            Self::Desert => "D",
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Room<const SIZE: usize> {
    id: Amphipod,
    slots: [Option<Amphipod>; SIZE],
}

impl<const SIZE: usize> Room<SIZE> {
    fn is_done(&self) -> bool {
        self.slots.iter().all(|s| s == &Some(self.id))
    }

    fn take(&mut self) -> Option<(Cost, Amphipod)> {
        for i in 0..SIZE {
            if self.slots[i].is_some() {
                if self.slots[i] != Some(self.id) {
                    return self.slots[i].take().map(|s| (i as Cost + 1, s));
                } else if self.slots[i + 1..].iter().any(|s| s != &Some(self.id)) {
                    return self.slots[i].take().map(|s| (i as Cost + 1, s));
                } else {
                    return None;
                }
            }
        }
        None
    }

    fn accepts(&self, amphipod: Amphipod) -> bool {
        if amphipod != self.id {
            return false;
        }
        self.slots
            .iter()
            .all(|s| s.is_none() || s == &Some(self.id))
    }

    fn accept(&mut self, amphipod: Amphipod) -> Cost {
        debug_assert!(self.accepts(amphipod));

        for i in (0..SIZE).rev() {
            if self.slots[i].is_none() {
                self.slots[i] = Some(amphipod);
                return i as Cost + 1;
            }
        }
        panic!("oops");
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Connections([Option<Amphipod>; 11]);

impl Connections {
    fn can_access_room_from_room(&self, from_room: usize, to_room: usize) -> Cost {
        let start_index = 2 + from_room * 2;
        let end_index = 2 + to_room * 2;
        self.int_move(start_index, end_index, false)
    }

    fn can_access_room_from_tile(&self, from_tile: usize, to_room: usize) -> Cost {
        let end_index = 2 + to_room * 2;
        self.int_move(from_tile, end_index, false)
    }

    fn can_acccess_tile_from_room(&self, room: usize, tile: usize) -> Cost {
        let room_index = 2 + room * 2;
        self.int_move(room_index, tile, true)
    }

    // fn can_access_tile_from_tile(&self, from_tile: usize, to_tile: usize) -> Cost {
    //     self.int_move(from_tile, to_tile, true)
    // }

    fn int_move(&self, from_tile: usize, to_tile: usize, target_is_tile: bool) -> Cost {
        if target_is_tile && to_tile >= 2 && to_tile <= self.0.len() - 2 && to_tile % 2 == 0 {
            // to_tile is not accessible (directly infront of a room)
            return 0;
        }
        if from_tile == to_tile {
            return 0;
        }
        let (cost, from_tile, to_tile) = if from_tile < to_tile {
            (to_tile - from_tile, from_tile + 1, to_tile)
        } else {
            (from_tile - to_tile, to_tile, from_tile - 1)
        };

        for s in &self.0[from_tile..=to_tile] {
            if s.is_some() {
                return 0;
            }
        }
        cost as Cost
    }
}

impl Display for Connections {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0 {
            match c {
                Some(amphipod) => write!(f, "{}", amphipod)?,
                None => write!(f, ".")?,
            }
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cave<const ROOM_SIZE: usize> {
    rooms: [Room<ROOM_SIZE>; 4],
    connections: Connections,
}

impl<const ROOM_SIZE: usize> Cave<ROOM_SIZE> {
    fn is_done(&self) -> bool {
        self.rooms.iter().all(|room| room.is_done())
    }

    fn generate_moves(&self) -> Option<Vec<(Cost, Self)>> {
        if self.is_done() {
            return None;
        }

        let mut moves = vec![];
        for i_room in 0..self.rooms.len() {
            let mut cave = *self;

            // see if anything can move from a path to a room
            for i_conn in 0..cave.connections.0.len() {
                let amphipod = if let Some(amphipod) = cave.connections.0[i_conn] {
                    amphipod
                } else {
                    continue;
                };

                let access_cost = self.connections.can_access_room_from_tile(i_conn, i_room);
                if access_cost > 0 && cave.rooms[i_room].accepts(amphipod) {
                    let mut move_ = cave;
                    move_.connections.0[i_conn] = None;
                    let move_in_cost = move_.rooms[i_room].accept(amphipod);
                    moves.push(((access_cost + move_in_cost) * amphipod.cost(), move_));
                }
            }

            // All things that require to move something out of a room
            let (move_out_cost, amphipod) =
                if let Some((move_out_cost, amphipod)) = cave.rooms[i_room].take() {
                    (move_out_cost, amphipod)
                } else {
                    continue;
                };

            for j_room in 0..self.rooms.len() {
                if i_room == j_room {
                    continue;
                }

                // see if anything can move from room to room
                let access_cost = self.connections.can_access_room_from_room(i_room, j_room);
                if access_cost > 0 && cave.rooms[j_room].accepts(amphipod) {
                    let mut move_ = cave;
                    let move_in_cost = move_.rooms[j_room].accept(amphipod);
                    moves.push((
                        (move_out_cost + access_cost + move_in_cost) * amphipod.cost(),
                        move_,
                    ));
                }
            }

            // see if anything can move out of a room into a free spot
            for i_conn in 0..cave.connections.0.len() {
                let access_cost = self.connections.can_acccess_tile_from_room(i_room, i_conn);
                if access_cost > 0 {
                    let mut move_ = cave;
                    move_.connections.0[i_conn] = Some(amphipod);
                    moves.push(((move_out_cost + access_cost) * amphipod.cost(), move_));
                }
            }
        }

        Some(moves)
    }
}

impl<const ROOM_SIZE: usize> Display for Cave<ROOM_SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#############")?;
        writeln!(f, "#{}#", self.connections)?;

        for i in 0..ROOM_SIZE {
            write!(f, "###")?;
            for room in &self.rooms {
                match room.slots[i] {
                    Some(amphipod) => write!(f, "{}", amphipod)?,
                    None => write!(f, ".")?,
                }
                write!(f, "#")?;
            }
            writeln!(f, "##")?;
        }
        write!(f, "  #########  ")
    }
}

fn parse<const ROOM_SIZE: usize>(reader: Input) -> Cave<ROOM_SIZE> {
    let mut s = reader
        .lines()
        .map(|line| line.unwrap().into_bytes())
        .skip(1);

    let mut connections = Connections::default();
    for (i, a) in s
        .next()
        .unwrap()
        .into_iter()
        .filter(|x| (b'A'..=b'D').contains(x) || *x == b'.')
        .enumerate()
    {
        connections.0[i] = Amphipod::parse(a);
    }

    let lines: Vec<_> = s.take(ROOM_SIZE).collect();

    let mut rooms = Vec::new();
    for i in 0..4 {
        let room = Room {
            id: Amphipod::from_index(i).unwrap(),
            slots: lines
                .iter()
                .map(|line| Amphipod::parse(line[3 + 2 * i]))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        };
        rooms.push(room);
    }
    let rooms = rooms.as_slice().try_into().unwrap();

    Cave { rooms, connections }
}

#[derive(Debug, PartialEq, Eq)]
struct Element<T: Eq> {
    cost: Cost,
    item: T,
}

impl<T: Eq> Element<T> {
    fn new(cost: Cost, item: T) -> Self {
        Self { cost, item }
    }
}

impl<T: Eq> PartialOrd for Element<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Ord for Element<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn solve<const ROOM_SIZE: usize>(cave: Cave<ROOM_SIZE>) -> Cost {
    let mut candidates = BinaryHeap::with_capacity(128);
    candidates.push(Element::new(0, cave));

    let mut seen = HashSet::new();

    while let Some(element) = candidates.pop() {
        if element.item.is_done() {
            return element.cost;
        }

        let moves = element.item.generate_moves().unwrap();
        for (cost, new_cave) in moves {
            if seen.insert(new_cave) {
                candidates.push(Element::new(element.cost + cost, new_cave));
            }
        }
    }
    0
}

pub fn part1(reader: Input) -> anyhow::Result<Cost> {
    Ok(solve(parse::<2>(reader)))
}

pub fn part2(reader: Input) -> anyhow::Result<Cost> {
    let small_cave = parse::<2>(reader);

    let middle: [[Amphipod; 2]; 4] = [
        [Amphipod::Desert, Amphipod::Desert],
        [Amphipod::Copper, Amphipod::Bronze],
        [Amphipod::Bronze, Amphipod::Amber],
        [Amphipod::Amber, Amphipod::Copper],
    ];

    let rooms: Vec<_> = small_cave
        .rooms
        .into_iter()
        .zip(middle)
        .map(|(room, mid)| Room {
            id: room.id,
            slots: [room.slots[0], Some(mid[0]), Some(mid[1]), room.slots[1]],
        })
        .collect();
    let rooms = rooms.as_slice().try_into().unwrap();

    let cave = Cave {
        connections: small_cave.connections,
        rooms,
    };

    // let cave = parse::<4>(reader);
    Ok(solve(cave))
}

pub fn main() {
    aoc2021::cli::run(part1, part2).unwrap();
}
