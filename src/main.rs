use tgr::{
    engine::{self, *},
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

struct cursor;
impl Module for cursor {
    fn update(&self, obj: &mut Node2d, _d: f64) {
        obj.position = *get_data::<Vec2>("cursor").unwrap();
    }
}

struct cursorx;
impl Module for cursorx {
    fn touch(&self, obj: &mut Node2d, _id: u64, touch: &Touch, pos: Vec2) {
        match touch {
            Press => {
                set_data("objpos", vec2(obj.get_global_position().x - 75. - pos.x, obj.get_global_position().y));
                println!("{} {} {}", obj.get_global_position().x - 75. - pos.x, obj.get_global_position().y, 5)
            }
            Move => {}
            Relese => {}
        }

        set_data("cursor", get_data::<Vec2>("objpos").unwrap() + vec2(pos.x, get_camera().y));

        //get_data::<Vec2>("cursor").unwrap()
    }
}

struct cursory;
impl Module for cursory {
    fn touch(&self, obj: &mut Node2d, _id: u64, touch: &Touch, pos: Vec2) {
        match touch {
            Press => {
                set_data("objpos", vec2(obj.get_global_position().x, (obj.get_global_position().y + 75.) - pos.y));
            }
            Move => {}
            Relese => {}
        }

        set_data("cursor", get_data::<Vec2>("objpos").unwrap() + vec2(0., pos.y));

        //get_data::<Vec2>("cursor").unwrap()
    }
}


struct object;
impl Module for object {
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

struct bg;
impl Module for bg {
    fn touch(&self, obj: &mut Node2d, _id: u64, touch: &Touch, pos: Vec2) {
        match touch {
            Press => {
                set_data("bgpos", get_camera() - pos);
                set_data("objname", String::from(""));
                set_data("cursor", vec2(10000., 10000.));
            }
            Move => {
                let pos = pos - get_data::<Vec2>("bgpos").unwrap() + get_camera();
                set_camera(pos.x, pos.y);
            }
            Relese => {
                if obj.position - pos == *get_data::<Vec2>("bgpos").unwrap() {
                    let pos = pos - obj.position;
                    obj.get_node("objects").unwrap().add_node(vec![rect(
                        &format!("{}", get_stat(0)).to_string(),
                        100.,
                        100.,
                        10.,
                    )
                    .position(pos.x, pos.y)
                    .script(&object)]);
                    add_stat(0, 1.);
                }
            }
        }
    }
}

fn main() {
    set_data("bgpos", vec2(0., 0.));
    set_stat(0, 0.);

    set_data("objname", String::from(""));
    set_data("objpos", vec2(0., 0.));

    set_data("cursor", vec2(10000., 10000.));

    let node = node2d!(
        rect("objects", 10000., 10000., 0.)
            .color(rgb(31, 31, 31))
            .node(vec![circle("a", 100.)]),
        Node2d::new("cursor", Obj2d::None)
            .node(vec![rect("x", 150., 15., 7.5)
                .position(75., 0.)
                .color(rgb(255, 0, 0)).script(&cursorx),
                rect("y", 15., 150., 7.5)
                .position(0., -75.)
                .color(rgb(0, 255, 0)).script(&cursory)])
            .script(&cursor)
    )
    .script(&bg);

    Engine
        .node2d(node)
        .view(KeepHeight, KeepHeight)
        .start("Tarantula");
}
