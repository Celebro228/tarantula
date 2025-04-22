use tgr::{
    engine::*,
    node2d,
};

/*
Сцена
Виджеты
Создание обьектов
Удаление обьектов
Создание проектов
Сохранение проектов
Загрузка проектов

*/

struct Cursor;
impl Module for Cursor {
    fn update(&self, obj: &mut Node2d, _d: f64) {
        obj.position = *get_data::<Vec2>("cursor").unwrap();
    }
}

struct CursorX;
impl Module for CursorX {
    fn touch(&self, obj: &mut Node2d, _id: u64, touch: &Touch, pos: Vec2) {
        match touch {
            Press => {
                set_data(
                    "objpos",
                    vec2(
                        obj.get_global_position().x - 75. - pos.x,
                        obj.get_global_position().y,
                    ),
                );
            }
            Move => {}
            Relese => {}
        }

        set_data(
            "cursor",
            get_data::<Vec2>("objpos").unwrap() + vec2(pos.x, 0.),
        );

        //get_data::<Vec2>("cursor").unwrap()
    }
}

struct CursorY;
impl Module for CursorY {
    fn touch(&self, obj: &mut Node2d, _id: u64, touch: &Touch, pos: Vec2) {
        match touch {
            Press => {
                set_data(
                    "objpos",
                    vec2(
                        obj.get_global_position().x,
                        (obj.get_global_position().y + 75.) - pos.y,
                    ),
                );
            }
            Move => {}
            Relese => {}
        }

        set_data(
            "cursor",
            get_data::<Vec2>("objpos").unwrap() + vec2(0., pos.y),
        );

        //get_data::<Vec2>("cursor").unwrap()
    }
}

struct Object;
impl Module for Object {
    //fn start

    fn update(&self, obj: &mut Node2d, _d: f64) {
        if obj.name == *get_data::<String>("objname").unwrap() {
            obj.position = *get_data::<Vec2>("cursor").unwrap();
        }
    }

    fn touch(&self, obj: &mut Node2d, _id: u64, touch: &Touch, pos: Vec2) {
        match touch {
            Press => {
                set_data("objpos", obj.position - pos);
                set_data("objname", obj.name.clone());
            }
            Move => {}
            Relese => {}
        }

        set_data("cursor", get_data::<Vec2>("objpos").unwrap() + pos);

        //get_data::<Vec2>("cursor").unwrap()
    }
}

struct Bg;
impl Module for Bg {
    fn touch(&self, obj: &mut Node2d, id: u64, touch: &Touch, pos: Vec2) {
        match touch {
            Press => {
                set_data("bgpos", pos);
                set_data("objname", String::from(""));
                set_data("cursor", vec2(10000., 10000.));
            }
            Move => {
                if id == 0 {
                    let pos = get_camera() - (pos - get_data::<Vec2>("bgpos").unwrap());
                    set_camera(pos.x, pos.y);
                }
            }
            Relese => {
                if id == 1 {
                    if pos == *get_data::<Vec2>("bgpos").unwrap() {
                        let pos = pos - obj.position;
                        obj.get_node("objects").unwrap().add_node(vec![rect(
                            &format!("{}", get_stat(0)).to_string(),
                            100.,
                            100.,
                            10.,
                        )
                        .position(pos.x, pos.y)
                        .script(&Object)]);
                        add_stat(0, 1.);
                    }
                }
            }
        }
    }
}

pub fn main() -> Node2d {
    set_data("bgpos", vec2(0., 0.));
    set_stat(0, 0.);

    set_data("objname", String::from(""));
    set_data("objpos", vec2(0., 0.));

    set_data("cursor", vec2(10000., 10000.));

    node2d!(
        Node2d::new("bg", Obj2d::None)
            .node(vec![
                rect("y", 1., 10000., 0.).color(rgb(0, 255, 0)),
                rect("x", 10000., 1., 0.).color(rgb(255, 0, 0)),
            ]),
        Node2d::new("objects", Obj2d::None),
        Node2d::new("cursor", Obj2d::None)
            .node(vec![
                rect("y", 15., 150., 7.5)
                    .position(0., -75.)
                    .color(rgb(0, 255, 0))
                    .script(&CursorY),
                rect("x", 150., 15., 7.5)
                    .position(75., 0.)
                    .color(rgb(255, 0, 0))
                    .script(&CursorX),
            ])
            .script(&Cursor)
    )
    .script(&Bg)
}
