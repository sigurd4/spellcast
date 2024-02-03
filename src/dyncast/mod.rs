moddef::moddef!(
    flat(pub) mod {
        dyncast_obj,
        dyncast_ref,
        dyncast_,
        try_dyncast_ref,
        try_dyncast
    }
);

use super::*;

#[macro_export]
macro_rules! impl_dyncast {
    ($sub:path : $super:path) => {
        impl<Medium> DyncastObj<dyn $sub, dyn $super> for Medium
        where
            Medium: $sub + $super + spellcast::Is<dyn $sub> + spellcast::Is<dyn $super> + ?Sized,
        {
            type Obj = dyn $sub;
        }
        impl<Medium> DyncastObj<dyn $super, dyn $sub> for Medium
        where
            Medium: $sub + $super + spellcast::Is<dyn $sub> + spellcast::Is<dyn $super> + ?Sized,
        {
            type Obj = dyn $sub;
        }
    };
}