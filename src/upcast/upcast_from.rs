use super::*;

/// A trait for upcasting types into a trait object
/// 
/// # Examples
/// ```rust
/// use spellcast::upcast::*;
/// 
/// trait EntityObj {}
/// trait PlayerObj: EntityObj {}
/// 
/// struct Human;
/// 
/// impl EntityObj for Human {}
/// impl PlayerObj for Human {}
/// 
/// let human = Box::new(Human);
/// 
/// let player = <dyn PlayerObj>::upcast_from(human);
/// let entity = <dyn EntityObj>::upcast_from(player);
/// ```
pub trait UpcastFrom<From>
where
    From: ?Sized
{
    /// Upcasts a type into a trait object
    /// 
    /// # Examples
    /// ```rust
    /// use spellcast::upcast::*;
    /// 
    /// trait EntityObj {}
    /// trait PlayerObj: EntityObj {}
    /// 
    /// struct Human;
    /// 
    /// impl EntityObj for Human {}
    /// impl PlayerObj for Human {}
    /// 
    /// let human = Box::new(Human);
    /// 
    /// let player = <dyn PlayerObj>::upcast_from_ref(&*human);
    /// let entity = <dyn EntityObj>::upcast_from_ref(player);
    /// ```
    fn upcast_from_ref(from: &From) -> &Self;
    
    /// Upcasts a type into a trait object
    /// 
    /// # Examples
    /// ```rust
    /// use spellcast::upcast::*;
    /// 
    /// trait EntityObj {}
    /// trait PlayerObj: EntityObj {}
    /// 
    /// struct Human;
    /// 
    /// impl EntityObj for Human {}
    /// impl PlayerObj for Human {}
    /// 
    /// let mut human = Box::new(Human);
    /// 
    /// let player = <dyn PlayerObj>::upcast_from_mut(&mut *human);
    /// let entity = <dyn EntityObj>::upcast_from_mut(player);
    /// ```
    fn upcast_from_mut(from: &mut From) -> &mut Self;
    
    /// Upcasts a type into a trait object
    /// 
    /// # Examples
    /// ```rust
    /// use spellcast::upcast::*;
    /// 
    /// trait EntityObj {}
    /// trait PlayerObj: EntityObj {}
    /// 
    /// struct Human;
    /// 
    /// impl EntityObj for Human {}
    /// impl PlayerObj for Human {}
    /// 
    /// let human = Box::new(Human);
    /// 
    /// let player = <dyn PlayerObj>::upcast_from(human);
    /// let entity = <dyn EntityObj>::upcast_from(player);
    /// ```
    fn upcast_from(from: Box<From>) -> Box<Self>;
}

impl<From, To> UpcastFrom<From> for To
where
    From: Upcast<To> + ?Sized,
    To: ?Sized
{
    fn upcast_from_ref(from: &From) -> &Self
    {
        from.upcast_ref()
    }
    fn upcast_from_mut(from: &mut From) -> &mut Self
    {
        from.upcast_mut()
    }
    fn upcast_from(from: Box<From>) -> Box<Self>
    {
        from.upcast()
    }
}