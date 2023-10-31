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
            chain.next_chunk::<$len>().unwrap().as_slice()
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

