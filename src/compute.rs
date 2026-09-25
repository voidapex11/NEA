use crate::ant;
use crate::constants;
use std::cell::RefCell;
use std::rc::Rc;

//use pyo3::prelude::*;

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

        // work out sum
        // divide by sum
        //
}

#[cfg(test)]
mod tests {
        use core::f32;

        use super::*;
        use rand;

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
}
