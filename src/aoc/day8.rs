use crate::Puzzle;
use typenum;
use kd_tree::{KdTree};
use std::collections::{HashSet};

// Day8 implements day 8 of AoC 2025, as uploaded at https://adventofcode.com/2025/day/8. 
pub struct Day8;

// given 1000 sets of coordinates in 3D space, work out how close each pair is to one another, and take the closest pair
// part 1) join the 10 closest pairs the size of the 3 largest circuits multiplied together
// part 2) (my guess) how many pairs you need to consider before every box is connected?

// we don't really want to do 1M operations every time we work out how far away each pair is. what classic tricks are there for this?
// [... recalling 2nd year further graphics ...]
// - quadtrees are the solution for this in 2D space, I guess we do something similar (oct-tree?) in 3D space
//     - yes, octrees are a thing https://en.wikipedia.org/wiki/Octree
// we could also partition space e.g. group all of 0-500, 500-1000 (x,y,z), then each closest pair we'd only need to consider within each square and its 26 neighbours
// - https://en.wikipedia.org/wiki/K-d_tree#Nearest_neighbour_search ah k-d trees ring a bell, guess it's a good point
//   I'm sure I'd learn a lot from implementing this from scratch but when there's a crate just there... I may as well learn from just the code anyway.
// (I'm very glad I did this, it took a couple of hours to solve this one)
#[derive(Clone, Copy, Debug)]
struct Point {
    i: usize,
    x: usize,
    y: usize,
    z: usize,
}

impl kd_tree::KdPoint for Point {
    // I would use usize here but the kd-tree library I'm using doesn't handle unsigned integer subtraction so underflows and panics
    type Scalar = isize;
    type Dim = typenum::U3;
    fn at(&self, k: usize) -> isize {
        match k {
            0 => self.x.try_into().unwrap(),
            1 => self.y.try_into().unwrap(),
            2 => self.z.try_into().unwrap(),
            _ => panic!("attempted to access dimension {k} of 3D point"),
        }
    }
}

fn parse_input(input: &str) -> (KdTree<Point>, Vec<Point>) {
    let mut points = Vec::<Point>::new();
    for (i, mut coords) in input.lines()
                      .map(|line| line.trim()
                                            .split(',')
                                            .map(|x| x.parse::<usize>().unwrap())
                      ).enumerate() {
        let p = Point{
            i: i,
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        };

        let next = coords.next();

        match next {
            Some(_) => panic!("more than 3 values on line {i} of input"),
            None => {}, // I'm sure this isn't idiomatic but it works
        }

        points.push(p);
    }

    return (KdTree::build(points.clone()), points);
}

fn square_diff(v1: usize, v2: usize) -> usize {
    if v1 > v2 {
        return (v1 - v2) * (v1 - v2);
    } else {
        return (v2 - v1) * (v2 - v1);
    }
}

fn square_distance(p1: Point, p2: Point) -> usize {
    return square_diff(p1.x, p2.x) + square_diff(p1.y, p2.y) + square_diff(p1.z, p2.z); 
}

#[derive(Debug, Clone, Copy)]
struct PointDist {
    i_point_a: usize,
    i_point_b: usize,
    dist: isize, // actually the squared distance but this is way easier to type
}

fn initialise_circuits(length: usize) -> Vec::<HashSet<usize>> {
    let mut circuits = Vec::<HashSet<usize>>::new();
    
    for i in 0..length {
        let mut set = HashSet::<usize>::new();
        set.insert(i);
        circuits.push(set);
    }

    return circuits
}

// n is 0-indexed, i.e. 0 returns the closest point, 1 returns the next closest point 
fn nth_closest(tree: &KdTree<Point>, point: &Point, n: usize) -> PointDist {
    // n=0 actually returns the second closest point in the tree (since the closest one is always that same point)
    let nearest = tree.nearests(point, n + 2)[n+1];
    return PointDist{
        i_point_a: point.i,
        i_point_b: nearest.item.i,
        dist: nearest.squared_distance,
    }
}

fn insert_next(tree: &KdTree<Point>, distances: &mut Vec<PointDist>, point: &Point, n: usize) {
    // get the next closest point...
    let next_point = nth_closest(tree, point, n);

    //println!("inserting new distance {next_point:?}");

    // and insert it at the correct place in the array. it's ordered with smallest last, so we need to iterate over the array from the end
    for i in (0..distances.len()).rev() {
        if distances[i].dist > next_point.dist {
            // we hit the last value in the array further away than the current one, so insert one index after this
            distances.insert(i+1, next_point);
            return;
        }
    }
    
    // we hit the start of the array and no value was greater than ours. we need to push to the front
    distances.insert(0, next_point);
}

