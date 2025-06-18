#[macro_export]
macro_rules! typestate_peripheral {
    (
        peripheral $peripheral:ident$( { $($peripheral_body:tt)* } )?;

        states {
            $($state:ident),* $(,)?
        };

        initial: $initial:ident;

        transitions {
            $(
                $method:ident
                ( $from:ident => $to:ident $(, $($arg:ident : $arg_ty:ty)* )? )
                $( { $($method_body:tt)* } )?;
            )*
        };        

        methods {
            $(
                $method_state:ident => [$method_name:ident $( ( $($argument:ident : $arg_type:ty),* ) )? -> $ret:ty 
                $( { $($method_body_:tt)* } )?];
            )*
        };
        
    ) => {
        pub trait State: Copy + Clone + PartialEq + Eq + std::fmt::Debug {}

        $(
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            pub struct $state { _private: () }
            impl State for $state {}
        )*

        pub struct $peripheral<S: State> {
            _state: std::marker::PhantomData<S>,
        }

        impl<S: State> $peripheral<S> {
            fn transition<NewS: State>(self) -> $peripheral<NewS>
            where
                Self: ValidTransition<S, NewS>,
            {
                $peripheral { 
                    $( $($peripheral_body)* )? 
                    _state: std::marker::PhantomData 
                }
            }

            pub fn with_callback<F, R>(mut self, callback: F) -> (Self, R)
            where
                F: Fn(&mut Self) -> R,
            {
                let result = callback(&mut self);
                (self, result)
            }

            pub fn expect<ExpectedS: State>(self)
            where
                S: std::cmp::PartialEq<ExpectedS>,
            {}
        }

        pub trait ValidTransition<From: State, To: State> {}
        $(
            impl ValidTransition<$from, $to> for $peripheral<$from> {}
        
            impl $peripheral<$from> {
                pub fn $method<F>(self $(, $($arg : $arg_ty)* )?, callback: F) -> $peripheral<$to>
                where
                    F: Fn(&mut $peripheral<$to>),
                {
                    {
                        $( $($method_body)* )?
                    }
                    let intermediate = self.transition::<$to>();
                    let (ret, _) = intermediate.with_callback(callback);
                    ret
                }
            }
        )*        

        $(
            impl $peripheral<$method_state> {
                pub fn $method_name(&mut self $(, $($argument : $arg_type)* )? ) -> $ret {
                    $( $($method_body_)* )?
                }
            }
        )*
        

        impl $peripheral<$initial> {
            pub fn new() -> Self {
                $peripheral { 
                    $( $($peripheral_body)* )?
                    _state: std::marker::PhantomData 
                }
            }
        }
    }
}