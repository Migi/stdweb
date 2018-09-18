use std::marker::PhantomData;
use std::fmt::{Debug, Formatter, Error};

use discard::Discard;
use webcore::value::{Value, Reference};
use webcore::discard::DiscardOnDrop;
use webcore::once::Once;
use webcore::mutfn::Mut;
use webcore::serialization::{JsSerialize, JsSerializeOwned, SerializedValue};
use webcore::try_from::{TryFrom, TryInto};

use std::ops::Deref;

struct DropInJsOnDiscard(Reference);

impl Discard for DropInJsOnDiscard {
	fn discard( self ) {
		js! { @(no_return)
			@{self.0}.drop();
		};
	}
}

/// Generic function handle.
/// 
/// This has all the same functionality as [`FnHandle`](struct.FnHandle.html),
/// [`FnMutHandle`](struct.FnMutHandle.html) and [`FnOnceHandle`](struct.FnOnceHandle.html) but
/// without having type parameters that encode the arguments and return value.
/// 
/// `FnHandle`, `FnMutHandle` and `FnOnceHandle` can be converted into a `GenericFnHandle` using
/// the rust standard library's [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)
/// and [`Into`](https://doc.rust-lang.org/std/convert/trait.Into.html) conversion traits.
#[must_use]
pub struct GenericFnHandle {
    discarder: DiscardOnDrop<DropInJsOnDiscard>
}

/// A wrapper for `FnOnce` closures to use them in `js!` without risking memory leaks.
/// 
/// If you have a [`FnOnce`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) function or closure
/// that you want to call from JavaScript, you can use it in the `js!` macro directly by using the
/// [`Once`](struct.Once.html) wrapper, like this:
/// 
/// ```rust
/// let callback = || { println!( "Hello world!" ); };
/// js! {
///     var cb = @{Once(callback)};
///     cb();
///     // Need to make sure that cb is either called or dropped.
///     // In this case it was called, so we don't need to call `cb.drop();`
/// }
/// ```
/// 
/// If you do this, you must remember to make sure that the callback is either called or dropped,
/// otherwise the rust closure is never dropped and you have a memory leak.
/// 
/// To prevent that memory leak, you can wrap the closure in a `FnOnceHandle`. This handle will automatically
/// drop the closure when it is dropped (any attempts to call the closure from JavaScript after that point
/// will throw a `ReferenceError` exception).
/// 
/// Wrapping a closure in a `FnOnceHandle` is done using the rust standard library's
/// [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) trait, like this:
/// `FnOnceHandle::from(f)`, or `f.into()` (type annotations may be needed for the latter).
/// 
/// # Example
/// 
/// ```rust
/// let callback = || { println!( "Hello world!" ); };
/// let handle = FnOnceHandle::from(callback);
/// js! {
///     var cb = @{&handle}; // note the &. It's necessary (otherwise handle would be moved).
///     cb();
///     // no need to drop cb or make sure cb was called
/// }
/// // callback is dropped when handle goes out of scope
/// ```
#[must_use]
pub struct FnOnceHandle< Args, Output > {
    discarder: DiscardOnDrop<DropInJsOnDiscard>,
	phantom_args: PhantomData<Args>,
	phantom_output: PhantomData<Output>
}

/// A wrapper for `FnMut` closures to use them in `js!` without risking memory leaks.
/// 
/// If you have a [`FnMut`](https://doc.rust-lang.org/std/ops/trait.FnMut.html) function or closure
/// that you want to call from JavaScript, you can use it in the `js!` macro directly by using the
/// [`Mut`](struct.Mut.html) wrapper, like this:
/// 
/// ```rust
/// let callback = || { println!( "Hello world!" ); };
/// js! {
///     var cb = @{Mut(callback)};
///     cb();
///     cb();
///     // Need to make sure to call drop() on cb, or there is a memory leak.
///     cb.drop();
/// }
/// ```
/// 
/// If you do this, you must remember to make sure to drop the callback in JavaScrip manually,
/// otherwise the rust closure is never dropped and you have a memory leak.
/// 
/// To prevent that memory leak, you can wrap the closure in a `FnMutHandle`. This handle will automatically
/// drop the closure when it is dropped (any attempts to call the closure from JavaScript after that point
/// will throw a `ReferenceError` exception).
/// 
/// Wrapping a closure in a `FnMutHandle` is done using the rust standard library's
/// [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) trait, like this:
/// `FnMutHandle::from(f)`, or `f.into()` (type annotations may be needed for the latter).
/// 
/// # Example
/// 
/// ```rust
/// let callback = || { println!( "Hello world!" ); };
/// let handle = FnMutHandle::from(callback);
/// js! {
///     var cb = @{&handle}; // note the &. It's necessary (otherwise handle would be moved).
///     cb();
///     // no need to drop cb or to make sure cb was called
/// }
/// // callback is dropped when handle goes out of scope
/// ```
#[must_use]
pub struct FnMutHandle< Args, Output > {
    discarder: DiscardOnDrop<DropInJsOnDiscard>,
	phantom_args: PhantomData<Args>,
	phantom_output: PhantomData<Output>
}

