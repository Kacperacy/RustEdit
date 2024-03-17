mod editor;
use editor::*;

mod screen;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut editor = Editor::new();
    editor.open(args.get(1));
    editor.set_status_message("HELP: Ctrl-S = save | Ctrl-Q = quit | Ctrl-F = find".into());
    editor.run();
}
