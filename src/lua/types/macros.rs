macro_rules! get_field {
    ( $fields:ident, [_1] $name:ident $( ; pre = $( $pre:tt )+ )? $( ; post = $( $post:tt )+ )? ) => {
        $fields.add_field_method_get(stringify!($name), |_lua, this| Ok(this.1. $($($pre)+.)? $name. $($($post)+.)?to_user_data()));
    };
    ( $fields:ident, [_1] $name:ident $( ; mid = $( $mid:tt )+ )? $( ; pre = $( $pre:tt )+ )? $( ; post = $( $post:tt )+ )? ) => {
        $fields.add_field_method_get(stringify!($name), |_lua, this| Ok(this.1. $($($pre)+.)? $($($mid)+.)? $($($post)+.) ?to_user_data()));
    };
    ( $fields:ident, $name:ident $( ; pre = $( $pre:tt )+ )? $( ; post = $( $post:tt )+ )? ) => {
        $fields.add_field_method_get(stringify!($name), |_lua, this| Ok(this.0. $($($pre)+.)? $name. $($($post)+.)?to_user_data()));
    };
    ( $fields:ident, $name:ident $( ; mid = $( $mid:tt )+ ) ?$( ; pre = $( $pre:tt )+ )? $( ; post = $( $post:tt )+ )? ) => {
        $fields.add_field_method_get(stringify!($name), |_lua, this| Ok(this.0. $($($pre)+.)? $($($mid)+.)? $($($post)+.) ?to_user_data()));
    };

}

macro_rules! get_fields {
    ( $fields:ident, $( [$($args:tt)*] ),+ $(,)? ) => {
        $(
            crate::lua::types::macros::get_field!($fields, $($args)*);
        )+
    };
}

macro_rules! add_method {
    ( $methods:ident, $name:ident ) => {
        $methods.add_method(stringify!($name), |_lua, this, _args: ()| {
            Ok(this.0.$name().to_user_data())
        });
    };
    ( $methods:ident, $name:ident $(; pre=$( $pre:tt )+)? $(; post=$( $post:tt )+ )? ) => {
        $methods.add_method(stringify!($name), |_lua, this, _args: ()| {
            Ok(this.0.$($( $pre )+.)?$name()$($( $post )+.)?.to_user_data())
        });
    };
    ( $methods:ident, $name:ident $( $method:tt )+ ) => {
        $methods.add_method(stringify!($name), |_lua, this, _args: ()| {
            Ok(this.0.$( $method )+.to_user_data())
        });
    };

}

macro_rules! add_methods {
    ( $methods:ident, $( [$($args:tt)*] ),+ $(,)? ) => {
        $(
            crate::lua::types::macros::add_method!($methods, $($args)*);
        )+
    };
}

pub(in super::super) use {add_method, add_methods, get_field, get_fields};
