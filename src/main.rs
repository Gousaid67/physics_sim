extern crate json;
extern crate rend3;
extern crate pollster;
extern crate winit;
extern crate chrono;

pub mod math;
pub mod render;
pub mod p_engine;
pub mod constants;
pub mod ch_com;


use chrono::{NaiveDateTime};
use std::fs;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};


#[derive(PartialEq)]
pub enum GlobalState
{
    Loading,
    Unloaded,
    Running,
    Loaded,
    Paused,
    Stopped,
}
//static prog_state : &'static std::sync::Arc<GlobalState> = &std::sync::Arc::new(GlobalState::Unloaded);
fn main() {
    
    
    let mut currpath = std::env::current_dir().unwrap();
    currpath.push("Sol.json");
    println!("{0}", currpath.to_str().unwrap());
    let mut phys_engine = p_engine::PEngine::default();

    let file_dat = fs::read_to_string(&currpath).unwrap();
    let parse_res = json::parse(&file_dat).unwrap();

    let time = &parse_res["World"]["date"].to_string();
    let p_name = &parse_res["World"]["name"].to_string();
    println!("name : {0}", p_name);
    let p_date = NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S").unwrap();

    let bodies = &parse_res["World"]["Bodies"];
    let body_iter = bodies.members();
    //**prog_state = GlobalState::Loading;
    phys_engine.worldstate = p_engine::PEngineState::Loading;
    phys_engine.world.name = p_name.to_string();
    phys_engine.timestamp = p_date.timestamp();
    for members in body_iter
    {
        phys_engine.world.bodylist.push(p_engine::Body
            {
                bID : phys_engine.bodycount,
                name : members["name"].to_string(),
                mass : members["mass"].as_f64().unwrap(),
                radius : members["radius"].as_f32().unwrap(),
                position : math::vec::Vec3::new(members["position"][0].as_f64().unwrap(), members["position"][1].as_f64().unwrap(), members["position"][1].as_f64().unwrap()),
                velocity : math::vec::Vec3::new(members["velocity"][0].as_f64().unwrap(), members["velocity"][1].as_f64().unwrap(), members["velocity"][1].as_f64().unwrap())

            });
            phys_engine.bodycount += 1;
    }
    phys_engine.worldstate = p_engine::PEngineState::Loaded;

    let (p_thread, r_thread, i_thread) = init(phys_engine);

    p_thread.join().unwrap();
    r_thread.join().unwrap();
    i_thread.join().unwrap();
    return;

    
   

    //timestamp = chrono::DateTime::from_str(&time.as_str())
    //let time = parse_res.
    //let mut p_engine = PEngine;      
}
fn init(engine_state : p_engine::PEngine) -> (std::thread::JoinHandle<()>, std::thread::JoinHandle<()>, std::thread::JoinHandle<()>)
{
    let (bodytx, bodyrx) : ( Sender<p_engine::PEngine> , Receiver<p_engine::PEngine> ) = mpsc::channel();
    let (intx, inrx) : (Sender<(String, String)>, Receiver<(String, String)>) = mpsc::channel();
    let phys_thread = std::thread::spawn(move || {
        println!("Physics thread started");
        p_loop(engine_state , bodytx)
        
    });
    let rend_thread = std::thread::spawn(move || {
        //render::init_Render(bodyrx);
        println!("Render thread started");
        #[allow(while_true)]
        while true
        {
            bodyrx.recv().expect("Connection to Sender lost!"); //p_engine will always send data, with the current delay this will never result in an Empty Err, but this should be accounted for once the renderer is implemented.
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });
    let input_thread = std::thread::spawn(move || {
        inloop(intx)

    });

   //**prog_state = GlobalState::Running;
    
    return (phys_thread, rend_thread, input_thread);
    
}

fn p_loop(mut engine_state : p_engine::PEngine, bodytx : std::sync::mpsc::Sender<p_engine::PEngine>) 
{
    engine_state.worldstate = p_engine::PEngineState::Running;
    while engine_state.worldstate != p_engine::PEngineState::Stopped
    {
        if engine_state.worldstate == p_engine::PEngineState::Paused
        {
            continue;
        }
        if engine_state.simticks >= 5256000
        {
            engine_state.worldstate = p_engine::PEngineState::Stopped;
        }
        engine_state.process_physics();
        engine_state.simticks = engine_state.simticks + 1;
        bodytx.send(engine_state.clone()).expect("Connection to reciever lost!");


    }
    println!("ticks ran: {0}", engine_state.simticks);

}
fn inloop(cmdsend : std::sync::mpsc::Sender<(String, String)>)
{
    return;

}
