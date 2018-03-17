use std::ops::{Add, Sub, Mul, Div, Rem};

macro_rules! implement_op_trait {
    ( $op_trait_:ident for $type_:ident fn $method_:ident ) => (
        impl $op_trait_ for $type_ {
            type Output = Self;

            fn $method_(self, $type_(b): $type_) -> Self {
                let $type_(a) = self;
                $type_(a.$method_(&b))
            }
        }
    )
}

macro_rules! unit_struct {
    ( $name_:ident : $type_:ty ) => (
        #[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
        pub struct $name_(pub $type_);

        implement_op_trait!(Add for $name_ fn add);
        implement_op_trait!(Sub for $name_ fn sub);
        implement_op_trait!(Mul for $name_ fn mul);
        implement_op_trait!(Div for $name_ fn div);
        implement_op_trait!(Rem for $name_ fn rem);
    )
}

unit_struct!(Temperature: u16);
unit_struct!(Elevation: u8);
