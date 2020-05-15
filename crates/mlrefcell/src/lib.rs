use once_cell::sync::OnceCell;
use std::cell;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::fmt;
use std::fmt::Debug;
use std::mem;

pub type BorrowError = cell::BorrowError;

/// Returned from `borrow_mut`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BorrowMutError {
    /// Returned if the MLRefCell has already been borrowed
    AlreadyBorrowed,
    /// Returned if the MLRefCell had been locked before
    Locked,
}

/// Returned when attempting to lock an already locked MLRefCell
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AlreadyLockedError;

/// A RefCell which can be "locked" to prevent future mutable borrows
pub struct MLRefCell<T: ?Sized> {
    lock: OnceCell<()>,
    inner: RefCell<T>,
}

impl<T> MLRefCell<T> {
    /// Creates a new `MLRefCell` containing `value`
    pub const fn new(value: T) -> MLRefCell<T> {
        MLRefCell {
            inner: RefCell::new(value),
            lock: OnceCell::new(),
        }
    }

    /// Consumes the `MLRefCell`, returning the wrapped value
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }

    /// Replaces the wrapped value with a new one, returning the old value, without deinitializing either one.
    /// This function corresponds to `std::mem::replace`.
    pub fn replace(&self, val: T) -> T {
        mem::replace(&mut *self.borrow_mut(), val)
    }

    /// Replaces the wrapped value with a new one computed from f, returning the old value, without deinitializing either one.
    pub fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T {
        let mut_borrow = &mut *self.borrow_mut();
        let replacement = f(mut_borrow);
        mem::replace(mut_borrow, replacement)
    }

    /// Swaps the wrapped value of self with the wrapped value of other, without deinitializing either one.
    /// This function corresponds to `std::mem:swap`.
    pub fn swap(&self, other: &RefCell<T>) {
        mem::swap(&mut *self.borrow_mut(), &mut *other.borrow_mut())
    }
}

impl<T: ?Sized> MLRefCell<T> {
    /// Locks the `MLRefCell`, so that future mutable borrows will result in an error.
    pub fn lock(&self) -> Result<(), AlreadyLockedError> {
        self.lock.set(()).map_err(|_| AlreadyLockedError {})
    }

    /// Returns whether the `MLRefCell` is locked.
    pub fn is_locked(&self) -> bool {
        self.lock.get().is_some()
    }

    /// Immutably borrows the wrapped value, panicking if the value is currently mutably
    /// borrowed.
    pub fn borrow(&self) -> Ref<'_, T> {
        self.try_borrow().expect("already mutably borrowed")
    }

    /// Immutably borrows the wrapped value, returning a `BorrowError` if the value is currently
    /// mutably borrowed.
    pub fn try_borrow(&self) -> Result<Ref<'_, T>, BorrowError> {
        self.inner.try_borrow()
    }

    /// Mutably borrows the wrapped value, panicking if the value is either currently borrowed or
    /// if the `MLRefCell` is locked.
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.try_borrow_mut().unwrap_or_else(|e| match e {
            BorrowMutError::Locked => {
                panic!("tried to borrow mutably a locked MLRefCell");
            }
            BorrowMutError::AlreadyBorrowed => panic!("already borrowed"),
        })
    }

    /// Mutably borrows the wrapped value, returning an error if the value is either currently borrowed or
    /// if the `MLRefCell` is locked.
    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, BorrowMutError> {
        if self.is_locked() {
            Err(BorrowMutError::Locked)
        } else {
            self.inner
                .try_borrow_mut()
                .map_err(|_| BorrowMutError::AlreadyBorrowed)
        }
    }

    /// Returns a raw pointer to the underlying data in this cell.
    pub fn as_ptr(&self) -> *mut T {
        self.inner.as_ptr()
    }

    /// Returns a mutable reference to the underlying data.
    /// This call borrows `self` mutably, so there is no need for runtime checks (including lock
    /// checks). This method is
    /// usually not what you want, so use `borrow_mut` instead.
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Immutably borrows the wrapped value, returning an error if the value is currently mutably borrowed.
    /// Unlike `RefCell::borrow`, this method is unsafe because it does not return a `Ref`, thus leaving the borrow flag untouched. Mutably borrowing the `RefCell` while the reference returned by this method is alive is undefined behaviour.
    pub unsafe fn try_borrow_unguarded(&self) -> Result<&T, BorrowError> {
        self.inner.try_borrow_unguarded()
    }
}

impl<T: ?Sized + Clone> Clone for MLRefCell<T> {
    fn clone(&self) -> MLRefCell<T> {
        MLRefCell::new(self.borrow().clone())
    }
}

impl<T: ?Sized + Default> Default for MLRefCell<T> {
    fn default() -> MLRefCell<T> {
        MLRefCell::new(Default::default())
    }
}

impl<T: ?Sized + Debug> Debug for MLRefCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.try_borrow() {
            Ok(borrow) => f.debug_struct("MLRefCell").field("value", &borrow).finish(),
            Err(_) => {
                // The MLRefCell is mutably borrowed so we can't look at its value
                // here. Show a placeholder instead.
                struct BorrowedPlaceholder;

                impl Debug for BorrowedPlaceholder {
                    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                        f.write_str("<borrowed>")
                    }
                }

                f.debug_struct("MLRefCell")
                    .field("value", &BorrowedPlaceholder)
                    .finish()
            }
        }
    }
}
