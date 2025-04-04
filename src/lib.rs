use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// Define traits that represent peripherals and their states
pub trait Peripheral: Clone + Eq + Hash {}

pub trait PeripheralState: Clone + Eq + Hash {
    type PeripheralType: Peripheral;
    
    // Returns the off state for a peripheral
    fn off() -> Self;
}

// Main typestate tracking system
pub struct Pudu<P, S>
where
    P: Peripheral,
    S: PeripheralState<PeripheralType = P>,
{
    // Track the current state of each peripheral
    states: HashMap<P, S>,
    
    // Track functions that manipulate peripherals
    sleep_functions: HashMap<P, Vec<fn(&P)>>,
    restore_functions: HashMap<P, Vec<fn(&P)>>,
    access_functions: HashMap<P, Vec<fn(&P)>>,
    
    // Track ISRs associated with peripherals
    isr_mappings: HashMap<P, HashSet<String>>,
    active_isrs: HashSet<String>,
}

impl<P, S> Pudu<P, S>
where
    P: Peripheral,
    S: PeripheralState<PeripheralType = P>,
{
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            sleep_functions: HashMap::new(),
            restore_functions: HashMap::new(),
            access_functions: HashMap::new(),
            isr_mappings: HashMap::new(),
            active_isrs: HashSet::new(),
        }
    }
        
    // Typestate Tracking
    pub fn enroll(&mut self, peripheral: P) {
        self.states.insert(peripheral.clone(), S::off());
    }
    
    pub fn update(&mut self, peripheral: &P, state: S) {
        self.states.insert(peripheral.clone(), state);
    }
    
    // Function Registration
    pub fn register_sleep(&mut self, peripheral: P, func: fn(&P)) {
        self.sleep_functions
            .entry(peripheral)
            .or_insert_with(Vec::new)
            .push(func);
    }
    
    pub fn register_restore(&mut self, peripheral: P, func: fn(&P)) {
        self.restore_functions
            .entry(peripheral)
            .or_insert_with(Vec::new)
            .push(func);
    }
    
    pub fn register_access(&mut self, peripheral: P, func: fn(&P)) {
        self.access_functions
            .entry(peripheral)
            .or_insert_with(Vec::new)
            .push(func);
    }
    
    // ISR Registration
    pub fn enable_isr(&mut self, peripheral: &P, isr_name: &str) {
        self.isr_mappings
            .entry(peripheral.clone())
            .or_insert_with(HashSet::new)
            .insert(isr_name.to_string());
        self.active_isrs.insert(isr_name.to_string());
    }
    
    pub fn disable_isr(&mut self, _peripheral: &P, isr_name: &str) {
        self.active_isrs.remove(isr_name);
    }
    
    // Checkers
    pub fn check(&self, peripheral: &P, expected_state: &S) -> bool {
        if let Some(state) = self.states.get(peripheral) {
            state == expected_state
        } else {
            false
        }
    }
}