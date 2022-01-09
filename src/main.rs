use ray_tracer::{
    img::generate_ppm,
    math::Vec3,
    {Camera, Color, Light, RayTracer, SceneObject, Screen, Sphere},
};
use std::fs;

fn main() {
    let scene_objects: Vec<Box<dyn SceneObject>> = vec![
        Box::new(Sphere::new(
            Vec3::new(-0.2, 0.2, 2.0),
            0.7,
            Color::new(0.8, 0.2, 0.2),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.1, -0.1, 1.0),
            0.1,
            Color::new(0.8, 0.3, 0.7),
        )),
        Box::new(Sphere::new(
            Vec3::new(-0.3, 0.2, 0.8),
            0.2,
            Color::new(0.2, 0.9, 0.4),
        )),
        Box::new(Sphere::new(
            Vec3::new(0.0, -20001.0, 2.0),
            20000.0,
            Color::new(0.3, 0.3, 0.9),
        )),
    ];
    let camera = Camera::new(Vec3::new(0.0, 0.0, -1.5));
    let (width, height) = (900, 600);
    let screen = Screen::new(
        (width, height),
        Vec3::new(-1.0, 1.0 / (width as f64 / height as f64), 0.0),
        Vec3::new(1.0, -1.0 / (width as f64 / height as f64), 0.0),
    );
    let light = Light::new(Vec3::new(1.0, 1.0, 0.0));
    let mut tracer = RayTracer::new(scene_objects, camera, screen, light);

    tracer.render();

    let pixels = tracer
        .get_pixels()
        .iter()
        .map(|color| {
            (
                (color.red() * 255.0) as u32,
                (color.green() * 255.0) as u32,
                (color.blue() * 255.0) as u32,
            )
        })
        .collect::<Vec<(u32, u32, u32)>>();

    fs::write(
        "images/render.ppm",
        generate_ppm((width, height), 255, pixels),
    )
    .expect("Cannot write to ppm file");
}
