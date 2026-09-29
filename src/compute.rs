use crate::ant;
use crate::constants;
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

//use pyo3::prelude::*;

/// rounds off floating point numbers to the nearest 1/2^16
/// to prevent floating point errors in tests
fn round_float(x:f64) -> f64 {
        let factor = 2.0_f64.powi(16);
        (x* factor).round() / factor
}


/// validate if a point is out of bounds
fn is_oob(x: f64, y: f64) -> bool {
        if x < 0. || (constants::DIMENTIONS[0] as f64) < x {
                return true;
        }
        if y < 0. || (constants::DIMENTIONS[1] as f64) < y {
                return true;
        }
        false
}

/// ticks the pathfinding for a vector of ants
fn compute_ant_directions(
        mut ants: Vec<ant::Ant>,
        old: [[f64; constants::DIMENTIONS[0] as usize];
                constants::DIMENTIONS[1] as usize],
) -> Vec<ant::Ant> {
        let binding = ants.clone();
        let ants_with_points: std::iter::Zip<
                std::slice::Iter<'_, ant::Ant>,
                std::iter::Repeat<[[f64; 3]; 49]>,
        > = binding
                .iter()
                .zip(std::iter::repeat(constants::PATHING_POINTS));
        let ant_point_pairs = ants_with_points.map(|pair| {
                let (ant, points) = pair;
                std::iter::repeat(ant).zip(points)
        });
        let filtered_ant_point_pairs =
                ant_point_pairs.map(|ant_iter| {
                        ant_iter.filter(|pair| {
                                let (ant, p) = pair;
                                let (p_x, p_y, _) =
                                        (p[0], p[1], p[2]);
                                !is_oob(p_x + ant.x, p_y + ant.y)
                        })
                });
        let pairs_absolute_value_cords = filtered_ant_point_pairs
                .map(|ant_iter| {
                        ant_iter.map(|pair| {
                                let (ant, p) = pair;
                                (
                                        ant,
                                        [
                                                ant.x + p[0],
                                                ant.y + p[1],
                                                p[2],
                                        ],
                                )
                        })
                });

        let pairs_with_sum =
                pairs_absolute_value_cords.map(|ant_iter| {
                        (
                                ant_iter.clone(),
                                ant_iter.map(|pair| pair.1[2])
                                        .sum::<f64>(),
                        )
                });
        let ajusted_values =
                pairs_with_sum.map(|ant_iter_w_sum| {
                        let (ant_iter, s) = ant_iter_w_sum;
                        ant_iter.zip(std::iter::repeat(s)).map(
                                |pair_w_sum| {
                                        let (pair, sum) = pair_w_sum;
                                        let (ant, p) = pair;
                                        let dy = p[1]-ant.y;
                                        let dx = p[0]-ant.x;
                                        let mut angle:f64 = 0.0;
                                        if dy !=0.0 {
                                                angle = (dy/dx).atan();   
                                        }
                                        
                                        (
                                                angle.cos(),
                                                angle.sin(),
                                                p[2] / sum * old[p[0].round()
                                                as usize]
                                                [p[1].round()
                                                        as usize])
                                },
                        )
                });

        let velocity_vectors = ajusted_values.map(|ant_iter| {
                let sx = Rc::new(RefCell::new(0.));
                let sy = Rc::new(RefCell::new(0.));
                ant_iter.for_each(|triplet| {
                        let (dx, dy, magnitude) = triplet;
                        *sx.borrow_mut() += dx * magnitude;
                        *sy.borrow_mut() += dy * magnitude;
                });

                let angle = (*sy.borrow() / *sx.borrow()).atan();
                (angle.cos(), angle.sin())
        });
        ants.iter_mut()
                .zip(velocity_vectors)
                .map(|pair| {
                        let (ant, (vx, vy)) = pair;

                        if !vx.is_nan() {
                                ant.vx = vx;
                        }
                        if !vy.is_nan() {
                                ant.vy = vy;
                        }
                        ant.clone()
                })
                .collect::<Vec<ant::Ant>>()
}

