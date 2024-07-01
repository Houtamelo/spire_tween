macro_rules! impl_ref_from_any {
	($outer_path: path {
		$($tween_ty: ty => $enum_var: path),+
		$(,)?
	}) => {
		pub mod conversions {
			use crate::internal::*;
			
			$(
				impl TweenConvert for $tween_ty {
					fn ref_from_any(tween: &mut AnyTween) -> Option<&mut SpireTween<Self>> {
						if let $outer_path($enum_var(t)) = tween {
							Some(t)
						} else {
							None
						}
					}
					
					fn from_any(tween: AnyTween) -> Result<SpireTween<Self>, AnyTween> {
						if let $outer_path($enum_var(t)) = tween {
							Ok(*t)
						} else {
							Err(tween)
						}
				    }
				}
			    
			    impl From<SpireTween<$tween_ty>> for AnyTween {
					fn from(tween: SpireTween<$tween_ty>) -> Self {
						$outer_path($enum_var(Box::new(tween))) 
					}
			    }
			)+
		}
    };
}

pub(crate) use impl_ref_from_any;

macro_rules! delegate_impls {
    (
	    pub enum $enum_ident: ident {
			$($var_ident: ident($var_field: ty)),* 
			$(,)?
		}
	    
	    pub trait $trait_ident: ident {
		    $(
		        fn $fn_ident: ident $(<$($gen: ident),+>)? (
			        self: $self_ty: ty,
			        $($args: ident: $arg_tys: ty),*
		        ) -> $fn_ret_ty: ty;
		    )*
	    }
    ) => {
	    delegate_impls!(1
	        ENUM: $enum_ident
	        TRAIT: $trait_ident
	        VARIANTS: { $($var_ident($var_field)),* }
	        FUNCTIONS: 
			    $(
			        fn $fn_ident $(<$($gen),+>)? (
				        self: $self_ty,
				        $($args: $arg_tys),*
			        )-> $fn_ret_ty;
			    )*
	    );
    };
	(1
		ENUM: $enum_ident: ident
		TRAIT: $trait_ident: ident
		VARIANTS: $var_idents: tt
		FUNCTIONS: 
			$(
				fn $fn_ident: ident $(<$($gen: ident),+>)? (
					self: $self_ty: ty,
					$($args: ident: $arg_tys: ty),*
				) -> $fn_ret_ty: ty;
			)*
	) => {
		delegate_impls!(2
			ENUM: $enum_ident
	        TRAIT: $trait_ident
			$(
				VARIANTS: [ $var_idents | { $($args),* } ]
				fn $fn_ident $(<$($gen),+>)? (
					self: $self_ty,
					$($args: $arg_tys),*
				) -> $fn_ret_ty;
			)*
		);
	};
	(2 
		ENUM: $enum_ident: ident
		TRAIT: $trait_ident: ident
		$(
			VARIANTS: [ { $($var_ident: ident($var_field: ty)),* } | $args_call: tt]
			fn $fn_ident: ident $(<$($gen: ident),+>)? (
				self: $self_ty: ty,
				$($args: ident: $arg_tys: ty),*
			) -> $fn_ret_ty: ty;
		)*
	) => {
		delegate_impls!(3
			ENUM: $enum_ident
	        TRAIT: $trait_ident
			$(
				VARIANTS: [ $($var_ident($var_field) | $args_call),* ]
				fn $fn_ident $(<$($gen),+>)? (
					self: $self_ty,
					$($args: $arg_tys),*
				) -> $fn_ret_ty;
			)*
		);
	};
	(3 
		ENUM: $enum_ident: ident
		TRAIT: $trait_ident: ident
		$(
			VARIANTS: [ $($var_ident: ident($var_field: ty) | { $($args_call: ident),* }),* ]
			fn $fn_ident: ident $(<$($gen: ident),+>)? (
				self: $self_ty: ty,
				$($args: ident: $arg_tys: ty),*
			) -> $fn_ret_ty: ty;
		)*
	) => {
		impl $trait_ident for $enum_ident {
			$(
				fn $fn_ident $(<$($gen),+>)? (
					self: $self_ty,
					$($args: $arg_tys),*
				) -> $fn_ret_ty {
			        match self {
			            $(
			                $enum_ident::$var_ident(variant) => {
			                    variant.$fn_ident($($args_call),*)
			                }
			            ),*
			        } 
				}
			)*
		}
	};
}

pub(crate) use delegate_impls;

macro_rules! impl_from_variants {
    (
	    pub enum $enum_ident: ident {
			$($var_ident: ident($var_field: ty)),* 
			$(,)?
		}
    ) => {
	    $(
	      impl From<$var_field> for $enum_ident {
		      fn from(variant: $var_field) -> Self {
		          $enum_ident::$var_ident(variant)
		      }
	      }
	    )*
    };
}

pub(crate) use impl_from_variants;