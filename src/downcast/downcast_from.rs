use crate::{Is, AsAny};

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
/// let human: Result<Box<Human>, Box<dyn PlayerObj>> = Human::downcast_from(player);
/// assert_eq!(*human.unwrap(), Human);
/// ```
pub trait DowncastFrom<From, Obj>: DowncastFromRef<From>
where
    From: Is<Obj> + ?Sized,
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
    /// let human: Result<Box<Human>, Box<dyn PlayerObj>> = Human::downcast_from(player);
    /// assert_eq!(*human.unwrap(), Human);
    /// ```
    fn downcast_from(from: Box<From>) -> Result<Box<Self>, Box<Obj>>;
}
impl<From, To, Obj> DowncastFrom<From, Obj> for To
where
    From: Is<Obj> + AsAny + ?Sized + 'static,
    To: DowncastFromRef<From> + 'static,
    Obj: ?Sized
{
    fn downcast_from(from: Box<From>) -> Result<Box<Self>, Box<Obj>>
    {
        if !from.as_any().is::<To>()
        {
            return Ok(from.into_any().downcast().unwrap());
        }
        return Err(from)
    }
}