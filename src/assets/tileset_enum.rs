/// How to use:
/// Define:
/// - $enum_name: The name of the enum to generate.
/// - $tile_size: The size of each tile in the tileset.
/// - ($size_x, $size_y): The size of the texture atlas in pixels.
/// - $variant => ($x, $y): Each variant of the enum and its corresponding top-left corner position
///   in the texture atlas, in pixels.
#[macro_export]
macro_rules! tileset_enum {
    ($enum_name:ident, $tile_size:expr, ($size_x:expr, $size_y:expr),
    $($variant:ident => ($x:expr, $y:expr)),* $(,)?) => {
        paste::paste! {
            tileset_enum!(@build_enum $enum_name [] [] ; $($variant),*);

            impl [<$enum_name TileType>] {
                pub const COUNT: usize = tileset_enum!(@count $($variant),*);
            }

            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct [<$enum_name Tileset>] {
                pub atlas_size: bevy::math::UVec2,
                /// The position of the top-left corner of each sprite in the atlas, in pixels.
                sprites: [bevy::math::UVec2; [<$enum_name TileType>]::COUNT],
            }

            impl [<$enum_name Tileset>] {
                pub fn sprite_topleft_position(&self, sprite_type: [<$enum_name TileType>]) -> bevy::math::UVec2 {
                    self.sprites[sprite_type as usize]
                }

                pub fn sprite_uv_rect(&self, sprite_type: [<$enum_name TileType>]) -> bevy::math::Rect {
                    let topleft = self.sprite_topleft_position(sprite_type);
                    bevy::math::Rect::new(
                        topleft.x as f32 / self.atlas_size.x as f32,
                        topleft.y as f32 / self.atlas_size.y as f32,
                        (topleft.x + $tile_size.x) as f32 / self.atlas_size.x as f32,
                        (topleft.y + $tile_size.y) as f32 / self.atlas_size.y as f32,
                    )
                }
            }

            pub const TILEMAP: [<$enum_name Tileset>] = [<$enum_name Tileset>] {
                atlas_size: bevy::math::UVec2::new($size_x, $size_y),
                sprites: [
                    $(
                        bevy::math::UVec2::new($x, $y),
                    )*
                ],
            };
        }

    };

    (@build_enum $enum_name:ident [$($variants:tt)*] [$($seen:ident),*] ; $head:ident, $($tail:ident),+) => {
        tileset_enum!(@build_enum $enum_name [
            $($variants)*
            $head = tileset_enum!(@count $($seen),*),
        ] [$($seen,)* $head] ; $($tail),+);
    };

    (@build_enum $enum_name:ident [$($variants:tt)*] [$($seen:ident),*] ; $last:ident) => {
        paste::paste!{
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(usize)]
            pub enum [<$enum_name TileType>] {
                $($variants)*
                $last = tileset_enum!(@count $($seen),*),
            }
        }
    };

    (@count $($variant:ident),*) => {
        <[()]>::len(&[$(tileset_enum!(@sub $variant)),*])
    };

    (@sub $variant:ident) => { () };
}
