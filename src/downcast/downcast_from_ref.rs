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
/// assert_eq!(Human::downcast_from_ref(&*player), Some(&Human));
/// ```
pub trait DowncastFromRef<From>: Is<From>
where
    From: ?Sized,
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
    /// assert_eq!(Human::downcast_from_ref(&*player), Some(&Human));
    /// ```
    fn downcast_from_ref<'a>(from: &'a From) -> Option<&'a Self>;
    
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
    /// if let Some(human) = Human::downcast_from_mut(&mut *player)
    /// {
    ///     human.is_alive = false;
    /// }
    /// 
    /// assert_eq!(player.is_alive(), false);
    /// ```
    fn downcast_from_mut<'a>(from: &'a mut From) -> Option<&'a mut Self>;
}
impl<From, To> DowncastFromRef<From> for To
where
    From: AsAny + ?Sized,
    To: Is<From> + 'static
{
    fn downcast_from_ref<'a>(from: &'a From) -> Option<&'a Self>
    {
        from.as_any().downcast_ref()
    }
    fn downcast_from_mut<'a>(from: &'a mut From) -> Option<&'a mut Self>
    {
        from.as_any_mut().downcast_mut()
    }
}