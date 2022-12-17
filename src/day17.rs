use std::collections::HashMap;
use std::fs;

pub fn solve(input_file: String, part: u8) {
    let contents = fs::read_to_string(&input_file).expect("Could not read input_file");
    let first_line = contents.lines().next().unwrap();

    let gas_directions: Vec<GasJetDirection> = first_line
        .chars()
        .map(|c| GasJetDirection::try_from(c).expect("Input has invalid characters"))
        .collect();

    let shapes = [
        Shape::Line,
        Shape::Cross,
        Shape::L,
        Shape::Tower,
        Shape::Square,
    ];

    println!(
        "There are {0} shapes and a {1}-length cycle of jet",
        shapes.len(),
        gas_directions.len()
    );
    let cycle_length = shapes.len() * gas_directions.len();

    let mut w = World::default();
    let mut time = 0;

    let rock_count = if part == 1 { 2022 } else { 1000000000000 };
    let mut state_signature = HashMap::<StateSignature, (usize, usize)>::new();

    let mut object_id = 0;
    let mut squeezed_height = 0;
    while object_id < rock_count {
        if object_id % 1000 == 0 {
            println!("{object_id}th rock falling");
        }
        let signature = StateSignature {
            top_signature: w.top_signature(),
            shape_index: object_id % shapes.len(),
            gas_jet_index: object_id % gas_directions.len(),
        };
        match state_signature.get(&signature) {
            None => {
                state_signature.insert(signature, (object_id, w.max_height()));
            }
            Some((cycle_start, height)) => {
                let cycle = object_id - cycle_start;
                let remaining_cycles = (rock_count - object_id) / cycle;
                if remaining_cycles > 0 {
                    println!("We found a cycle at iteration {object_id}");
                    println!("There is a cycle of size {cycle}, let's skip a few iterations");
                    let jump = remaining_cycles * cycle + object_id;
                    squeezed_height = (w.max_height() - height) * remaining_cycles;
                    println!("Will jump directly to iteration {jump}");
                    object_id = jump;
                }
            }
        }
        let shape = shapes[object_id % shapes.len()];
        let mut s = ShapeObject {
            shape,
            bottom_left_edge: w.spawn_point(),
        };
        loop {
            // print_world(w, Some(s));
            let jet = gas_directions[time % gas_directions.len()];
            s = s.gas_jet(jet, w);
            let attempt_position = s.move_down();
            if attempt_position.is_none() || w.has_collision(attempt_position.unwrap()) {
                w.land(s);
                time += 1;
                break;
            } else {
                s = attempt_position.unwrap();
            }
            time += 1;
        }
        // print_world(w, None);
        //dbg!(w.max_height());
        object_id += 1;
    }
    dbg!(w.max_height() + squeezed_height);
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Line,
    Cross,
    L,
    Tower,
    Square,
}

const COLUMN_COUNT: usize = 7;
const MAX_HEIGHT: usize = 7 * 10091 * 4 + 10;

#[derive(Debug, Clone, Copy)]
struct World {
    columns: [[bool; MAX_HEIGHT]; COLUMN_COUNT],
    column_heights: [usize; COLUMN_COUNT],
}

