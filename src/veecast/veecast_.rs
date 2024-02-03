use crate::{downcast::Downcast, upcast::Upcast};

use super::*;

/// A trait for casting between two trait objects, using a mutual struct.
/// 
/// # Example
/// ```rust
/// use spellcast::veecast::*;
/// use core::any::Any;
/// 
/// trait ControlObj: Any {}
/// trait PlayerObj: Any {}
/// 
/// struct Human;
/// 
/// impl ControlObj for Human {}
/// impl PlayerObj for Human {}
/// 
/// let player = Box::new(Human) as Box<dyn PlayerObj>;
/// 
/// let control: Option<&dyn ControlObj> = (&*human).veecast_ref::<Human>();
/// ```
pub trait Veecast<To, Obj>: VeecastRef<To>
where
    To: ?Sized,
    Obj: ?Sized
{
    fn veecast<Struct>(self: Box<Self>) -> Result<Box<To>, Box<Obj>>
    where
        Self: Downcast<Struct, Obj>,
        Struct: Upcast<To>;
}
impl<From, To, Obj> Veecast<To, Obj> for From
where
    From: ?Sized,
    To: ?Sized,
    Obj: ?Sized
{
    fn veecast<Struct>(self: Box<Self>) -> Result<Box<To>, Box<Obj>>
    where
        Self: Downcast<Struct, Obj>,
        Struct: Upcast<To>
    {
        self.downcast().map(|vee| vee.upcast())
    }
}