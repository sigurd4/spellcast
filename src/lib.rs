#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(associated_type_bounds)]
#![feature(trait_alias)]
#![feature(new_uninit)]
#![feature(decl_macro)]

#![feature(specialization)]

moddef::moddef!(
    pub mod {
        downcast,
        upcast,
        veecast,
        //dyncast,
        convert
    },
    flat(pub) mod {
        is,
        as_any,
        object
    }
);

#[cfg(test)]
mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn it_works() {
        
    }
}

mod private
{
    use std::marker::Unsize;

    use crate::Is;
    
    pub trait IsObjOf<Type>
    where
        Type: ?Sized {}
    impl<Type, Trait> IsObjOf<Type> for Trait
    where
        Type: Is<Trait> + ?Sized,
        Trait: ?Sized {}
}