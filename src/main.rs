use rppal::pwm::{Channel, Polarity, Pwm};
use std::thread::sleep;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::fs;
use simple_signal::{self, Signal};


mod data;

pub use data::{FanSpeed, SpeedStep};

const OFF_THRESHOLD : f32  = 45f32;
const ON_THRESHOLD : f32 = 60f32;
const MIN_DUTY_CYCLE : f32 = 0.50f32;
const SENSITVITY :f32 = 1f32;
const INTERVAL : u64 = 5;



fn get_temp() -> f32 {
    let mut temp_text = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp").unwrap();
    temp_text.truncate(5);
    temp_text.parse::<i64>().unwrap() as f32 / 1000f32
}


fn main() {

    let running = Arc::new(AtomicBool::new(true));

    simple_signal::set_handler(
        &[Signal::Int, Signal::Term],
        {
            let running = running.clone();
            move |_| { running.store(false, Ordering::SeqCst) }
        }
    );

    let fan_speed = FanSpeed::new(MIN_DUTY_CYCLE, OFF_THRESHOLD, ON_THRESHOLD);
    // fan_speed.push(SpeedStep::new(50f32,0.6f32));

    let pwm = Pwm::with_frequency(Channel::Pwm0, 25f64, 0f64, Polarity::Normal, true).unwrap();

    let mut new_temp;
    let mut old_temp = 0f32;
    let mut new_speed;
    let mut old_speed = 0f32;
    
    while running.load(Ordering::SeqCst) {
        
        new_temp = get_temp();
        
        if (new_temp - old_temp).abs() > SENSITVITY {
            new_speed = fan_speed.get_speed(new_temp);

            if (new_speed - old_speed).abs() > 0.01f32  {
                pwm.set_duty_cycle(new_speed as f64).unwrap();
                println!("Current temperature : {}, Fan Speed : {}", new_temp, new_speed);
            }else{
                println!("Current temperature : {}", new_temp)
            }

            old_temp = new_temp;
            old_speed = new_speed
        }

        sleep(Duration::from_secs(INTERVAL));        
    }

        
}

    




