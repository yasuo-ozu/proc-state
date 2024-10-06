#![doc = include_str!("README.md")]

use core::mem::MaybeUninit;
use std::hash::{DefaultHasher, Hash, Hasher};

#[doc(hidden)]
pub mod _impl {
    pub use proc_state_macro::random;
}

/// Generate [`Global<T>`] at the context. No arguments are supported.
///
/// ```
/// # use std::sync::Mutex;
/// const COUNTER: proc_state::Global<Mutex<usize>> = proc_state::new!();
/// ```
#[macro_export]
macro_rules! new {
    () => {
        unsafe {
            $crate::Global::_new_inner(
                ::core::file!(),
                ::core::line!(),
                ::core::module_path!(),
                $crate::_impl::random!(),
            )
        }
    };
}

/// Owning global state, shared between macro call.
pub struct Global<T> {
    file: &'static str,
    line: u32,
    module_path: &'static str,
    random: usize,
    phantom: core::marker::PhantomData<T>,
}

impl<T: Send + Sync> Global<T> {
    #[doc(hidden)]
    pub const unsafe fn _new_inner(
        file: &'static str,
        line: u32,
        module_path: &'static str,
        random: usize,
    ) -> Self {
        Global {
            file,
            line,
            module_path,
            random,
            phantom: core::marker::PhantomData,
        }
    }

    fn get_ident_name(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.file.hash(&mut hasher);
        self.line.hash(&mut hasher);
        self.module_path.hash(&mut hasher);
        self.random.hash(&mut hasher);
        format!("PROC_STATE_PTR_{}", hasher.finish())
    }

    /// Get a reference to the inner value, or insert new value if empty.
    pub fn or_insert(&self, slot: T) -> &'static T {
        self.or_insert_with(|| slot)
    }

    /// Get a reference to the inner value.
    pub fn get(&self) -> Option<&'static T> {
        let name = self.get_ident_name();
        if let Ok(var) = std::env::var(&name) {
            let pointer: Result<usize, _> = var.parse();
            if let Ok(pointer) = pointer {
                unsafe {
                    return Some(std::mem::transmute(pointer));
                }
            }
        }
        None
    }

    /// Get a reference to the inner value, or insert new value if empty.
    pub fn or_insert_with(&self, f: impl FnOnce() -> T) -> &'static T {
        if let Some(got) = self.get() {
            return got;
        }
        let name = self.get_ident_name();
        let mut allocated = Box::new(MaybeUninit::<T>::uninit());
        unsafe { std::env::set_var(&name, (allocated.as_ref() as *const _ as usize).to_string()) }
        if let Some(got) = self.get() {
            if got as *const _ as usize == allocated.as_ref() as *const _ as usize {
                // Panicking in f() has no effect on this function
                let generated = f();
                allocated.write(generated);
                let r = allocated.as_ref() as *const _;
                std::mem::forget(allocated);
                unsafe { std::mem::transmute(r) }
            } else {
                got
            }
        } else {
            panic!("Cannot set environment variable")
        }
    }
}