fn move_ant_by_n(
        mut ant: ant::Ant,
        walls: [[bool; constants::DIMENTIONS[0] as usize];
                constants::DIMENTIONS[1] as usize],
        mut n: f64,
) -> ant::Ant {
        if n<0.0 {
                panic!("n < 0");
        }
        while 0.9 < n {
                ant = move_ant_by_n(ant, walls, 0.9);
                n -= 0.9;
        }

        let new_x = ant.x + ant.vx * n;
        let new_y = ant.y + ant.vy * n;

        let x_ok = !(walls[new_x.floor() as usize]
                [ant.y.floor() as usize]
                || is_oob(new_x.floor(), ant.y.floor()));
        let y_ok = !(walls[ant.x.floor() as usize]
                [new_y.floor() as usize]
                || is_oob(ant.x.floor(), new_y.floor()));
        let end_ok = !(walls[new_x.floor() as usize]
                [new_y.floor() as usize]
                || is_oob(new_x.floor(), new_y.floor()));

        if x_ok && y_ok && end_ok {
                ant.x = new_x;
                ant.y = new_y;
                return ant.clone();
        }
        let mut dx = f64::NAN;
        if ant.vx.is_sign_negative() {
                dx = ant.x - ant.x.floor();
        } else {
                dx = ant.x.ceil() - ant.x;
        };
        let mut dy = f64::NAN;
        if ant.vy.is_sign_negative() {
                dy = ant.y - ant.y.floor();
        } else {
                dy = ant.y.ceil() - ant.y;
        };
        let tx = dx / ant.vx.abs();
        let ty = dy / ant.vy.abs();

        let new_n = (tx.min(ty)).max(tx.min(ty) + f64::EPSILON);

        let new_x = ant.x + ant.vx * new_n;
        let new_y = ant.y + ant.vy * new_n;
        ant.x = new_x;
        ant.y = new_y;
        let remaining_n = n - new_n;
        if !x_ok && round_float(tx) <= round_float(ty) {
                ant.vx *= -1.0;
        };
        if !y_ok && round_float(tx) >= round_float(ty) {
                ant.vy *= -1.0;
        };
        if remaining_n.abs()> 2.0_f64.powi(-16) {
                ant = move_ant_by_n(
                        dbg!(ant),
                        walls,
                        dbg!(remaining_n),
                );
        }

        ant.clone()
}

fn move_ants_by_n(
        mut ants: Vec<ant::Ant>,
        walls: [[bool; constants::DIMENTIONS[0] as usize];
                constants::DIMENTIONS[1] as usize],
        n: f64,
) -> Vec<ant::Ant> {
        let new_cords = ants
                .iter_mut()
                .map(|ant| move_ant_by_n(ant.clone(), walls, n));
        new_cords.collect::<Vec<ant::Ant>>()
}

#[cfg(test)]
mod tests {
        use core::f64;

        use super::*;

        #[test]
        fn test_not_oob() {
                assert_eq!(is_oob(1., 2.), false);
        }

        #[test]
        fn test_boundary_oob() {
                assert_eq!(is_oob(0., 0.), false);
        }

        #[test]
        fn test_is_oob() {
                assert_eq!(is_oob(-1., 0.), true)
        }

        #[test]
        fn compute_ant_directions_valid() {
                let mut old = [[0.0; 96]; 60];
                old[40 + 1][40 + 1] = 100.0;
                //old[40][41] = 500.0;
                let result = compute_ant_directions(
                        vec![ant::Ant {
                                x: 40.,
                                y: 40.,
                                vx: 0.,
                                vy: 0.,
                        }],
                        old,
                );
                assert!(f64::abs(
                        result[0].vx
                                - std::f64::consts::SQRT_2 / 2.0
                ) < 0.01);
                assert!(f64::abs(
                        result[0].vy
                                - std::f64::consts::SQRT_2 / 2.0
                ) < 0.01);
        }

        #[test]
        fn move_unobstructed() {
                let mut walls = [[false; 96]; 60];
                let result = move_ant_by_n(
                        ant::Ant {
                                x: 0.0,
                                y: 0.0,
                                vx: 3.0_f64.sqrt() / 2.0,
                                vy: 0.5,
                        },
                        walls,
                        2.0,
                );
                assert_eq!(result.x, result.vx * 2.0);
                assert_eq!(result.y, result.vy * 2.0)
        }

        #[test]
        fn move_obstructed_box() {
                let mut walls = [[false; 96]; 60];
                walls[0][0] = true;
                walls[0][1] = true;
                walls[0][2] = true;
                walls[1][2] = true;
                walls[1][0] = true;
                walls[2][0] = true;
                walls[2][1] = true;
                walls[2][2] = true;
                let n = 3.0;
                let angle = f64::consts::PI / 180.0 * (90.0);
                let ant = ant::Ant {
                        x: 1.5,
                        y: 1.5,
                        vx: angle.cos(),
                        vy: angle.sin(),
                };
                let result =
                        dbg!(move_ant_by_n(ant.clone(), walls, n));
                assert_eq!(round_float(result.x), 1.5);
                assert_eq!(round_float(result.y), 1.5);
                assert_eq!(round_float(result.vy), round_float(-ant.vy));
                assert_eq!(round_float(result.vx), round_float(ant.vx));
        }

        #[test]
        fn move_bounce_corner() {
                let mut walls = [[false; 96]; 60];
                walls[0][0] = true;
                walls[0][1] = true;
                walls[0][2] = true;
                walls[1][0] = true;
                walls[2][0] = true;
                let n = f64::consts::SQRT_2*5.0/2.0;
                let angle = f64::consts::PI / 180.0 * (-135.0);
                let ant = ant::Ant {
                        x: 2.0,
                        y: 3.0,
                        vx: angle.cos(),
                        vy: angle.sin(),
                };
                let result =
                        dbg!(move_ant_by_n(dbg!(ant.clone()), walls, n));
                assert_eq!(round_float(result.x),1.5);
                assert_eq!(round_float(result.y),1.5);
                assert_eq!(round_float(result.vy), round_float(-ant.vy));
                assert_eq!(round_float(result.vx), round_float(ant.vx));

        }
}
