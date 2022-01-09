use ray_tracer::{
    img::generate_ppm,
    math::Vec3,
    {Camera, Light, RayTracer, SceneObject, Screen, Sphere},
};

fn main() {
    let scene_objects: Vec<Box<dyn SceneObject>> = vec![Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 2.0),
        0.6,
        (0.6, 0.1, 0.4),
    ))];
    let camera = Camera::new(Vec3::new(0.0, 0.0, -1.0));
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
        .map(|(r, g, b)| ((r * 255.0) as u32, (g * 255.0) as u32, (b * 255.0) as u32))
        .collect::<Vec<(u32, u32, u32)>>();

    std::fs::write("image.ppm", generate_ppm((width, height), 255, pixels))
        .expect("Cannot write to ppm file");
}
