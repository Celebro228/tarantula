use tgr::engine::*;

mod project;
mod editor;

static BLACK: Rgba = Rgba::new(0.09, 0.09, 0.09, 1.);

fn main() {
    Engine
        .node2d(editor::main())
        .backgraund(rgb(31, 31, 31))
        .view(Window, Window)
        .start("Tarantula");
}

/*
Фон#
Вращение обьектов#
Инфа обьекта
Экран*
Создание обьектов*
Компиляция*
*/
