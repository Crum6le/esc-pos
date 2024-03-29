use crate::type_state::{BeepTheBuzzer, SelectUserDefinedCharacterSet};

#[macro_export]
macro_rules! def_const_bytes {
    (
        $(
            $const_ident:ident => $const_value:expr
        ),*
    ) => {
        $(
            pub const $const_ident: u8 = $const_value;
        )*
    };
}
#[macro_export]
macro_rules! def_cmd {
    (
        $(
            $const_ident:ident => [$($const_value:expr),*]
        ),*
    ) => {
        $(
            pub const $const_ident: [u8; {
                [$($const_value,)*].len()
            }] = [$($const_value,)*];
        )*
    };
}
#[macro_export]
macro_rules! gen_fixed_cmd {
    (
        $len:expr,
        $(
            $item:expr
        ),*
    ) => {
        {
            let chain = ::core::iter::empty();
            $(
                let chain = chain.chain($item);
            )*
            let mut chain = chain;
            chain.collect::<Vec<_>>().as_slice()
        }
    };
}
#[macro_export]
macro_rules! impl_trait {
    (
        $traitname:ident,
        [$(
            $structname:ident
        ),+]

    ) => {
        $(impl $traitname for $structname {})+
    };
}
macro_rules! define_printer {
    (
        printer_name: $printer_name:ident,
        $(
            select_userdefined_character_set: {
                x: $x:expr,
                y: $y:expr
            },
        )?
        ext_traits: [$(
            $ext_trait: ident
        ),*]
    ) => {
        pub struct $printer_name;
        $(
            impl SelectUserDefinedCharacterSet for $printer_name {
                const X: u8 = $x;
                const Y: u8 = $y;
            }
        )?
        $(
            impl $ext_trait for $printer_name {}
        ),*
    };
}
define_printer!(
    printer_name: EUM30L,
    select_userdefined_character_set: {
        x: 3,
        y: 12
    },
    ext_traits: [
        
    ]
);
