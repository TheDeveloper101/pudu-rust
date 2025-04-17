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
    
    pub fn get(&self) -> &P {
        &self.peripheral
    }
    
    pub fn expect<ExpectedS: State>(self) -> Self 
    where 
        S: std::cmp::PartialEq<ExpectedS>,
        P: InState<ExpectedS>
    {
        self
    }
}

#[macro_export]
macro_rules! define_transition {
    ($peripheral:ty, $from:ty, $to:ty) => {
        impl ValidTransition<$from, $to> for $peripheral {}
    };
}
