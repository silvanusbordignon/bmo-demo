use std::rc::Rc;
use std::cell::RefCell;

use bmo::BMO;
use olympus::channel::Channel;
use olympus::Visualizer;
use rip_worldgenerator::MyWorldGen;
use macroquad::{prelude::*};

// Visualizer config | Leave function signature unchanged

fn window_conf() -> Conf {
    Conf {
        window_title: "Olympus".to_string(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        ..Default::default()
    }
}

// Macroquad entrypoint
#[macroquad::main(window_conf)]

async fn main() {

    // Channel used by the robot to comunicate with the GUI
    let channel = Rc::new(RefCell::new(Channel::default()));

    // World Generator

    let dimension:usize = 200;
    let n_mari:i32 = 5;
    let n_prati:i32 = 5;
    let n_deserti:i32 = 5;
    let fully_connected:bool = true;
    let remove_street:bool = false;
    let n_citta:i32 = 5;
    let flat:bool = true;
    let seed:Option<u64> = Some(25);

    let world_generator = MyWorldGen::new_param(
        dimension,
        n_mari,
        n_prati,
        n_deserti,
        fully_connected,
        remove_street,
        n_citta,
        flat,
        seed
    );

    // Robot, having a channel as its field
    let robot = Box::new(BMO::new(Rc::clone(&channel)));

    // Launch the visualizer
    let mut visualizer = Visualizer::new(robot, world_generator, dimension, Rc::clone(&channel));
    visualizer.start().await;
}