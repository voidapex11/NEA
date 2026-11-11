use pyo3::prelude::*;
use crate::constants;
use std::rc::Rc;
use std::cell::RefCell;


#[derive(FromPyObject)]
pub struct Wall {
        x: i32,
        y: i32
}

pub fn walls_to_mask(walls: Vec<Wall>) -> 
        [
                [bool; constants::DIMENTIONS[0] as usize];
                constants::DIMENTIONS[1] as usize
        ]
 {
        let mut output = Rc::new(RefCell::new([[false; constants::DIMENTIONS[0] as usize];
                constants::DIMENTIONS[1] as usize]));
        walls.iter().for_each(
                |wall|{(*output.borrow_mut())[wall.x as usize][wall.y as usize]=true}
        );
        <RefCell<[
                [bool; constants::DIMENTIONS[0] as usize];
                constants::DIMENTIONS[1] as usize
        ]> as Clone>::clone(&output).into_inner()
}