fn print_world(world: World, object: Option<ShapeObject>) {
    let coords = object.map_or(vec![], ShapeObject::coordinates);
    let height_start = world
        .column_heights
        .iter()
        .max()
        .expect("World has at least one column");
    for height in (0..height_start + 7).rev() {
        print!("|");
        for column in 0..COLUMN_COUNT {
            if coords.iter().any(|&p| p == Point { column, height }) {
                print!("@");
            } else if world.columns[column][height] {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!("|");
    }
    println!("+-------+");
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct StateSignature {
    top_signature: [usize; COLUMN_COUNT],
    shape_index: usize,
    gas_jet_index: usize,
}

impl World {
    fn default() -> World {
        World {
            columns: [[false; MAX_HEIGHT]; COLUMN_COUNT],
            column_heights: [0; COLUMN_COUNT],
        }
    }

    fn top_signature(self) -> [usize; COLUMN_COUNT] {
        let h = self.max_height();
        let mut signature = [0; COLUMN_COUNT];
        for column in 0..COLUMN_COUNT {
            signature[column] = h - self.column_heights[column];
        }
        signature
    }

    fn max_height(self) -> usize {
        *self
            .column_heights
            .iter()
            .max()
            .expect("World has at least one column")
    }
    fn spawn_point(self) -> Point {
        let column = 2;
        Point {
            column,
            height: self.max_height() + 3,
        }
    }
    fn land(&mut self, rock: ShapeObject) {
        // sediment a shape and return the new world
        for p in rock.coordinates() {
            self.columns[p.column][p.height] = true;
        }
        for column in 0..COLUMN_COUNT {
            // TODO: we can do better because we know the max piece size and the previous height
            let mut height = 0;
            for (h, &b) in self.columns[column].iter().enumerate() {
                if b {
                    height = h;
                }
            }
            self.column_heights[column] = height + 1;
        }
    }

    fn has_collision(self, rock: ShapeObject) -> bool {
        rock.coordinates()
            .iter()
            .any(|p| self.columns[p.column][p.height])
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    column: usize,
    height: usize,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({0},{1})", self.column, self.height)
    }
}

#[derive(Clone, Copy)]
struct ShapeObject {
    shape: Shape,
    bottom_left_edge: Point,
}

impl std::fmt::Debug for ShapeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{0:?} BT: {1:?}", self.shape, self.bottom_left_edge)
    }
}

#[derive(Debug, Clone, Copy)]
enum GasJetDirection {
    Left,
    Right,
}

impl TryFrom<char> for GasJetDirection {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, <Self as TryFrom<char>>::Error> {
        match c {
            '>' => Ok(GasJetDirection::Right),
            '<' => Ok(GasJetDirection::Left),
            _ => Err("Only accept < and > characters"),
        }
    }
}

impl ShapeObject {
    fn gas_jet(self, direction: GasJetDirection, world: World) -> ShapeObject {
        let candidate_position = match direction {
            GasJetDirection::Left => self.move_left(),
            GasJetDirection::Right => self.move_right(),
        };
        match candidate_position {
            None => self, // we hurt a wall on the side
            Some(object) => {
                if world.has_collision(object) {
                    self
                } else {
                    object
                }
            }
        }
    }

    fn coordinates(self) -> Vec<Point> {
        // gives coordinates of all elements of this shape
        let mut coords = vec![];
        match self.shape {
            Shape::Line => {
                for column in self.bottom_left_edge.column..self.bottom_left_edge.column + 4 {
                    coords.push(Point {
                        column,
                        height: self.bottom_left_edge.height,
                    });
                }
            }
            Shape::Cross => {
                coords.push(Point {
                    column: self.bottom_left_edge.column,
                    height: self.bottom_left_edge.height + 1,
                });
                coords.push(Point {
                    column: self.bottom_left_edge.column + 1,
                    height: self.bottom_left_edge.height + 2,
                });
                coords.push(Point {
                    column: self.bottom_left_edge.column + 1,
                    height: self.bottom_left_edge.height + 1,
                });
                coords.push(Point {
                    column: self.bottom_left_edge.column + 1,
                    height: self.bottom_left_edge.height,
                });
                coords.push(Point {
                    column: self.bottom_left_edge.column + 2,
                    height: self.bottom_left_edge.height + 1,
                });
            }
            Shape::L => {
                coords.push(Point {
                    column: self.bottom_left_edge.column + 2,
                    height: self.bottom_left_edge.height + 2,
                });
                coords.push(Point {
                    column: self.bottom_left_edge.column + 2,
                    height: self.bottom_left_edge.height + 1,
                });
                coords.push(Point {
                    column: self.bottom_left_edge.column,
                    height: self.bottom_left_edge.height,
                });
                coords.push(Point {
                    column: self.bottom_left_edge.column + 1,
                    height: self.bottom_left_edge.height,
                });
                coords.push(Point {
                    column: self.bottom_left_edge.column + 2,
                    height: self.bottom_left_edge.height,
                });
            }
            Shape::Tower => {
                for height in self.bottom_left_edge.height..self.bottom_left_edge.height + 4 {
                    coords.push(Point {
                        column: self.bottom_left_edge.column,
                        height,
                    });
                }
            }
            Shape::Square => {
                coords.push(Point {
                    column: self.bottom_left_edge.column,
                    height: self.bottom_left_edge.height + 1,
                });
                coords.push(Point {
                    column: self.bottom_left_edge.column + 1,
                    height: self.bottom_left_edge.height + 1,
                });
                coords.push(Point {
                    column: self.bottom_left_edge.column,
                    height: self.bottom_left_edge.height,
                });
                coords.push(Point {
                    column: self.bottom_left_edge.column + 1,
                    height: self.bottom_left_edge.height,
                });
            }
        }
        coords
    }

    fn move_left(self) -> Option<ShapeObject> {
        if self.bottom_left_edge.column == 0 {
            return None;
        }
        Some(ShapeObject {
            bottom_left_edge: Point {
                column: self.bottom_left_edge.column - 1,
                ..self.bottom_left_edge
            },
            ..self
        })
    }

    fn move_right(self) -> Option<ShapeObject> {
        let s = ShapeObject {
            bottom_left_edge: Point {
                column: self.bottom_left_edge.column + 1,
                ..self.bottom_left_edge
            },
            ..self
        };
        if s.coordinates().iter().any(|p| p.column >= COLUMN_COUNT) {
            None
        } else {
            Some(s)
        }
    }

    fn move_down(self) -> Option<ShapeObject> {
        if self.bottom_left_edge.height == 0 {
            return None;
        }
        Some(ShapeObject {
            bottom_left_edge: Point {
                height: self.bottom_left_edge.height - 1,
                ..self.bottom_left_edge
            },
            ..self
        })
    }
}
