pub trait State: Copy + Clone + PartialEq + Eq + std::fmt::Debug {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Idle {  _private: () }
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Configured { _private: () }
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Running {  _private: () }
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Stop {  _private: () }

impl State for Idle {}
impl State for Configured {}
impl State for Running {}
impl State for Stop {}

pub trait ValidTransition<From, To> {}
// pretend this is a real i2c bus
pub struct I2CBus<S: State> {
    _state: std::marker::PhantomData<S>,
}

impl<S: State> I2CBus<S> {
    fn transition<NewS: State>(self) -> I2CBus<NewS>
    where
        Self: ValidTransition<S, NewS>,
    {
        I2CBus {
            _state: std::marker::PhantomData,
        }
    }
    
    pub fn expect<ExpectedS: State>(self) -> ()
    where
        S: std::cmp::PartialEq<ExpectedS>,
    {}
}

impl I2CBus<Stop> {
    pub fn new() -> Self {
        I2CBus {
            _state: std::marker::PhantomData,
        }
    }
    
    pub fn start<F>(mut self, callback: F) -> I2CBus<Idle> 
    where F: Fn(&mut Self) {
        println!("started");
        callback(&mut self);
        self.transition::<Idle>()
    }
}

impl I2CBus<Idle> {
    pub fn configure<F>(mut self, num: u32, callback: F) -> I2CBus<Configured> 
    where F: Fn(&mut Self) {
        println!("configured with number: {num}");
        callback(&mut self);
        self.transition::<Configured>()
    }
    
    pub fn stop<F>(mut self, callback: F) -> I2CBus<Stop> 
    where F: Fn(&mut Self) {
        println!("stopped");
        callback(&mut self);
        self.transition::<Stop>()
    }
}

impl I2CBus<Configured> {
    pub fn run<F>(mut self, callback: F) -> I2CBus<Running> 
    where F: Fn(&mut Self) {
        println!("running");
        callback(&mut self);
        self.transition::<Running>()
    }
}

impl I2CBus<Running> {
    pub fn idle<F>(mut self, callback: F) -> I2CBus<Idle> 
    where F: Fn(&mut Self) {
        println!("idling");
        callback(&mut self);
        self.transition::<Idle>()
    }
}


macro_rules! allow_transition {
    ($peripheral:ty, $from:ty, $to:ty) => {
        impl ValidTransition<$from, $to> for $peripheral {}
    };
}

allow_transition!(I2CBus<Stop>, Stop, Idle);
allow_transition!(I2CBus<Idle>, Idle, Configured);
allow_transition!(I2CBus<Configured>, Configured, Running);
allow_transition!(I2CBus<Running>, Running, Idle);
allow_transition!(I2CBus<Idle>, Idle, Stop);

fn callback_stop(_bus: &mut I2CBus<Stop>) {}
fn callback_idle(_bus: &mut I2CBus<Idle>)  {}
fn callback_configured(_bus: &mut I2CBus<Configured>) {}
fn callback_running(_bus: &mut I2CBus<Running>)  {}

fn main() {
    let new = I2CBus::new();
    
    let running = new
        .start(callback_stop)
        .configure(1000, callback_idle)
        .run(callback_configured);
    
    let idle = running.idle(callback_running);
    
    if true {
        let stop = idle
            .configure(123, callback_idle)
            .run(callback_configured)
            .idle(callback_running)
            .stop(callback_idle);
        
        stop.expect::<Stop>();
    } else {
        let configured = idle.configure(456,  callback_idle);        
        configured.expect::<Configured>();
    }
    
}