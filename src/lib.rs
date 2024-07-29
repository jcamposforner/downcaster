use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;

    fn type_name(&self) -> &'static str;
}

impl<T: Any> AsAny for T {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }

    #[inline(always)]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    #[inline(always)]
    fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    #[inline(always)]
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

pub trait Downcast: AsAny {
    #[inline]
    fn is<T: AsAny>(&self) -> bool {
        self.as_any().is::<T>()
    }

    #[inline]
    fn downcast_ref<T: AsAny>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    #[inline]
    fn downcast_mut<T: AsAny>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut::<T>()
    }

    #[inline]
    fn downcast<T: AsAny>(self: Box<Self>) -> Result<Box<T>, Box<Self>> where Self: Sized {
        if self.is::<T>() {
            unsafe {
                let raw: *mut dyn Any = Box::into_raw(self);
                let raw = raw as *mut T;
                Ok(Box::from_raw(raw))
            }
        } else {
            Err(self)
        }
    }
}

impl<T: ?Sized + AsAny> Downcast for T {}

macro_rules! downcast {
    ($value:expr, $type:ty) => {
        $value.downcast::<$type>()
    };
    () => {};
}

macro_rules! downcast_ref {
    ($value:expr, $type:ty) => {
        $value.downcast_ref::<$type>()
    };
    () => {};
}

macro_rules! downcast_mut {
    ($value:expr, $type:ty) => {
        $value.downcast_mut::<$type>()
    };
    () => {};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_any() {
        let value = 42;

        assert_eq!(value.downcast_ref::<i32>(), Some(&42));
    }


    #[test]
    fn test_as_any_macro() {
        let value = 42;

        assert_eq!(downcast_ref!(value, i32), Some(&42));
    }

    #[test]
    fn as_any_mut_macro() {
        let mut value = 42;

        assert_eq!(downcast_mut!(value, i32), Some(&mut 42));
    }

    #[test]
    fn as_any_mut() {
        let mut value = 42;

        assert_eq!(value.downcast_mut::<i32>(), Some(&mut 42));
    }

    #[test]
    fn into_any_macro() {
        let value= Box::new(42);
        let any = value.as_any_box();
        let option = downcast!(any, i32);

        assert!(option.is_ok());
    }


    #[test]
    fn into_any() {
        let value= Box::new(42);
        let any = value.as_any_box();
        let option = any.downcast::<i32>();

        assert!(option.is_ok());
    }

    #[test]
    fn type_name() {
        let value = 42;
        assert_eq!(value.type_name(), "i32");
    }
}