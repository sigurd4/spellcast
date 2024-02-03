use crate::{downcast::DowncastRef, upcast::Upcast};

use super::*;

pub trait VeecastRef<To>
where
    To: ?Sized
{
    fn veecast_ref<Struct>(self: &Self) -> Option<&To>
    where
        Self: DowncastRef<Struct>,
        Struct: Upcast<To> + 'static;
    fn veecast_mut<Struct>(self: &mut Self) -> Option<&mut To>
    where
        Self: DowncastRef<Struct>,
        Struct: Upcast<To> + 'static;
}
impl<From, To> VeecastRef<To> for From
where
    From: ?Sized,
    To: ?Sized
{
    fn veecast_ref<Struct>(self: &Self) -> Option<&To>
    where
        Self: DowncastRef<Struct>,
        Struct: Upcast<To> + 'static
    {
        self.downcast_ref().map(|vee| vee.upcast_ref())
    }
    fn veecast_mut<Struct>(self: &mut Self) -> Option<&mut To>
    where
        Self: DowncastRef<Struct>,
        Struct: Upcast<To> + 'static
    {
        self.downcast_mut().map(|vee| vee.upcast_mut())
    }
}