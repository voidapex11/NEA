use crate::ant;
use crate::constants;
use crate::wall;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc,Mutex};
use std::vec;
use pyo3::prelude::*;
use pyo3::types::PyInt;
use rayon::prelude::*;
use std::time::SystemTime;
/// rounds off floating point numbers to the nearest 1/2^16
/// to prevent floating point errors in tests
fn round_float(x: f64) -> f64 {
        let factor = 2.0_f64.powi(16);
        (x * factor).round() / factor
}

fn place_pheremones(
        ants: Vec<ant::Ant>,
        old: [[f64; constants::DIMENTIONS[0] as usize];
                constants::DIMENTIONS[1] as usize],
) -> [[f64; constants::DIMENTIONS[0] as usize];
           constants::DIMENTIONS[1] as usize] {
        let new = Arc::new(Mutex::new(old.clone()));
        ants.iter().for_each(|ant| {
                place_pheremone(ant, new.clone());
        });
        return *(new.lock().unwrap());
}

fn place_pheremone(
        ant: &ant::Ant,
        new: Arc<Mutex<[[f64; constants::DIMENTIONS[0] as usize];
                constants::DIMENTIONS[1] as usize]>>,
) {
        (*new.lock().unwrap())[ant.x.floor() as usize][ant.y.floor() as usize] +=
                ant.trail_conc;
}

