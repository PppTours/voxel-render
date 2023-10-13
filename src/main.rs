use raylib::{
    ffi::{mint::Vector3, Camera3D, CameraMode, CameraProjection},
    prelude::{Color, RaylibDraw},
};

fn main() {
    let (rl, t) = raylib::init().vsync().build();
    rl.disable_cursor();

    let mut camera = Camera3D {
        position: Vector3 {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        },
        target: Vector3 {
            x: 1f32,
            y: 1f32,
            z: 1f32,
        },
        up: Vector3 {
            x: 0f32,
            y: 1f32,
            z: 0f32,
        },
        fovy: 70f32,
        projection: CameraProjection::CAMERA_PERSPECTIVE as _,
    };

    while !rl.window_should_close() {
        rl.update_camera(&mut camera, CameraMode::CAMERA_FIRST_PERSON);

        rl.begin_drawing(&t, |d| {
            d.clear_background(Color::SKYBLUE);

            d.begin_camera_3d(&camera, || {
                d.draw_cube(
                    Vector3 {
                        x: 1f32,
                        y: 1f32,
                        z: 1f32,
                    },
                    0.5f32,
                    0.5f32,
                    0.5f32,
                    Color::RED,
                );

                d.draw_cube_wires(
                    Vector3 {
                        x: 1f32,
                        y: 1f32,
                        z: 1f32,
                    },
                    0.51f32,
                    0.51f32,
                    0.51f32,
                    Color::BLACK,
                );
            });

            d.draw_fps(10, 10);
        })
    }
}
