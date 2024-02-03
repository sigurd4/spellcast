use std::{any::{self, Any}, marker::Unsize};

use crate::upcast::Upcast;

use self::{downcast::{DowncastRef, DowncastFromRef}, upcast::TryUpcastRef};

use super::*;

/// Type conversion trait. Relies on implementation of [ConvertInto](ConvertInto)
/// 
/// # Examples
/// ```rust
/// #![feature(unsize)]
/// 
/// use spellcast::{convert::*, impl_object};
/// use clone_box::clone_box;
/// use core::any::Any;
/// 
/// #[clone_box]
/// trait PlayerObj: Any + ConvertInto<Human>
/// {
///     fn is_human(&self) -> bool;
/// }
/// impl_object!(PlayerObj);
/// 
/// #[derive(Clone)]
/// struct Human;
/// 
/// #[clone_box]
/// impl PlayerObj for Human
/// {
///     fn is_human(&self) -> bool
///     {
///         true
///     }
/// }
/// impl ConvertInto<Human> for Human
/// {
///     fn convert_into(self: Box<Self>) -> Box<Human>
///     {
///         self
///     }
/// }
/// 
/// #[derive(Clone)]
/// struct Bot;
/// 
/// #[clone_box]
/// impl PlayerObj for Bot
/// {
///     fn is_human(&self) -> bool
///     {
///         false
///     }
/// }
/// impl ConvertInto<Human> for Bot
/// {
///     fn convert_into(self: Box<Self>) -> Box<Human>
///     {
///         Box::new(Human)
///     }
/// }
/// 
/// let mut player = Box::new(Bot) as Box<dyn PlayerObj>;
/// 
/// assert!(!player.is_human());
/// 
/// let human = Human::convert_get(&mut player);
/// 
/// assert!(human.is_human());
/// assert!(player.is_human());
/// ```
pub trait Convert<Obj>: Is<Obj>
where
    Obj: ?Sized
{
    fn convert_from(object: Box<Obj>) -> Box<Self>;
    fn convert(object: &mut Box<Obj>);
    fn convert_get(object: &mut Box<Obj>) -> &Self;
    fn convert_get_mut(object: &mut Box<Obj>) -> &mut Self;
}
impl<To, Obj> Convert<Obj> for To
where
    To: Is<Obj> + Unsize<Obj> + 'static,
    Obj: ConvertInto<Self> + AsAny + ?Sized + 'static,
    Box<Obj>: Clone
{
    fn convert_from(object: Box<Obj>) -> Box<Self>
    {
        object.convert_into()
    }
    fn convert(object: &mut Box<Obj>)
    {
        if Self::downcast_from_ref(&**object).is_none()
        {
            *object = object.clone().convert_into()
        }
    }
    fn convert_get(object: &mut Box<Obj>) -> &Self
    {
        Self::convert(object);
        (&**object).downcast_ref().unwrap()
    }
    fn convert_get_mut(object: &mut Box<Obj>) -> &mut Self
    {
        Self::convert(object);
        (&mut **object).downcast_mut().unwrap()
    }
}