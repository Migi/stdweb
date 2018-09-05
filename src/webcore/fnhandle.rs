use std::marker::PhantomData;
use std::fmt::{Debug, Formatter, Error};

use discard::Discard;
use webcore::value::{Value, Reference};
use webcore::discard::DiscardOnDrop;
use webcore::once::Once;
use webcore::serialization::JsSerializeOwned;
use webcore::try_from::{TryFrom, TryInto};

// TODO: should this be Clone?
pub struct FnOnceHandle< Args, Output > {
    reference: Reference,
	phantom_args: PhantomData<Args>,
	phantom_output: PhantomData<Output>
}

#[derive( Clone )]
pub struct FnMutHandle< Args, Output > {
    reference: Reference,
	phantom_args: PhantomData<Args>,
	phantom_output: PhantomData<Output>
}

#[derive( Clone )]
pub struct FnHandle< Args, Output > {
    reference: Reference,
	phantom_args: PhantomData<Args>,
	phantom_output: PhantomData<Output>
}

impl< Args, Output > Discard for FnOnceHandle< Args, Output > {
    fn discard( self ) {
        js! { @(no_return)
            @{&self.reference}.drop();
        }
    }
}

impl< Args, Output > Discard for FnMutHandle< Args, Output > {
    fn discard( self ) {
        js! { @(no_return)
            @{&self.reference}.drop();
        }
    }
}

impl< Args, Output > Discard for FnHandle< Args, Output > {
    fn discard( self ) {
        js! { @(no_return)
            @{&self.reference}.drop();
        }
    }
}

impl< Args, Output > Debug for FnOnceHandle< Args, Output > {
    fn fmt( &self, fmt: &mut Formatter ) -> Result< (), Error > {
		fmt.debug_tuple("FnOnceHandle").field(&self.reference).finish()
    }
}

impl< Args, Output > Debug for FnMutHandle< Args, Output > {
    fn fmt( &self, fmt: &mut Formatter ) -> Result< (), Error > {
		fmt.debug_tuple("FnMutHandle").field(&self.reference).finish()
    }
}

impl< Args, Output > Debug for FnHandle< Args, Output > {
    fn fmt( &self, fmt: &mut Formatter ) -> Result< (), Error > {
		fmt.debug_tuple("FnHandle").field(&self.reference).finish()
    }
}

macro_rules! define {
    ($next:tt => $($kind:ident),*) => {
        impl< R: JsSerializeOwned, $($kind: TryFrom<Value>,)* > FnOnceHandle< ($($kind,)*), R > {
			pub fn new< F: FnOnce( $($kind,)* ) -> R + 'static >(f: F) -> DiscardOnDrop<Self> {
				DiscardOnDrop::new(Self {
					reference: js!(return @{Once(f)};).try_into().unwrap(),
					phantom_args: PhantomData,
					phantom_output: PhantomData
				})
			}
        }
		
        impl< R: JsSerializeOwned, $($kind: TryFrom<Value>,)* > FnMutHandle< ($($kind,)*), R > {
			pub fn new< F: FnMut( $($kind,)* ) -> R + 'static >(f: F) -> DiscardOnDrop<Self> {
				DiscardOnDrop::new(Self {
					reference: js!(return @{f};).try_into().unwrap(),
					phantom_args: PhantomData,
					phantom_output: PhantomData
				})
			}
        }
		
        impl< R: JsSerializeOwned, $($kind: TryFrom<Value>,)* > FnHandle< ($($kind,)*), R > {
			pub fn new< F: Fn( $($kind,)* ) -> R + 'static >(f: F) -> DiscardOnDrop<Self> {
				DiscardOnDrop::new(Self {
					reference: js!(return @{f};).try_into().unwrap(),
					phantom_args: PhantomData,
					phantom_output: PhantomData
				})
			}
        }

        next! { $next }
    }
}

loop_through_identifiers!( define );

#[cfg(test)]
mod test_fnhandle {
    use super::*;

	#[test]
	fn todo() {
		// todo
	}
}