/// validate if a point is out of bounds
pub fn is_oob(x: f64, y: f64) -> bool {
        // index of dimentions is correct, its this way because of how indexing works with type def.
        if x < 0. || (constants::DIMENTIONS[1]-1) < x.floor() as i32 {
                return true;
        }
        if y < 0. || (constants::DIMENTIONS[0]-1) < y.floor() as i32 {
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
        let ants_with_points = binding
                .par_iter()
                .zip(rayon::iter::repeat_n(
                        constants::PATHING_POINTS,
                        ants.clone().len())
        );

        let ant_point_pairs = ants_with_points.map(|pair| {
                let (ant, points) = pair;
                rayon::iter::repeat(ant).zip(points)
        });
        let filtered_ant_point_pairs =
                ant_point_pairs.map(|ant_iter| {
                        ant_iter.filter(|pair| {
                                let (ant, p) = pair;
                                let (p_x, p_y, _) =
                                        (p[0], p[1], p[2]);
                                !is_oob((p_x + ant.x).round(), (p_y + ant.y).round())
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
                        
                        ant_iter.map_with(
                                s,
                                |sum,pair| {
                                        let (ant, p) = pair;
                                        
                                        // correctly gets dy and dx
                                        let dy = p[1]-ant.y;
                                        let dx = p[0]-ant.x;

                                        if dy==0.0 && dx ==0.0 {
                                                return (0.0,0.0,p[2] / *sum * old[p[0]
                                                                as usize]
                                                                [p[1]as usize])
                                        }
                                        
                                        let mut angle:f64 = 0.0;

                                        if dx.is_normal() {
                                                angle = (dy/dx).atan();
                                                (
                                                angle.cos(),
                                                angle.sin(),
                                                p[2] / *sum * old[p[0]
                                                as usize]
                                                [p[1]
                                                        as usize]
                                                )
                                        } else if dx==0.0 {
                                                (
                                                        0.0,
                                                        dy.signum(),
                                                        p[2] / *sum * old[p[0]
                                                                as usize]
                                                                [p[1]as usize]
                                                )
                                        } else {
                                                (0.0,0.0,0.0)
                                        } 
                                },
                        ).filter(|p|(*p).2!=0.0)
                });

        let velocity_vectors = ajusted_values.map(|ant_iter| {
                let sx = Rc::new(RefCell::new(0.0));
                let sy = Rc::new(RefCell::new(0.0));
                ant_iter.collect::<Vec<(f64,f64,f64)>>().iter().for_each(|triplet| {
                        let (dx, dy, magnitude) = triplet;
                        if *magnitude==0.0{
                                return
                        }
                        let pending_change_x = dx * magnitude;
                        let pending_change_y = dy * magnitude;
                        {
                                let mut sx = sx.borrow_mut();
                                *sx += pending_change_x;
                        }
                        {
                                let mut sy = sy.borrow_mut();
                                *sy += pending_change_y;
                        }
                });

                let angle = (*(sy.borrow()) / *(sx.borrow())).atan();
                if angle.is_normal() {
                        (angle.cos(), angle.sin())
                } else {
                        (f64::NAN,f64::NAN)
                }
        });
        ants.iter_mut()
                .zip(
                        velocity_vectors.collect::<Vec<(f64,f64)>>()
                )
                .map(|pair| {
                        let (ant, (vx, vy)) = pair;
                        let momentum = 2.0_f64.powi(8);
                        if !vx.is_nan() {
                                ant.vx = (vx+momentum*ant.vx)/(momentum+1.0);
                        }
                        if !vy.is_nan() {
                                ant.vy = (vy+momentum*ant.vy)/(momentum+1.0);
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
        //let mut time = SystemTime::now();
        //(ant,n,time) = dbg!((ant,n,SystemTime::now()));
        if n < 0.0 {
                panic!("n < 0, n={}", n);
        }
        while 0.9 < n {
                n -= 0.9;
                ant = move_ant_by_n(ant, walls, 0.9);
        }

        let new_x = ant.x + ant.vx * n;
        let new_y = ant.y + ant.vy * n;
        let mut end_ok = false;
        let mut x_ok = false;
        let mut y_ok = false;
        if !is_oob(new_x, new_y) {
                end_ok = !(walls[new_x.floor() as usize]
                [new_y.floor() as usize]
                || is_oob(new_x.floor(), new_y.floor()));
        }
        if !is_oob(new_x, ant.y) {
                x_ok = !(walls[new_x.floor() as usize]
                [ant.y.floor() as usize]
                || is_oob(new_x.floor(), ant.y.floor()));
        }
        if !is_oob(ant.x, new_y) {
                y_ok = !(walls[ant.x.floor() as usize]
                [new_y.floor() as usize]
                || is_oob(ant.x.floor(), new_y.floor()));
        }
        
        
        if x_ok && y_ok && end_ok {
                ant.x = new_x;
                ant.y = new_y;
                if ant.is_oob() {
                        println!("bother {:?}",ant);
                }
                return ant.clone();
        }

        // how far can be moved in x and y
        let mut dx = f64::NAN;
        if ant.vx.is_sign_negative() {
                dx = ant.x - ant.x.floor();
        } else {
                dx = ant.x.ceil() - ant.x;
        };
        if dx == 0. {
                dx = ant.vx.abs()*n
        }

        let mut dy = f64::NAN;
        if ant.vy.is_sign_negative() {
                dy = ant.y - ant.y.floor();
        } else {
                dy = ant.y.ceil() - ant.y;
        };
        if dy == 0. {
                dy = ant.vy.abs()*n
        }
        // time that can be moved in x and y
        let mut tx = (dx / ant.vx.abs());
        let mut ty = (dy / ant.vy.abs());
        


        if tx.is_nan() {
                tx = f64::INFINITY;//ant.vx;
        } else {
                tx = tx.min(n)
        }
        if ty.is_nan() {
                ty = f64::INFINITY;//ant.vy;
        } else {
                tx = tx.min(n)
        }
        
        let new_n = tx.min(ty);
        
        let new_x = ant.x + ant.vx * new_n;
        let new_y = ant.y + ant.vy * new_n;
        if !is_oob(new_x, new_y) {
                ant.x = new_x;
                ant.y = new_y;
        }
        
        // this is how it gets negative
        let remaining_n = n - new_n;
        

        if !x_ok && (round_float(tx) <= round_float(ty)) {
                ant.vx *= -1.0;
        };
        if !y_ok && (round_float(tx) >= round_float(ty)) {
                ant.vy *= -1.0;
        };

        if remaining_n.abs() > 2.0_f64.powi(-16) {
                ant = move_ant_by_n(
                        ant,
                        walls,
                        remaining_n,
                );
        }
        //(tx,ty,dx,dy,ant,n,time) = dbg!((tx,ty,dx,dy,ant,n,SystemTime::now()));
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

#[pyfunction]
#[pyo3(name = "demo_function")]
pub fn python_tick<'py>( py: Python<'py>,py_int: Py<PyInt>, 
//py_ant: Bound<'py, PyAny>
ant: ant::Ant
) -> PyResult<ant::Ant> {
        println!("before");
        let int: i32 = dbg!(py_int).extract(py)?;
        println!("{} {:?}",int, ant);
        assert_eq!(ant,ant::Ant {x:1.,y:1.,vx:0.,vy:0.,trail_conc:1.,wait_left:0, wait_max:5});
        Ok(ant)
}

#[pyfunction]
pub fn init_pheromones() -> PyResult<Vec<Vec<f64>>> {
        Ok(vec![
                vec![
                        0.0;
                        constants::DIMENTIONS[0] as usize
                        ];
                constants::DIMENTIONS[1] as usize])
}

#[pyfunction]
pub fn tick_n(
        n:i32,
        mut ants: Vec<ant::Ant>,
        vec_walls: Vec<wall::Wall>,
        vec_pheromones: Vec<Vec<f64>>
) -> PyResult<(Vec<ant::Ant>,Vec<Vec<f64>>)> {
        let walls = wall::walls_to_mask(vec_walls);
        let mut pheromones: [[f64; constants::DIMENTIONS[0] as usize]; constants::DIMENTIONS[1] as usize] = vec_pheromones
                .iter()
                .map(|x| (x.clone()).try_into().unwrap())
                .collect::<Vec<[f64; constants::DIMENTIONS[0] as usize]>>()
                .try_into().unwrap();

        for i in 0..n {
                (ants,pheromones)=tick(ants,walls,pheromones)?;
        };
                Ok((
                ants,
                pheromones.to_vec()
                .iter()
                .map(|arr|arr.to_vec())
                .collect()))
}

pub fn tick(
        mut ants: Vec<ant::Ant>,
        walls: [[bool; constants::DIMENTIONS[0] as usize]; constants::DIMENTIONS[1] as usize],
        vec_pheromones: [[f64; constants::DIMENTIONS[0] as usize]; constants::DIMENTIONS[1] as usize],
) -> PyResult<(Vec<ant::Ant>,[[f64; constants::DIMENTIONS[0] as usize]; constants::DIMENTIONS[1] as usize])> {
        let mut pheromones: [[f64; constants::DIMENTIONS[0] as usize]; constants::DIMENTIONS[1] as usize] = vec_pheromones
                .iter()
                .map(|x| (x.clone()).try_into().unwrap())
                .collect::<Vec<[f64; constants::DIMENTIONS[0] as usize]>>()
                .try_into().unwrap();

        ants = compute_ant_directions(ants, pheromones);
        
        pheromones = place_pheremones(ants.clone(), pheromones);

        // degrade
        ants = ants.clone().par_iter().cloned().map(|mut ant|
                {
                        let new = (1.0/ant.trail_conc)+0.0;
                        if new.is_normal() {
                                ant.trail_conc = 1.0/new;
                        }
                        ant
                }
        ).collect();
        
        // problem
        ants = move_ants_by_n(ants, walls, constants::MOVE_SPEED);

        Ok((
                ants,
                pheromones))
}

#[cfg(test)]
mod tests {
        use core::f64;

        use super::*;

        #[test]
        fn test_place_pheremone() {
                let old = [[0.0; 96]; 60];
                let mut new = Rc::new(RefCell::new(old.clone()));
                let ant = ant::Ant {
                        x: 1.,
                        y: 1.,
                        vx: 0.,
                        vy: 0.,
                        trail_conc: 1.,
                        wait_left:0,
                        wait_max:5
                };
                place_pheremone(&ant, new.clone());
                assert_eq!(new.borrow()[1][1],1.0)
        }

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
                let mut old = [[0.0; constants::DIMENTIONS[0] as usize]; constants::DIMENTIONS[1] as usize];
                old[1 + 1][1 + 1] = 100.0;
                //old[40][41] = 500.0;
                let result = compute_ant_directions(
                        vec![ant::Ant {
                                x: 1.,
                                y: 1.,
                                vx: 0.,
                                vy: 0.,
                                trail_conc: 1.,
                                wait_left:0,
                                wait_max:5
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
                let mut walls = [[false; constants::DIMENTIONS[0] as usize]; constants::DIMENTIONS[1] as usize];
                let result = move_ant_by_n(
                        ant::Ant {
                                x: 0.0,
                                y: 0.0,
                                vx: 3.0_f64.sqrt() / 2.0,
                                vy: 0.5,
                                trail_conc: 1.,
                                wait_left:0, 
                                wait_max:5
                        },
                        walls,
                        2.0,
                );
                assert_eq!(result.x, result.vx * 2.0);
                assert_eq!(result.y, result.vy * 2.0)
        }

        #[test]
        fn move_obstructed_box() {
                let mut walls = [[false; constants::DIMENTIONS[0] as usize]; constants::DIMENTIONS[1] as usize];
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
                        trail_conc: 1.,
                        wait_left:0,
                        wait_max:5
                };
                let result =
                        move_ant_by_n(ant.clone(), walls, n);
                assert_eq!(round_float(result.x), 1.5);
                assert_eq!(round_float(result.y), 1.5);
                assert_eq!(
                        round_float(result.vy),
                        round_float(-ant.vy)
                );
                assert_eq!(
                        round_float(result.vx),
                        round_float(ant.vx)
                );
        }

        #[test]
        fn move_bounce_corner() {
                let mut walls = [[false; constants::DIMENTIONS[0] as usize]; constants::DIMENTIONS[1] as usize];
                walls[0][0] = true;
                walls[0][1] = true;
                walls[0][2] = true;
                walls[1][0] = true;
                walls[2][0] = true;
                let n = f64::consts::SQRT_2 * 5.0 / 2.0;
                let angle = f64::consts::PI / 180.0 * (-135.0);
                let ant = ant::Ant {
                        x: 2.0,
                        y: 3.0,
                        vx: angle.cos(),
                        vy: angle.sin(),
                        trail_conc: 1.,
                        wait_left:0,
                        wait_max:5
                };
                let result = dbg!(move_ant_by_n(
                        dbg!(ant.clone()),
                        walls,
                        n
                ));
                assert_eq!(round_float(result.x), 1.5);
                assert_eq!(round_float(result.y), 1.5);
                assert_eq!(
                        round_float(result.vy),
                        round_float(-ant.vy)
                );
                assert_eq!(
                        round_float(result.vx),
                        round_float(ant.vx)
                );
        }
}
