use crate::Is;

use super::*;

/// A trait for downcasting objects.
/// 
/// # Examples
/// ```rust
/// use spellcast::{downcast::*};
/// use core::{fmt::Debug, any::Any};
/// 
/// // Object trait must extend Any to support downcasts.
/// trait PlayerObj: Any + Debug {}
/// 
/// #[derive(PartialEq, Debug)]
/// struct Human;
/// 
/// impl PlayerObj for Human {}
/// 
/// let player = Box::new(Human) as Box<dyn PlayerObj>;
/// 
/// let human: Result<Box<Human>, Box<dyn PlayerObj>> = player.downcast();
/// assert_eq!(*human.unwrap(), Human);
/// ```
pub trait Downcast<To, Obj>: DowncastRef<To> + Is<Obj>
where
    To: ?Sized,
    Obj: ?Sized
{
    /// Downcasts an object into a struct implementing the object's trait.
    /// 
    /// # Examples
    /// ```rust
    /// use spellcast::{downcast::*};
    /// use core::{fmt::Debug, any::Any};
    /// 
    /// // Object trait must extend Any to support downcasts.
    /// trait PlayerObj: Any + Debug {}
    /// 
    /// #[derive(PartialEq, Debug)]
    /// struct Human;
    /// 
    /// impl PlayerObj for Human {}
    /// 
    /// let player = Box::new(Human) as Box<dyn PlayerObj>;
    /// 
    /// let human: Result<Box<Human>, Box<dyn PlayerObj>> = player.downcast();
    /// assert_eq!(*human.unwrap(), Human);
    /// ```
    fn downcast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>;
}
impl<'a, From, To, Obj> Downcast<To, Obj> for From
where
    From: DowncastRef<To> + Is<Obj> + ?Sized,
    To: DowncastFrom<Self, Obj> + ?Sized,
    Obj: ?Sized
{
    fn downcast(self: Box<Self>) -> Result<Box<To>, Box<Obj>>
    {
        To::downcast_from(self)
    }
}