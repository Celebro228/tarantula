use tgr::engine::*;

mod editor;

fn main() {
    Engine
        .node2d(editor::main())
        .backgraund(rgb(31, 31, 31))
        .view(KeepHeight, KeepHeight)
        .start("Tarantula");
}
