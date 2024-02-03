use super::*;

pub unsafe trait Object<Trait>
where
    Trait: ?Sized
{

}
unsafe impl<Trait, T> Object<Trait> for T
where
    Trait: ?Sized,
    T: ?Sized
    + Is<Trait>
    + crate::upcast::Upcast<Trait>
    + crate::downcast::Downcast<Trait, Trait>
{

}

#[macro_export]
macro_rules! impl_object {
    ($trait:path) => {
        impl spellcast::downcast::DowncastFromRef<dyn $trait> for dyn $trait
        {
            fn downcast_from_ref(from: &dyn $trait) -> Option<&Self>
            {
                Some(from)
            }
        
            fn downcast_from_mut(from: &mut dyn $trait) -> Option<&mut Self>
            {
                Some(from)
            }
        }
        impl spellcast::downcast::DowncastFrom<dyn $trait, dyn $trait> for dyn $trait
        {
            fn downcast_from(from: Box<dyn $trait>) -> Result<Box<Self>, Box<dyn $trait>>
            {
                Ok(from)
            }
        }
        impl spellcast::convert::TryConvertInto<dyn $trait, dyn $trait> for dyn $trait
        {
            fn try_convert_into(self: Box<Self>) -> Result<Box<dyn $trait>, Box<dyn $trait>>
            {
                Ok(self)
            }
        }
        //spellcast::assert_is!(dyn $trait: $trait);
    };
}
pub macro assert_is {
    ($type:ty : $trait:path) => {
        static_assertions::assert_obj_safe!(crate::Object<dyn $trait>);
        static_assertions::assert_impl_one!(dyn core::any::Any: core::marker::Unsize<dyn core::any::Any>);
        static_assertions::assert_impl_one!(dyn $trait: crate::Is<dyn $trait>);
        static_assertions::assert_impl_one!($type: core::marker::Unsize<dyn $trait>);
        static_assertions::assert_impl_one!($type: crate::Is<dyn $trait>);
        static_assertions::assert_impl_one!($type: crate::upcast::Upcast<dyn $trait>);
        static_assertions::assert_impl_one!(dyn $trait: crate::downcast::DowncastFrom<dyn $trait, dyn $trait>);
        static_assertions::assert_impl_one!(dyn $trait: crate::downcast::DowncastFromRef<dyn $trait>);
        static_assertions::assert_impl_one!(dyn $trait: crate::downcast::DowncastRef<dyn $trait>);
        static_assertions::assert_impl_one!(dyn $trait: crate::downcast::Downcast<dyn $trait, dyn $trait>);
        static_assertions::assert_impl_one!(dyn $trait: crate::downcast::Downcast<$type, dyn $trait>);
        static_assertions::assert_impl_one!(dyn $trait: crate::Object<dyn $trait>);
    }
}