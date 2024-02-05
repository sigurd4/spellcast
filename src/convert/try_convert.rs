use std::marker::Unsize;

use crate::downcast::{DowncastRef, DowncastFromRef};

use super::*;

pub trait TryConvert<Obj>: Is<Obj>
where
    Obj: ?Sized
{
    fn try_convert_from(object: Box<Obj>) -> Result<Box<Self>, Box<Obj>>;
    fn try_convert(object: &mut Box<Obj>) -> bool;
    fn try_convert_get(object: &mut Box<Obj>) -> Option<&Self>;
    fn try_convert_get_mut(object: &mut Box<Obj>) -> Option<&mut Self>;
}
impl<'a, To, Obj> TryConvert<Obj> for To
where
    To: Is<Obj> + Unsize<Obj> + ?Sized + DowncastFromRef<Obj>,
    Obj: Is<Obj> + TryConvertInto<To, Obj> + DowncastRef<To> + ?Sized,
    Box<Obj>: Clone
{
    fn try_convert_from(object: Box<Obj>) -> Result<Box<Self>, Box<Obj>>
    {
        object.try_convert_into()
    }
    fn try_convert(object: &mut Box<Obj>) -> bool
    {
        if Self::downcast_from_ref(&**object).is_none()
        {
            match object.clone().try_convert_into()
            {
                Ok(obj) => {
                    *object = obj;
                },
                Err(obj) => {
                    *object = obj;
                    return false;
                },
            }
        }
        true
    }
    fn try_convert_get(object: &mut Box<Obj>) -> Option<&Self>
    {
        if !Self::try_convert(object)
        {
            return None
        }
        (&**object).downcast_ref()
    }
    fn try_convert_get_mut(object: &mut Box<Obj>) -> Option<&mut Self>
    {
        if !Self::try_convert(object)
        {
            return None
        }
        (&mut **object).downcast_mut()
    }
}