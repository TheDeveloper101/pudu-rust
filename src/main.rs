pub trait State: Copy + Clone + PartialEq + Eq + std::fmt::Debug {}

pub trait Peripheral {
    type State: State;
}

pub trait InState<S> {}

pub trait ValidTransition<From, To> {}

// a peripheral that has state
pub struct Stateful<P, S> {
    pub peripheral: P,
    _state: std::marker::PhantomData<S>,
}

impl<P: Peripheral, S> Stateful<P, S> 
where 
    S: State,
    P: InState<S>,
{
    pub fn new<NewState: State>(peripheral: P) -> Self {
        Stateful {
            peripheral,
            _state: std::marker::PhantomData,
        }
    }
    
    pub fn transition<NewS: State>(self) -> Stateful<P, NewS> 
    where 
        P: InState<NewS>,
        P: ValidTransition<S, NewS>,
    {
        Stateful {
            peripheral: self.peripheral,
            _state: std::marker::PhantomData,
        }
    }
    pub fn expect<ExpectedS: State>(self) -> () 
    where 
        S: std::cmp::PartialEq<ExpectedS>,
        P: InState<ExpectedS>
    {}
}

macro_rules! allow_transition {
    ($peripheral:ty, $from:ty, $to:ty) => {
        impl ValidTransition<$from, $to> for $peripheral {}
    };
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Idle { Default}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Configured {Default}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Running {Default}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Stop { Default}
impl State for Idle {}
impl State for Configured {}
impl State for Running {}
impl State for Stop {}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum States {
    Idle(Idle),
    Configured(Configured),
    Running(Running),
    Stop(Stop),
}

impl State for States {}

// actually implementing the peripheral
pub struct I2CBus {
    // pretend this is a real i2c bus
}

impl I2CBus {
    fn new() -> Self {
        I2CBus {}
    }
    
    // Each method would like actually do stuff with the hardware
    fn configure(&mut self, num: u32) {
        println!("configured with number: {num}")
    }
    
    fn run(&mut self) {
        println!("running")
    }

    fn start(&mut self) {
        println!("started")
    }
    
    fn stop(&mut self) {
        println!("stopped")
    }

    fn idle(&mut self) {
        println!("idling")
    }
}

impl Peripheral for I2CBus {
    type State = States;
}

// states the I2C bus can be in
impl InState<Stop> for I2CBus {}
impl InState<Configured> for I2CBus {}
impl InState<Running> for I2CBus {}
impl InState<Idle> for I2CBus {}

// valid state transitions
allow_transition!(I2CBus, Stop, Idle);
allow_transition!(I2CBus, Idle, Configured);
allow_transition!(I2CBus, Configured, Running);
allow_transition!(I2CBus, Running, Idle);
allow_transition!(I2CBus, Idle, Stop);

impl Stateful<I2CBus, Stop> {
    pub fn start<F>(mut self, callback: F) -> Stateful<I2CBus, Idle>  
    where for<'a> F: FnOnce(&I2CBus) -> () {
        self.peripheral.start();
        callback(&self.peripheral);
        self.transition::<Idle>()
    }
}

impl Stateful<I2CBus, Idle> {
    pub fn configure<F>(mut self, number: u32, callback: F) -> Stateful<I2CBus, Configured> 
    where for<'a> F: FnOnce(&I2CBus) -> () {
        self.peripheral.configure(number);
        callback(&self.peripheral);
        self.transition::<Configured>()
    }

    pub fn stop<F>(mut self, callback: F) -> Stateful<I2CBus, Stop> 
    where for<'a> F: FnOnce(&I2CBus) -> () {
        self.peripheral.stop();
        callback(&self.peripheral);
        self.transition::<Stop>()
    }
}

impl Stateful<I2CBus, Configured> {
    pub fn run<F>(mut self, callback: F) -> Stateful<I2CBus, Running> 
    where for<'a> F: FnOnce(&I2CBus) -> () {
        self.peripheral.run();
        callback(&self.peripheral);
        self.transition::<Running>()
    } 
}

impl Stateful<I2CBus, Running> {
    pub fn idle<F>(mut self, callback: F) -> Stateful<I2CBus, Idle> 
    where for<'a> F: FnOnce(&I2CBus) -> () {
        self.peripheral.idle();
        callback(&self.peripheral);
        self.transition::<Idle>()
    }
}

fn main() {
    let bus = I2CBus::new();
    fn dummy(bus: &I2CBus) {};
    let stop = Stateful::<I2CBus, Stop>::new::<Stop>(bus);
    let idle = stop.start(dummy);
    let configured = idle.configure(1000, dummy);
    
    let running = configured.run(dummy);

    let idle = running.idle(dummy);
    let test: Stateful<I2CBus, _>;
    if true == true {
        let stopped = idle.configure(123, dummy).run(dummy).idle(dummy).stop(dummy);
        stopped.expect::<Stop>();
        // stopped.run();
    } else {
        test = idle.configure(123, dummy).run(dummy).idle(dummy).stop(dummy);
    }

    // test.expect::<Running>();
}