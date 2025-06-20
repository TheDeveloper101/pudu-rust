mod util;

typestate_peripheral! {
    peripheral I2CBus {
        num: u32,
    };

    states {
        Stop,
        Idle,
        Configured,
        Running,
    };

    initial: Stop;

    transitions {

        start(Stop => Idle) {
            println!("started");
        };
       
        configure(Idle => Configured, num: u32) {
            println!("configured with number {}", num);
        };

        run(Configured => Running) {
            println!("running");
        };

        idle(Running => Idle) {
            println!("idling")
        };

        stop(Idle => Stop) {
            println!("stopped");
        };
    };

    methods {
        Configured => [check_num(self: &mut Self) -> u32 {
            self.num
        }];
    };
}

fn cb_idle(_bus: &mut I2CBus<Configured>) {}
fn cb_run(_bus: &mut I2CBus<Running>) {}
fn cb_start(_bus: &mut I2CBus<Idle>) {}
fn cb_stop(_bus: &mut I2CBus<Stop>) {}

fn main() {
    let bus = I2CBus::new(42)
        .start(cb_start)
        .configure( 43, cb_idle)
        .run(cb_run)
        .idle(cb_start)
        .stop(cb_stop);

    let num = bus
    .clone()
    .start(cb_start)
    .configure(44, cb_idle)
    .check_num();

    println!("number {}", num);
    
    bus.expect::<Stop>();
} 

