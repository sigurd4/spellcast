use crate::IsObjOf;

use super::*;

/// A trait for downcasting objects.
/// 
/// # Examples
/// ```rust
/// use spellcast::{downcast::*};
/// use core::any::Any;
/// 
/// // Object trait must extend Any to support downcasts.
/// trait PlayerObj: Any {}
/// 
/// #[derive(PartialEq, Debug)]
/// struct Human;
/// 
/// impl PlayerObj for Human {}
/// 
/// let player = Box::new(Human) as Box<dyn PlayerObj>;
/// 
/// assert_eq!((&*player).downcast_ref(), Some(&Human));
/// ```
pub trait DowncastRef<To>: IsObjOf<To>
where
    To: ?Sized
{
    /// Downcasts an object into a struct implementing the object's trait.
    /// 
    /// # Examples
    /// ```rust
    /// use spellcast::{downcast::*};
    /// use core::any::Any;
    /// 
    /// // Object trait must extend Any to support downcasts.
    /// trait PlayerObj: Any {}
    /// 
    /// #[derive(PartialEq, Debug)]
    /// struct Human;
    /// 
    /// impl PlayerObj for Human {}
    /// 
    /// let player = Box::new(Human) as Box<dyn PlayerObj>;
    /// 
    /// assert_eq!((&*player).downcast_ref(), Some(&Human));
    /// ```
    fn downcast_ref(self: &Self) -> Option<&To>;

    /// Downcasts an object into a struct implementing the object's trait.
    /// 
    /// # Examples
    /// ```rust
    /// use spellcast::{downcast::*};
    /// use core::any::Any;
    /// 
    /// // Object trait must extend Any to support downcasts.
    /// trait PlayerObj: Any
    /// {
    ///     fn is_alive(&self) -> bool;
    /// }
    /// 
    /// struct Human
    /// {
    ///     is_alive: bool
    /// }
    /// 
    /// impl PlayerObj for Human
    /// {
    ///     fn is_alive(&self) -> bool
    ///     {
    ///         self.is_alive
    ///     }
    /// }
    /// 
    /// let mut player = Box::new(Human {is_alive: true}) as Box<dyn PlayerObj>;
    /// 
    /// assert_eq!(player.is_alive(), true);
    /// 
    /// // We can access the inner struct by downcasting.
    /// let human: Option<&mut Human> = (&mut *player).downcast_mut();
    /// if let Some(human) = human
    /// {
    ///     human.is_alive = false;
    /// }
    /// 
    /// assert_eq!(player.is_alive(), false);
    /// ```
    fn downcast_mut(self: &mut Self) -> Option<&mut To>;
}
impl<'a, From, To> DowncastRef<To> for From
where
    From: ?Sized,
    To: DowncastFromRef<From> + ?Sized
{
    fn downcast_ref(self: &Self) -> Option<&To>
    {
        To::downcast_from_ref(self)
    }
    fn downcast_mut(self: &mut Self) -> Option<&mut To>
    {
        To::downcast_from_mut(self)
    }
}