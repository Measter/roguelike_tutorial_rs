extern crate tcod;
use tcod::RootConsole;
use tcod::{Console};
use tcod::console::{FontLayout, FontType, BackgroundFlag};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HIGHT: i32 = 50;

fn main() {
    let mut root = RootConsole::initializer()
                    .size(SCREEN_WIDTH, SCREEN_HIGHT)
                    .title("Roguelike Tutorial")
                    .fullscreen(false)
                    .font("arial10x10.png", FontLayout::Tcod)
                    .font_type(FontType::Greyscale)
                    .init();

    root.set_default_foreground(tcod::colors::WHITE);

    while !root.window_closed() {
        root.put_char(1, 1, '@', BackgroundFlag::None);
        root.flush();

        let key = root.wait_for_keypress(true);
    }
}