/// A wrapper for `Fn` closures to use them in `js!` without risking memory leaks.
/// 
/// If you have a [`Fn`](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) function or closure
/// that you want to call from JavaScript, you can use it in the `js!` macro directly, like this:
/// 
/// ```rust
/// let callback = || { println!( "Hello world!" ); };
/// js! {
///     var cb = @{callback};
///     cb();
///     // Need to make sure that cb is either called or dropped.
///     // In this case it was called, so we don't need to call `cb.drop();`
/// }
/// ```
/// 
/// If you do this, you must remember to make sure to drop the callback in JavaScrip manually,
/// otherwise the rust closure is never dropped and you have a memory leak.
/// 
/// To prevent that memory leak, you can wrap the closure in a `FnHandle`. This handle will automatically
/// drop the closure when it is dropped (any attempts to call the closure from JavaScript after that point
/// will throw a `ReferenceError` exception).
/// 
/// Wrapping a closure in a `FnHandle` is done using the rust standard library's
/// [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) trait, like this:
/// `FnHandle::from(f)`, or `f.into()` (type annotations may be needed for the latter).
/// 
/// # Example
/// 
/// ```rust
/// let callback = || { println!( "Hello world!" ); };
/// let handle = FnHandle::from(callback);
/// js! {
///     var cb = @{&handle}; // note the &. It's necessary (otherwise handle would be moved).
///     cb();
///     // no need to drop cb or to make sure cb was called
/// }
/// // callback is dropped when handle goes out of scope
/// ```
#[must_use]
pub struct FnHandle< Args, Output > {
    discarder: DiscardOnDrop<DropInJsOnDiscard>,
	phantom_args: PhantomData<Args>,
	phantom_output: PhantomData<Output>
}

impl GenericFnHandle {
	pub fn new( reference: Reference ) -> GenericFnHandle {
		GenericFnHandle {
			discarder: DiscardOnDrop::new( DropInJsOnDiscard( reference ) )
		}
	}

    pub fn leak( self ) -> Reference {
        self.discarder.leak().0
    }
}

impl< Args, Output > FnOnceHandle< Args, Output > {
	/// Leak the handle.
	/// 
	/// This means that the rust closure won't be dropped unless you call .drop() from the JavaScript side.
	/// This method returns a reference to the JavaScript handle (the thing you're supposed to call .drop() on).
	pub fn leak( self ) -> Reference {
        self.discarder.leak().0
    }
}
impl< Args, Output > FnMutHandle< Args, Output > {
	/// Leak the handle.
	/// 
	/// This means that the rust closure won't be dropped unless you call .drop() from the JavaScript side.
	/// This method returns a reference to the JavaScript handle (the thing you're supposed to call .drop() on).
	pub fn leak( self ) -> Reference {
        self.discarder.leak().0
    }
}
impl< Args, Output > FnHandle< Args, Output > {
	/// Leak the handle.
	/// 
	/// This means that the rust closure won't be dropped unless you call .drop() from the JavaScript side.
	/// This method returns a reference to the JavaScript handle (the thing you're supposed to call .drop() on).
	pub fn leak( self ) -> Reference {
        self.discarder.leak().0
    }
}

impl< Args, Output > From< FnOnceHandle< Args, Output > > for GenericFnHandle {
	fn from( value: FnOnceHandle< Args, Output > ) -> GenericFnHandle {
		GenericFnHandle::new( value.leak() )
	}
}
impl< Args, Output > From< FnMutHandle< Args, Output > > for GenericFnHandle {
	fn from( value: FnMutHandle< Args, Output > ) -> GenericFnHandle {
		GenericFnHandle::new( value.leak() )
	}
}
impl< Args, Output > From< FnHandle< Args, Output > > for GenericFnHandle {
	fn from( value: FnHandle< Args, Output > ) -> GenericFnHandle {
		GenericFnHandle::new( value.leak() )
	}
}