fn part1(tree: KdTree<Point>, points: Vec<Point>) -> usize {
    // unfortunately the library I'm using doesn't support deleting points from a tree once it's already constructed
    // however we can use a cheeky workaround which isn't the most efficient, but is more than good enough
    //
    // we can get the closest point to every point, find the closest distance of those, then increment the index for the two points (since the closest one's neighbour will be the same point unless there are two at the same distance)
    // i.e. we need two vecs, one of the current index of the nth neighbour, and one with the current nth neighbour point
    // then we also need to keep track of the circuits - easiest way of doing this is going to be having a map of points to the circuits they belong to as a pointer to a set
    // nb. indices must start at 1 because every point's closest point is otherwise itself. I don't think this even assumes there are no duplicates.
    let mut position_map = tree.iter().map(|_| 0_usize).collect::<Vec<usize>>();
    let mut distances = tree.iter().map(|point| nth_closest(&tree, point, 0)).collect::<Vec<PointDist>>(); 
    
    let mut circuits = initialise_circuits(tree.len());
    let mut circuit_map = circuits.iter().enumerate().map(|(i, _)| i).collect::<Vec<usize>>();
    // we now have:
    // position_map: a vec of "0" (i.e. the index of the next closest non-self position) for each position in the tree
    // distances: a vec of the index of the point (i.e. the point's i value) together with the distance to its next closest not-yet-added point, and its next closest not-yet-added point
    // circuits: a vec which contains the circuit for each point (as a lookup from point index to a pointer to the set containing it). at the moment this is just {i} for each index i
    // circuit_map: a map of the point index to the circuit index. this is the map we update when we combine circuits together. at the moment this is just i for each index i

    // the trick is now that we can just iterate on this - we need to only sort the distances array in _descending_ order of distance, then pop off the end
    // this gives us the indices of both points, which we can then merge into one another and increment both indices
    // (we could be more efficient and store the distances only for the lower member of each pair but oh well)
    // careful: we need to make sure we increment _both_ members of the pair, so we might need to inspect back a little bit further from the back of the array if there are any ties.
    // these should be pretty rare in a 3D space (depending how points are distributed, hopefully not a grid...)
    // given there's no discussion of how to handle ties in the problem description let's just say it's out of scope at least to start off with and hope it doesn't bite me for part 2
    
    // comparing b to a gives reversed (descending) order
    distances.sort_by(|a, b| b.dist.cmp(&a.dist));
    //println!("distances: {:?}", distances);
    for _ in 0..1000 {
        // cheeky trick assuming that there are no ties: we should be able to pop two points and get two that point to one another
        let first = distances.pop().unwrap();
        let second = distances.pop().unwrap();

        if first.i_point_a != second.i_point_b || first.i_point_b != second.i_point_a {
            panic!("popped two points which didn't refer to one another {first:?} {second:?}");
        }

        // we have our closest pair! now we need to insert the next new points for each, then connect them if we need to
        let i_a = first.i_point_a;
        let i_b = first.i_point_b;
        
        //println!("pairing {:?} with {:?} with distance {}", points[i_a], points[i_b], first.dist);

        // now fetch and insert the next closest point for each of the two 
        position_map[i_a] += 1;
        position_map[i_b] += 1;

        insert_next(&tree, &mut distances, &points[i_a], position_map[i_a]);
        insert_next(&tree, &mut distances, &points[i_b], position_map[i_b]);

        let c_i_a = circuit_map[i_a];
        let c_i_b = circuit_map[i_b];

        // if the two circuits are the same already, then we've added a redundant connection! no need to merge the circuits
        if c_i_a == c_i_b {
            continue;
        }

        // otherwise, we need to merge the two circuits into one 
        // arbitrarily merge the second point's circuit into the first's
        {
            // we need to clone c_b to iterate over it, because it could in theory be the same as c_a, even though it won't ever be in practice
            let c_b = circuits[c_i_b].clone();
            let c_a = &mut circuits[c_i_a];

            // update everything in the second circuit to belong as if it's in the first one, and add each of those values to the first circuit
            for old_circuit_value in c_b.iter() {
                c_a.insert(*old_circuit_value);
                circuit_map[*old_circuit_value] = c_i_a;
            }
        }

        // now empty out the other set - we don't want it to count anymore 
        {
            let c_b = &mut circuits[c_i_b];
            c_b.clear();
        }

        // if I were writing C this is where I'd free the old circuit
    }


    // we now have some number of sets, and we want the three largest ones
    // all of the ones which are no longer connected are now empty, so they will have length 0
    let mut circuit_sizes = circuits.iter().map(|s| s.len()).collect::<Vec<usize>>();
    circuit_sizes.sort();
    return circuit_sizes.iter().rev().take(3).product();
}

