use std::io;
use std::io::Read;
use std::mem::swap;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Copy, Clone, PartialEq, Eq)]
struct Edge {
    p1: Point,
    p2: Point,
}
impl Point {
    fn manhattan_norm(&self) -> i32 {
        return self.x.abs() + self.y.abs();
    }
    fn manhattan_dist(&self, other: Point) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }
}
impl Edge {
    fn interval_intersect(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let mut a1 = x1;
        let mut b1 = y1;
        let mut a2 = x2;
        let mut b2 = y2;
        if y1 < x1 {
            swap(&mut a1, &mut b1);
        }
        if y2 < x2 {
            swap(&mut a2, &mut b2);
        }
        return (a1 >= a2 && a1 <= b2)
            || (b1 >= a2 && b1 <= b2)
            || (a2 >= a1 && a2 <= b1)
            || (b2 >= a1 && b2 <= b1);
    }
    fn intersect(&self, other: Edge) -> bool {
        return Edge::interval_intersect(self.p1.x, self.p2.x, other.p1.x, other.p2.x)
            && Edge::interval_intersect(self.p1.y, self.p2.y, other.p1.y, other.p2.y);
    }
}
fn build_wires(contents: String) -> Vec<Vec<Edge>> {
    let mut wires: Vec<Vec<Edge>> = vec![];
    let wire_strs = contents.split("\n");
    for wireStr in wire_strs {
        let edge_strs = wireStr.split(",");
        let mut old_point = Point { x: 0, y: 0 };
        let mut wire: Vec<Edge> = vec![];
        for edgeStr in edge_strs {
            let (code, diststr) = edgeStr.trim().split_at(1);
            let mut new_point: Point = Point { x: 0, y: 0 };
            let dist: i32 = diststr.parse::<i32>().unwrap();
            match code {
                "U" => {
                    new_point = Point {
                        x: old_point.x,
                        y: old_point.y + dist,
                    }
                }
                "D" => {
                    new_point = Point {
                        x: old_point.x,
                        y: old_point.y - dist,
                    }
                }
                "L" => {
                    new_point = Point {
                        x: old_point.x - dist,
                        y: old_point.y,
                    }
                }
                "R" => {
                    new_point = Point {
                        x: old_point.x + dist,
                        y: old_point.y,
                    }
                }
                _ => panic!("Something unexpected was encountered while parsing wires."),
            }
            let e: Edge = Edge {
                p1: old_point,
                p2: new_point,
            };
            wire.push(e);
            old_point = new_point;
        }
        wires.push(wire);
    }
    wires
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let wires = build_wires(input);
    let mut manhattan_distances: Vec<i32> = vec![];
    let mut wire_distances: Vec<i32> = vec![];
    //Iterate pairwise through all wires and compare their edges.
    for i in 0..wires.len() {
        let mut i_wire_cost = 0;
        for j in 0..wires[i].len() {
            let mut k_wire_cost = 0;
            for k in 0..wires.len() {
                if i != k {
                    for l in 0..wires[k].len() {
                        if wires[i][j].intersect(wires[k][l]) {
                            let mut inter = Point {
                                x: wires[i][j].p1.x,
                                y: wires[k][l].p1.y,
                            };
                            if wires[i][j].p1.x != wires[i][j].p2.x {
                                inter = Point {
                                    x: wires[k][l].p1.x,
                                    y: wires[i][j].p1.y,
                                };
                            }
                            manhattan_distances.push(inter.manhattan_norm());
                            wire_distances.push(
                                i_wire_cost
                                    + wires[i][j].p1.manhattan_dist(inter)
                                    + k_wire_cost
                                    + wires[k][l].p1.manhattan_dist(inter),
                            );
                        }
                        k_wire_cost += wires[k][l].p1.manhattan_dist(wires[k][l].p2);
                    }
                }
            }
            i_wire_cost += wires[i][j].p1.manhattan_dist(wires[i][j].p2);
        }
    }
    let part1 = manhattan_distances
        .iter()
        .filter(|x| **x != 0)
        .min()
        .unwrap();
    let part2 = wire_distances.iter().filter(|x| **x != 0).min().unwrap();
    println!("Part 1: {}\nPart 2: {}", part1, part2);
}
