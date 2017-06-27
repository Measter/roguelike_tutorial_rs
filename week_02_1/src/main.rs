extern crate tcod;
use tcod::RootConsole;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HIGHT: i32 = 50;

fn main() {
    let mut root = RootConsole::initializer()
                    .size(SCREEN_WIDTH, SCREEN_HIGHT)
                    .title("Roguelike Tutorial")
                    .fullscreen(false)
                    .init();

    while !root.window_closed() {
        root.flush();
        let key = root.wait_for_keypress(true);
    }
}
