use crate::register::*;


// 
// In IIS3DWB, Interrupts are distinct from Wake up sources
// Therefore, we need to implement a wake up lib. 
// 


pub struct SleepDuration(u8);
impl SleepDuration{
    pub fn from_seconds(data_rate: DataRate, seconds: u32){

    }
}

#[derive(Debug, Copy, Clone)]
pub struct WakeUp {
    filtered: bool,
    threshold: Threshold,
    wake_duration: WakeDuration,
    sleep_duration: SleepDuration,
}


impl WakeUp {

}

