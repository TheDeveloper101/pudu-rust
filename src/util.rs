#[macro_export]
macro_rules! typestate_peripheral {
    (
        peripheral $peripheral:ident $( { $(
            $field_name:ident : $field_type:ty
        ),* $(,)? } )?;

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
                $method_state:ident => [
                    $method_name:ident ( $($sig:tt)* ) -> $ret:ty
                    $( { $($method_body_:tt)* } )?
                ];
            )*
        };
    ) => {
        pub trait State: Copy + Clone + PartialEq + Eq + std::fmt::Debug {}

        $(
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            pub struct $state { _private: () }
            impl State for $state {}
        )*

        #[derive(Clone)]
        pub struct $peripheral<S: State> {
            $(
                $(
                    $field_name: $field_type,
                )*
            )?
            _state: std::marker::PhantomData<S>,
        }

        impl<S: State> $peripheral<S> {
            fn transition<NewS: State>(self) -> $peripheral<NewS>
            where
                Self: ValidTransition<S, NewS>,
            {
                let $peripheral {
                    $(
                        $(
                            $field_name,
                        )*
                    )?
                    _state: _,
                } = self;

                $peripheral {
                    $(
                        $(
                            $field_name,
                        )*
                    )?
                    _state: std::marker::PhantomData,
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

        impl $peripheral<$initial> {
            pub fn new($($($field_name: $field_type),*)?) -> Self {
                $peripheral {
                    $(
                        $(
                            $field_name,
                        )*
                    )?
                    _state: std::marker::PhantomData,
                }
            }
        }

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
                pub fn $method_name($($sig)*) -> $ret {
                    $( $($method_body_)* )?
                }
            }
        )*
    };
}
