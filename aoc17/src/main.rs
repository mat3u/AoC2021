use std::fmt::Display;

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct MovingPoint {
    position: Point,
    velocity: Point,
}

struct Area {
    lt: Point,
    br: Point,
}

impl Point {
    const ZERO: Point = Point { x: 0, y: 0 };

    fn offset(&self, d: &Point) -> Point {
        Point {
            x: self.x + d.x,
            y: self.y + d.y,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl MovingPoint {
    const DRAG: i32 = 1;

    fn next(&self) -> MovingPoint {
        MovingPoint {
            position: self.position.offset(&self.velocity),
            velocity: {
                let x = if self.velocity.x < 0 {
                    self.velocity.x + MovingPoint::DRAG
                } else if self.velocity.x > 0 {
                    self.velocity.x - MovingPoint::DRAG
                } else {
                    0
                };

                let y = self.velocity.y - 1;

                Point { x, y }
            },
        }
    }
}

impl Area {
    fn is_in(&self, point: &Point) -> bool {
        point.x >= self.lt.x && point.x <= self.br.x && point.y <= self.lt.y && point.y >= self.br.y
    }
}

fn main() {
    // let target = Area {
    //     lt: Point { x: 20, y: -5 },
    //     br: Point { x: 30, y: -10 },
    // };

    let target = Area {
        lt: Point { x: 143, y: -71 },
        br: Point { x: 177, y: -106 },
    };

    let N = 2000;

    let mut cnt = 0;
    let mut max_y = i32::MIN;

    for x in -200..200 {
        for y in -200..200 {
            let velocity = Point { x, y };
            let v2 = velocity.clone();

            let moving = MovingPoint {
                position: Point::ZERO.clone(),
                velocity: velocity,
            };

            let mut local_max_y = i32::MIN;

            let generated: Vec<Point> = (1..N)
                .scan(moving, |current, _| {
                    *current = current.next();

                    if current.position.y > local_max_y {
                        local_max_y = current.position.y;
                    }

                    Some(current.position.clone())
                })
                .filter(|v| target.is_in(&v))
                .collect();

            if !generated.is_empty() {
                print!("For V={}: ", v2);
                println!(" Ym = {}", local_max_y);

                cnt += 1;
            }

            if !generated.is_empty() && local_max_y > max_y {
                max_y = local_max_y;
            }
        }
    }

    println!("RESULT = {}", cnt);
}
