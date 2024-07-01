macro_rules! impl_do_property {
    (pub trait $trait_ty: ident where Self: $sub_ty: ty {
	    #[$property: literal]
	    fn $fn_name: ident($val_ty: ty);
    }) => {
	    pub trait $trait_ty {
			fn $fn_name(&self, end_val: $val_ty, duration: f64) -> SpireTween<Property<$val_ty>>;
		}
		
		impl<TClass: GodotClass + Inherits<$sub_ty> + Inherits<Object>> $trait_ty for Gd<TClass> {
			fn $fn_name(&self, end_val: $val_ty, duration: f64) -> SpireTween<Property<$val_ty>> {
				self.do_property($property, end_val, duration)
			}
		}
    };
}

pub(crate) use impl_do_property;
