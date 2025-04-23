use tgr::{engine::*, node2d};

use std::f32::consts::PI;

static ROTATION_POS: f32 = 70.71068;
static ROTATION_PI: f32 = 2.3561945;

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
        //println!("{}", get_fps())
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
                        obj.get_global_position().x - 50. - pos.x,
                        obj.get_global_position().y,
                    ),
                );

                obj.color.a = 0.45;
            }
            Move => {
                set_data(
                    "cursor",
                    get_data::<Vec2>("objpos").unwrap() + vec2(pos.x, 0.),
                );
            }
            Relese => {
                obj.color.a = 0.9;
            }
        }

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
                        (obj.get_global_position().y + 50.) - pos.y,
                    ),
                );

                obj.color.a = 0.45;
            }
            Move => {
                set_data(
                    "cursor",
                    get_data::<Vec2>("objpos").unwrap() + vec2(0., pos.y),
                );
            }
            Relese => {
                obj.color.a = 0.9;
            }
        }

        //get_data::<Vec2>("cursor").unwrap()
    }
}

struct CursorR;
impl Module for CursorR {
    fn touch(&self, obj: &mut Node2d, _id: u64, touch: &Touch, pos: Vec2) {
        match touch {
            Press => {
                set_stat(2, *get_stat(1));
                //set_stat(3, (obj.get_global_position().x - 50. - pos.x).atan2(
                //    (obj.get_global_position().y + 50.) - pos.y));

                obj.color.a = 0.45;
            }
            Move => {
                let center = obj.get_global_position() - obj.position;
                let a = (pos.y - center.y).atan2(pos.x - center.x);

                obj.position = vec2(a.cos() * ROTATION_POS, a.sin() * ROTATION_POS);
                obj.rotation = a;

                set_stat(1, *get_stat(2) + (a + PI / 4.));
            }
            Relese => {
                obj.position = vec2(50., -50.);
                obj.rotation = ROTATION_PI;
                obj.color.a = 0.9;
            }
        }

        //get_data::<Vec2>("cursor").unwrap()
    }
}

struct Object;
impl Module for Object {
    //fn start

    fn update(&self, obj: &mut Node2d, _d: f64) {
        if obj.name == *get_data::<String>("objname").unwrap() {
            obj.position = *get_data::<Vec2>("cursor").unwrap();
            obj.rotation = *get_stat(1);
        }
    }

    fn touch(&self, obj: &mut Node2d, _id: u64, touch: &Touch, pos: Vec2) {
        match touch {
            Press => {
                set_data("objpos", obj.position - pos);
                set_data("objname", obj.name.clone());
                set_stat(1, obj.rotation);
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
    fn update(&self, _obj: &mut Node2d, _d: f64) {
        let mut y = -get_mouse_wheel_d().y / 240.;

        if y != 0. {
            y = get_stat(3) + y;
            set_stat(3, y);

            set_zoom(if y > 0. {
                y
            } else {
                1. / (y.abs() + 3.)
            });
        }
    }

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
    set_stat(3, 0.);

    set_data("objname", String::from(""));
    set_data("objpos", vec2(0., 0.));

    set_data("cursor", vec2(10000., 10000.));
    set_stat(1, 0.);

    let mut bg = Vec::new();

    for i in -50..50 {
        bg.push(
            rect(&i.to_string(), 10000., 1., 0.)
                .position(0., i as f32 * 50.)
                .color(rgba(50, 50, 50, 1.)),
        );
    }

    for i in -50..50 {
        bg.push(
            rect(&(100 + i).to_string(), 1., 10000., 0.)
                .position(i as f32 * 50., 0.)
                .color(rgba(50, 50, 50, 1.)),
        );
    }

    bg.push(rect("y", 1., 10000., 0.).color(rgb(0, 255, 0)));
    bg.push(rect("x", 10000., 1., 0.).color(rgb(255, 0, 0)));

    node2d!(
        Node2d::new("bg", Obj2d::None)
            .node(bg),
        Node2d::new("objects", Obj2d::None),
        Node2d::new("cursor", Obj2d::None)
            .node(vec![
                rect("y", 15., 50., 0.)
                    .position(0., -50.)
                    .color(rgba(0, 255, 0, 0.9))
                    .script(&CursorY),
                rect("x", 50., 15., 0.)
                    .position(50., 0.)
                    .color(rgba(255, 0, 0, 0.9))
                    .script(&CursorX),
                rect("r", 15., 40., 0.)
                    .position(50., -50.)
                    .rotation(ROTATION_PI)
                    .color(rgba(255, 255, 0, 0.9))
                    .script(&CursorR),
            ])
            .script(&Cursor)
    )
    .script(&Bg)
}