// it's late and I'm tired so I'm just copying and modifying my part1 for part 2 rather than deduplicating them (using a predicate over circuit_sizes)
fn part2(tree: KdTree<Point>, points: Vec<Point>) -> usize {
    // unfortunately the library I'm using doesn't support deleting points from a tree once it's already constructed
    // however we can use a cheeky workaround which isn't the most efficient, but is more than good enough
    //
    // we can get the closest point to every point, find the closest distance of those, then increment the index for the two points (since the closest one's neighbour will be the same point unless there are two at the same distance)
    // i.e. we need two vecs, one of the current index of the nth neighbour, and one with the current nth neighbour point
    // then we also need to keep track of the circuits - easiest way of doing this is going to be having a map of points to the circuits they belong to as a pointer to a set
    // nb. indices must start at 1 because every point's closest point is otherwise itself. I don't think this even assumes there are no duplicates.
    let mut position_map = tree.iter().map(|_| 0_usize).collect::<Vec<usize>>();
    let mut distances = tree.iter().map(|point| nth_closest(&tree, point, 0)).collect::<Vec<PointDist>>(); 
    
    let mut circuits = initialise_circuits(tree.len());
    let mut circuit_map = circuits.iter().enumerate().map(|(i, _)| i).collect::<Vec<usize>>();
    let mut circuits_removed = 0;
    // we now have:
    // position_map: a vec of "0" (i.e. the index of the next closest non-self position) for each position in the tree
    // distances: a vec of the index of the point (i.e. the point's i value) together with the distance to its next closest not-yet-added point, and its next closest not-yet-added point
    // circuits: a vec which contains the circuit for each point (as a lookup from point index to a pointer to the set containing it). at the moment this is just {i} for each index i
    // circuit_map: a map of the point index to the circuit index. this is the map we update when we combine circuits together. at the moment this is just i for each index i

    // the trick is now that we can just iterate on this - we need to only sort the distances array in _descending_ order of distance, then pop off the end
    // this gives us the indices of both points, which we can then merge into one another and increment both indices
    // (we could be more efficient and store the distances only for the lower member of each pair but oh well)
    // careful: we need to make sure we increment _both_ members of the pair, so we might need to inspect back a little bit further from the back of the array if there are any ties.
    // these should be pretty rare in a 3D space (depending how points are distributed, hopefully not a grid...)
    // given there's no discussion of how to handle ties in the problem description let's just say it's out of scope at least to start off with and hope it doesn't bite me for part 2
    
    // comparing b to a gives reversed (descending) order
    distances.sort_by(|a, b| b.dist.cmp(&a.dist));
    //println!("distances: {:?}", distances);

    let mut point_a: Option<Point> = None;
    let mut point_b: Option<Point> = None;
    while circuits_removed < circuits.len() - 1 {
        // cheeky trick assuming that there are no ties: we should be able to pop two points and get two that point to one another
        let first = distances.pop().unwrap();
        let second = distances.pop().unwrap();

        if first.i_point_a != second.i_point_b || first.i_point_b != second.i_point_a {
            panic!("popped two points which didn't refer to one another {first:?} {second:?}");
        }

        // we have our closest pair! now we need to insert the next new points for each, then connect them if we need to
        let i_a = first.i_point_a;
        let i_b = first.i_point_b;
        
        //println!("pairing {:?} with {:?} with distance {}", points[i_a], points[i_b], first.dist);

        // now fetch and insert the next closest point for each of the two 
        position_map[i_a] += 1;
        position_map[i_b] += 1;

        insert_next(&tree, &mut distances, &points[i_a], position_map[i_a]);
        insert_next(&tree, &mut distances, &points[i_b], position_map[i_b]);

        let c_i_a = circuit_map[i_a];
        let c_i_b = circuit_map[i_b];

        // if the two circuits are the same already, then we've added a redundant connection! no need to merge the circuits
        if c_i_a == c_i_b {
            continue;
        }

        // otherwise, we need to merge the two circuits into one 
        // arbitrarily merge the second point's circuit into the first's
        {
            // we need to clone c_b to iterate over it, because it could in theory be the same as c_a, even though it won't ever be in practice
            let c_b = circuits[c_i_b].clone();
            let c_a = &mut circuits[c_i_a];

            // update everything in the second circuit to belong as if it's in the first one, and add each of those values to the first circuit
            for old_circuit_value in c_b.iter() {
                c_a.insert(*old_circuit_value);
                circuit_map[*old_circuit_value] = c_i_a;
            }

            point_a = Some(points[i_a]);
            point_b = Some(points[i_b]);
            circuits_removed += 1;
        }

        // now empty out the other set - we don't want it to count anymore 
        {
            let c_b = &mut circuits[c_i_b];
            c_b.clear();
        }

        // if I were writing C this is where I'd free the old circuit
    }
    
    println!("largest index was {}", position_map.iter().max().unwrap());

    // we now have our one big circuit, and we were keeping track of our last selected point. so just return the two x coords
    return match (point_a, point_b) {
        (Some(a), Some(b)) => a.x * b.x,
        _ => panic!("ended part 2 without two points (how? you should be in an infinite loop)")
    }
}
impl Puzzle for Day8 {
    fn part1(&self, input: &String) -> String {
        let (tree, points) = parse_input(input); 
        return part1(tree, points).to_string(); 
    }
    fn part2(&self, input: &String) -> String {
        let (tree, points) = parse_input(input); 
        return part2(tree, points).to_string(); 
    }
}

