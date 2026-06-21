#[macro_export]
macro_rules! tileset_enum {
    ($enum_name:ident, $($variant:ident => $x:expr, $y:expr),* $(,)?) => {
        tileset_enum!(@build_enum $enum_name [] [] ; $($variant),*);

        impl $enum_name {
            const COUNT: usize = tileset_enum!(@count $($variant),*);

            const VARIANTS: [Self; Self::COUNT] = [
                $(
                    Self::$variant,
                )*
            ];

            pub const fn index(self) -> usize {
                self as usize
            }
        }

        const SPRITES: [UVec2; $enum_name::COUNT] = [
            $(
                UVec2::new($x, $y),
            )*
        ];
    };

    (@build_enum $enum_name:ident [$($variants:tt)*] [$($seen:ident),*] ; $head:ident, $($tail:ident),+) => {
        tileset_enum!(@build_enum $enum_name [
            $($variants)*
            $head = tileset_enum!(@count $($seen),*),
        ] [$($seen,)* $head] ; $($tail),+);
    };

    (@build_enum $enum_name:ident [$($variants:tt)*] [$($seen:ident),*] ; $last:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(usize)]
        pub enum $enum_name {
            $($variants)*
            $last = tileset_enum!(@count $($seen),*),
        }
    };

    (@count $($variant:ident),*) => {
        <[()]>::len(&[$(tileset_enum!(@sub $variant)),*])
    };

    (@sub $variant:ident) => { () };
}
