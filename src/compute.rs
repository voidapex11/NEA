use crate::ant;
use crate::constants;
use pyo3::prelude::*;

/// validate if a point is out of bounds
/// ```
/// assert_eq!(is_oob(-4.0,-3.0),true)
/// ```
fn is_oob(x: f64, y: f64) -> bool {
        if x <= 0. || constants::DIMENTIONS[0] as f64 <= x {
                return true;
        }
        if y <= 0. || constants::DIMENTIONS[1] as f64 <= y {
                return true;
        }
        return false;
}

fn compute_ant_directions(
        ants: Vec<ant::Ant>,
        old: [[f64; constants::DIMENTIONS[0] as usize];
                constants::DIMENTIONS[1] as usize],
) {
        let ants_with_points: std::iter::Zip<
                std::slice::Iter<'_, ant::Ant>,
                std::iter::Repeat<[[f64; 3]; 49]>,
        > = ants.iter()
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
        
        let pairs_with_sum = pairs_absolute_value_cords.map(|ant_iter|{
                (ant_iter.clone(),ant_iter.map(|pair| pair.1[2]).sum::<f64>())
        });
        let pairs_magnitude_from_old = pairs_with_sum.map(|ant_iter_w_sum| {
                let (ant_iter,s) = ant_iter_w_sum;
                ant_iter.map(|pair| {
                        let (ant,p) = pair;
                        (ant,p[2]/s*old[(p[0].round() as usize)][(p[1].round() as usize)])
                })
        });// zip together a sum w. each pair?

        // read from actual coordinates
        // work out sum
        // divide by sum4
        //
}
