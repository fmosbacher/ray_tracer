use crate::math::*;
use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Color(f64, f64, f64);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(r, g, b)
    }

    pub fn red(self) -> f64 {
        self.0
    }

    pub fn green(self) -> f64 {
        self.1
    }

    pub fn blue(self) -> f64 {
        self.2
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(
            min(self.red() + other.red(), 1.0),
            min(self.green() + other.green(), 1.0),
            min(self.blue() + other.blue(), 1.0),
        )
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(
            min(self.red() * other.red(), 1.0),
            min(self.green() * other.green(), 1.0),
            min(self.blue() * other.blue(), 1.0),
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, value: f64) -> Self {
        Self(
            min(self.red() * value, 1.0),
            min(self.green() * value, 1.0),
            min(self.blue() * value, 1.0),
        )
    }
}

pub trait SceneObject {
    fn intersection_dist(&self, ray: Ray) -> f64;
    fn get_normal_at(&self, position: Vec3) -> Vec3;
    fn get_color(&self) -> Color;
}

pub struct Sphere {
    position: Vec3,
    radius: f64,
    color: Color,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f64, color: Color) -> Self {
        Self {
            position,
            radius,
            color,
        }
    }
}

impl SceneObject for Sphere {
    fn intersection_dist(&self, ray: Ray) -> f64 {
        let sphere_to_camera = ray.origin - self.position;

        let a = 1.; // Always 1 because direction is a unit vector -> Vec3::dot(dir, dir)
        let b = 2. * Vec3::dot(ray.direction, sphere_to_camera);
        let c = sphere_to_camera.len().powi(2) - self.radius.powi(2);

        let determinant = b * b - 4. * a * c;
        let det_sqrt = determinant.sqrt();

        if determinant > 0.0 {
            let d1 = (-b + det_sqrt) / 2. * a;
            let d2 = (-b - det_sqrt) / 2. * a;

            if d1 > 0.0 && d2 > 0.0 {
                if d1 < d2 {
                    d1
                } else {
                    d2
                }
            } else if d1 > 0. && d2 < 0. {
                d1
            } else if d1 < 0. && d2 > 0. {
                d2
            } else {
                f64::INFINITY
            }
        } else {
            f64::INFINITY
        }
    }

    fn get_normal_at(&self, position: Vec3) -> Vec3 {
        Vec3::unit(position - self.position)
    }

    fn get_color(&self) -> Color {
        self.color
    }
}

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: Vec3::unit(direction),
        }
    }

    fn position_at(&self, dist: f64) -> Vec3 {
        self.origin + self.direction * dist
    }

    fn get_nearest(
        self,
        scene_objects: &[Box<dyn SceneObject>],
    ) -> (Option<&dyn SceneObject>, f64) {
        let mut nearest_object: Option<&dyn SceneObject> = None;
        let mut closest_dist = f64::INFINITY;

        for object in scene_objects {
            let dist = object.intersection_dist(self);

            if dist < closest_dist {
                closest_dist = dist;
                nearest_object = Some(&**object)
            }
        }

        (nearest_object, closest_dist)
    }
}

pub struct Camera {
    position: Vec3,
}

impl Camera {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }
}

pub struct Light {
    position: Vec3,
}

impl Light {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }
}

pub struct Screen {
    resolution: (u32, u32),
    top_left_point: Vec3,
    bottom_right_point: Vec3,
    pixels: Vec<Color>,
}

impl Screen {
    pub fn new(resolution: (u32, u32), top_left_point: Vec3, bottom_right_point: Vec3) -> Self {
        Self {
            resolution,
            top_left_point,
            bottom_right_point,
            pixels: vec![],
        }
    }

    pub fn get_pixel_position(&self, row: u32, col: u32) -> Vec3 {
        let (width, height) = self.resolution;
        let tl = self.top_left_point;
        let br = self.bottom_right_point;
        let x = tl.x + col as f64 * (br.x - tl.x) / width as f64;
        let y = tl.y - row as f64 * (tl.y - br.y) / height as f64;
        let z = self.top_left_point.z;

        Vec3::new(x, y, z)
    }
}

pub struct RayTracer {
    scene_objects: Vec<Box<dyn SceneObject>>,
    camera: Camera,
    light: Light,
    screen: Screen,
}

impl RayTracer {
    pub fn new(
        scene_objects: Vec<Box<dyn SceneObject>>,
        camera: Camera,
        screen: Screen,
        light: Light,
    ) -> Self {
        Self {
            scene_objects,
            camera,
            screen,
            light,
        }
    }

    pub fn get_pixels(&self) -> &Vec<Color> {
        &self.screen.pixels
    }

    fn blinn_phong(&self, object: &dyn SceneObject, intersection_position: Vec3) -> Color {
        let intersection_to_light = Vec3::unit(self.light.position - intersection_position);
        let intersection_to_cam = Vec3::unit(self.camera.position - intersection_position);
        let normal = object.get_normal_at(intersection_position);
        let shininess = 300.0;
        let white = Color::new(1.0, 1.0, 1.0);

        // Decrease diffuse color to get ambient color
        let ambient = object.get_color() * 0.1 * white;

        let diffuse = object.get_color() * white * Vec3::dot(intersection_to_light, normal);

        let specular = object.get_color()
            * 1.5 // Increase diffuse color to get specular color
            * white
            * Vec3::dot(
                normal,
                Vec3::unit(intersection_to_light + intersection_to_cam),
            )
            .powf(shininess / 4.);

        ambient + diffuse + specular
    }

    fn generate_pixel(&self, row: u32, col: u32) -> Color {
        let pixel_position = self.screen.get_pixel_position(row, col);
        let cam_to_pixel = pixel_position - self.camera.position;
        let primary_ray = Ray::new(self.camera.position, cam_to_pixel);

        let (nearest_object, dist) = primary_ray.get_nearest(&self.scene_objects);

        if let Some(object) = nearest_object {
            let intersection_position = primary_ray.position_at(dist);
            let normal = object.get_normal_at(intersection_position);
            let shifted_intersection_position = intersection_position + normal * 1e-05;
            let shadow_ray = Ray::new(
                shifted_intersection_position,
                self.light.position - shifted_intersection_position,
            );
            let (_, dist) = shadow_ray.get_nearest(&self.scene_objects);
            let is_shadowed = dist < f64::INFINITY;

            if is_shadowed {
                // Color::new(0.0, 0.0, 0.0)
                object.get_color() * 0.1
            } else {
                self.blinn_phong(object, intersection_position)
            }
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }

    pub fn render(&mut self) {
        let (width, height) = self.screen.resolution;

        for row in 0..height {
            for col in 0..width {
                self.screen.pixels.push(self.generate_pixel(row, col));
            }
        }
    }
}