impl Debug for GenericFnHandle {
    fn fmt( &self, fmt: &mut Formatter ) -> Result< (), Error > {
		fmt.debug_tuple( "GenericFnHandle" ).field( &self.discarder.deref().0 ).finish()
    }
}
impl< Args, Output > Debug for FnOnceHandle< Args, Output > {
    fn fmt( &self, fmt: &mut Formatter ) -> Result< (), Error > {
		fmt.debug_tuple( "FnOnceHandle" ).field( &self.discarder.deref().0 ).finish()
    }
}
impl< Args, Output > Debug for FnMutHandle< Args, Output > {
    fn fmt( &self, fmt: &mut Formatter ) -> Result< (), Error > {
		fmt.debug_tuple( "FnMutHandle" ).field( &self.discarder.deref().0 ).finish()
    }
}
impl< Args, Output > Debug for FnHandle< Args, Output > {
    fn fmt( &self, fmt: &mut Formatter ) -> Result< (), Error > {
		fmt.debug_tuple( "FnHandle" ).field( &self.discarder.deref().0 ).finish()
    }
}

impl< 'a > JsSerialize for &'a GenericFnHandle {
    fn _into_js< 'b >( &'b self ) -> SerializedValue< 'b > {
		self.discarder.deref().0._into_js()
	}
}
impl< 'a, Args, Output > JsSerialize for &'a FnOnceHandle< Args, Output > {
    fn _into_js< 'b >( &'b self ) -> SerializedValue< 'b > {
		self.discarder.deref().0._into_js()
	}
}
impl< 'a, Args, Output > JsSerialize for &'a FnMutHandle< Args, Output > {
    fn _into_js< 'b >( &'b self ) -> SerializedValue< 'b > {
		self.discarder.deref().0._into_js()
	}
}
impl< 'a, Args, Output > JsSerialize for &'a FnHandle< Args, Output > {
    fn _into_js< 'b >( &'b self ) -> SerializedValue< 'b > {
		self.discarder.deref().0._into_js()
	}
}

macro_rules! define {
    ($next:tt => $($kind:ident),*) => {
        impl< R: JsSerializeOwned, $($kind: TryFrom<Value>,)* F > From< F > for FnOnceHandle< ($($kind,)*), R > where F: FnOnce( $($kind,)* ) -> R + 'static {
			fn from( f: F ) -> Self {
				Self {
					discarder: DiscardOnDrop::new(DropInJsOnDiscard(js!(return @{Once(f)};).try_into().unwrap())),
					phantom_args: PhantomData,
					phantom_output: PhantomData
				}
			}
        }
		
        impl< R: JsSerializeOwned, $($kind: TryFrom<Value>,)* F > From< F > for FnMutHandle< ($($kind,)*), R > where F: FnMut( $($kind,)* ) -> R + 'static {
			fn from( f: F ) -> Self {
				Self {
					discarder: DiscardOnDrop::new(DropInJsOnDiscard(js!(return @{Mut(f)};).try_into().unwrap())),
					phantom_args: PhantomData,
					phantom_output: PhantomData
				}
			}
        }
		
        impl< R: JsSerializeOwned, $($kind: TryFrom<Value>,)* F > From< F > for FnHandle< ($($kind,)*), R > where F: Fn( $($kind,)* ) -> R + 'static {
			fn from( f: F ) -> Self {
				Self {
					discarder: DiscardOnDrop::new(DropInJsOnDiscard(js!(return @{f};).try_into().unwrap())),
					phantom_args: PhantomData,
					phantom_output: PhantomData
				}
			}
        }

        next! { $next }
    }
}

loop_through_identifiers!( define );

#[cfg(test)]
mod test_fnhandle {
    use super::*;
	use std::cell::Cell;
	use std::rc::Rc;

	#[test]
	fn test_fn_handle() {
		let rc = Rc::new(Cell::new(0));

		struct IncrOnDrop(Rc<Cell<i32>>);
		impl Drop for IncrOnDrop {
			fn drop( &mut self ) {
				self.0.set( self.0.get() + 1 );
			}
		}
		let incr_on_drop = IncrOnDrop(rc.clone());

		let rc2 = rc.clone();

		// this closure adds x to the value in rc
		let f = move |x: i32| {
			rc2.set(rc2.get() + x);
			let _ = &incr_on_drop; // make sure incr_on_drop is moved into the closure
		};

		{
			let handle = FnHandle::from(f);
			js! {
				@{&handle}(2);
				@{&handle}(3);
			}
			assert_eq!(rc.get(), 5);

			// when handle is dropped, the incr_on_drop inside it should be too,
			// and the count should be incremented from 5 to 6.
		}

		assert_eq!(rc.get(), 6);
	}
}
