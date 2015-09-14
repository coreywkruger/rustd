extern crate sdl2;
// extern crate sdl2;

use super::keymap::KeyPressMap;
use super::screen::Screen;
use super::camera::Camera;
use super::vector3d::Vector3d;
use super::vector2d::Vector2d;
use super::mesh::Mesh;
use super::geometry::Geometry;
use super::scene::Scene;

use self::sdl2::rect::Point;
use self::sdl2::sdl::Sdl;
use self::sdl2::keycode::KeyCode;
use self::sdl2::event::{Event, EventPump, WindowEventId};
use self::sdl2::video::{Window, WindowPos, RESIZABLE};
use self::sdl2::render::{RenderDriverIndex, SOFTWARE, Renderer};
use self::sdl2::pixels::Color;

use std::thread;
use std::f64::{consts};

// use gl::types::{GLfloat, GLenum, GLuint, GLint, GLchar, GLsizeiptr};
// use gl::types::{GLboolean};

pub struct Game {
    running: bool,
    paused: bool,
    screen: Screen,
    keymap: KeyPressMap,
    scene: Scene,
    camera: Camera
}


impl Game {
    pub fn new(width: i32, height: i32) -> Game {

        let screen = Screen::new(width, height);
        let mut cube = Mesh::new(Geometry::new_cube_geometry());
        cube.position.set_z(39.0);
        // cube.position.set_x(-11.0);

        let mut cube2 = Mesh::new(Geometry::new_cube_geometry());
        cube2.position.set_z(39.0);
        cube2.position.set_x(11.0);


        // let mut plane = Mesh::new(Geometry::new_plane_geometry(10, 16.0));
        // plane.rotation.set_x(consts::PI / 2.0);
        // plane.position.set_z(50.0);
        // plane.position.set_x(1.0);
        // plane.position.set_y(5.0);


        // cube.position.set_y(6.0);
        let mut scene = Scene::new();
        scene.add_object(cube);
        // scene.add_object(cube2);
        // scene.add_object(plane);

        let fov = 0.8 * (consts::PI / 4.0);
        let f_width = width as f64;
        let f_height = height as f64;
        let fl = 1.0;

        let camera = Camera::new(Vector3d::new(0.0,0.0,0.0), fov, fl, f_width / f_height);

        Game{
            running: true,
            paused: true,
            screen: screen,
            keymap: KeyPressMap::new(),
            scene: scene,
            camera: camera
        }
    }

    pub fn run(&mut self, sdl_context: Sdl) {
        let mut event_pump = sdl_context.event_pump();
        let window = match Window::new(&sdl_context, "PWONG", WindowPos::PosCentered, WindowPos::PosCentered, self.screen.width, self.screen.height, RESIZABLE) {
            Ok(window) => window,
            Err(err) => panic!("failed to create window: {}", err)
        };
        let mut renderer = match Renderer::from_window(window, RenderDriverIndex::Auto, SOFTWARE) {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create renderer: {}", err)
        };
        while self.running {
            self.capture_events(&mut event_pump);
            self.move_objects();
            self.wipe(&mut renderer);
            self.draw(&mut renderer);
            thread::sleep_ms(17);
        }
    }

    pub fn handle_resize(&mut self, window_width: i32, window_height: i32) {

        let f_width = window_width as f64;
        let f_height = window_height as f64;

        self.camera.set_aspect(f_width / f_height);
        self.screen.width = window_width;
        self.screen.height = window_height;
    }

    pub fn capture_events(&mut self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: KeyCode::Escape, .. } => self.quit(),
                Event::KeyDown{ keycode: KeyCode::Space, .. } => self.pause(),
                Event::KeyDown{ keycode: KeyCode::R, .. } => self.reset(),
                Event::KeyDown{ keycode, .. } => self.keymap.press(keycode),
                Event::KeyUp{ keycode, .. } => self.keymap.release(keycode),
                Event::Window { win_event_id: WindowEventId::Resized, data1, data2, .. } => self.handle_resize(data1, data2),
                _ => {}
            }
        }
    }

    pub fn move_objects(&mut self) {
        if ! self.paused {

            // Rotation stuff happens here...
            self.scene.objects[0].rotation.x = 
                self.scene.objects[0].rotation.x + 0.012;
            self.scene.objects[0].rotation.y = 
                self.scene.objects[0].rotation.y + 0.012;
            // self.scene.objects[0].rotation.z = 
                // self.scene.objects[0].rotation.z + 0.012;


            // self.scene.objects[1].rotation.x = 
            //     self.scene.objects[1].rotation.x + 0.01;
            // self.scene.objects[1].rotation.y = 
            //     self.scene.objects[1].rotation.y + 0.015;
        }
    }

    pub fn wipe(&mut self, renderer: &mut Renderer) {
        let mut drawer = renderer.drawer();
        drawer.set_draw_color(Color::RGB(0, 0, 0));
        drawer.clear();
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        let mut drawer = renderer.drawer();
        drawer.set_draw_color(Color::RGB(100, 255, 0));

        let obs = &mut self.scene.objects;

        for ob in obs {
            
            let faces = &mut ob.geometry.faces;
            for face in faces {
                
                let a = &ob.geometry.vertices[face.a];
                let b = &ob.geometry.vertices[face.b];
                let c = &ob.geometry.vertices[face.c];

                let mut p1 = Vector3d::new(a.x, a.y, a.z);
                let mut p2 = Vector3d::new(b.x, b.y, b.z);
                let mut p3 = Vector3d::new(c.x, c.y, c.z);

                let x_axis = Vector3d::get_x_axis(&ob.rotation);
                let y_axis = Vector3d::get_y_axis(&ob.rotation);
                let z_axis = Vector3d::get_z_axis(&ob.rotation);

                p1.rotate_on_axis(&x_axis, ob.rotation.x);
                p1.rotate_on_axis(&y_axis, ob.rotation.y);
                p1.rotate_on_axis(&z_axis, ob.rotation.z);

                p2.rotate_on_axis(&x_axis, ob.rotation.x);
                p2.rotate_on_axis(&y_axis, ob.rotation.y);
                p2.rotate_on_axis(&z_axis, ob.rotation.z);

                p3.rotate_on_axis(&x_axis, ob.rotation.x);
                p3.rotate_on_axis(&y_axis, ob.rotation.y);
                p3.rotate_on_axis(&z_axis, ob.rotation.z);

                p1.add(&ob.position);
                p2.add(&ob.position);
                p3.add(&ob.position);

                let width = self.screen.width as f64;
                let height = self.screen.height as f64;

                let pixel_1 = &mut self.camera.project_to_2d(p1, width as f64, height as f64);
                let pixel_2 = &mut self.camera.project_to_2d(p2, width as f64, height as f64);
                let pixel_3 = &mut self.camera.project_to_2d(p3, width as f64, height as f64);

                drawer.draw_line(
                    Point::new(pixel_1.x as i32, pixel_1.y as i32), 
                    Point::new(pixel_2.x as i32, pixel_2.y as i32));
                drawer.draw_line(
                    Point::new(pixel_1.x as i32, pixel_1.y as i32), 
                    Point::new(pixel_3.x as i32, pixel_3.y as i32));
                drawer.draw_line(
                    Point::new(pixel_3.x as i32, pixel_3.y as i32), 
                    Point::new(pixel_2.x as i32, pixel_2.y as i32));

                // Draw a triangle from the 3 vertices
                // gl::DrawArrays(gl::TRIANGLES, 0, 3);

                // Face stuff
                // let mut top = Vector2d::new(0.0,0.0);
                // let mut mid = Vector2d::new(0.0,0.0);
                // let mut bot = Vector2d::new(0.0,0.0);
                
                // if  pixel_1.y > pixel_2.y && 
                //     pixel_1.y > pixel_3.y {

                //     top.set(pixel_1.x, pixel_1.y);

                //     if  pixel_2.y > pixel_3.y {
                //         mid.set(pixel_2.x, pixel_2.y);
                //         bot.set(pixel_3.x, pixel_3.y);
                //     } else 
                //     if  pixel_3.y > pixel_2.y {
                //         mid.set(pixel_3.x, pixel_3.y);
                //         bot.set(pixel_2.x, pixel_2.y);
                //     }
                // } else 
                // if  pixel_2.y > pixel_1.y && 
                //     pixel_2.y > pixel_3.y {

                //     top.set(pixel_2.x, pixel_2.y);

                //     if  pixel_1.y > pixel_3.y {
                //         mid.set(pixel_1.x, pixel_1.y);
                //         bot.set(pixel_3.x, pixel_3.y);
                //     } else 
                //     if  pixel_3.y > pixel_1.y {
                //         mid.set(pixel_3.x, pixel_3.y);
                //         bot.set(pixel_1.x, pixel_1.y);
                //     }
                // } else 
                // if  pixel_3.y > pixel_1.y && 
                //     pixel_3.y > pixel_2.y {

                //     top.set(pixel_3.x, pixel_3.y);

                //     if  pixel_1.y > pixel_2.y {
                //         mid.set(pixel_1.x, pixel_1.y);
                //         bot.set(pixel_2.x, pixel_2.y);
                //     } else 
                //     if  pixel_2.y > pixel_1.y {
                //         mid.set(pixel_2.x, pixel_2.y);
                //         bot.set(pixel_1.x, pixel_1.y);
                //     }
                // }

                // // println!("||| {}", top.y);

                // // x * (rise / run) == y
                // // y * (run / rise) == x

                // // let s1 = Vector2d::sign(pixel_1, pixel_2, pixel_3);

                // let mut left = 0.0;
                // let mut right = 0.0;

                // if  top.x > mid.x &&
                //     top.x > bot.x {
                //     right = top.x;
                //     if mid.x < bot.x {
                //         left = mid.x;
                //     } else {
                //         left = bot.x;
                //     }
                // } else 
                // if  mid.x > top.x &&
                //     mid.x > bot.x {
                //     right = mid.x;
                //     if top.x < bot.x {
                //         left = top.x;
                //     } else {
                //         left = bot.x;
                //     }
                // } else 
                // if  bot.x > top.x &&
                //     bot.x > mid.x {
                //     right = bot.x;
                //     if top.x < mid.x {
                //         left = top.x;
                //     } else {
                //         left = mid.x;
                //     }
                // }

                

                // for y in 0..((top.y - bot.y) as i32) {
                    // for x in 0..((right - left) as i32) {

                        // let pt = Vector2d::new((left) + (x as f64), top.y + (y as f64));

                        // let s1 = Vector2d::sign(&pt, pixel_1, pixel_2);
                        // let s2 = Vector2d::sign(&pt, pixel_2, pixel_3);
                        // let s3 = Vector2d::sign(&pt, pixel_3, pixel_1);

                        // let b1 = s1 < 0.0;
                        // let b2 = s2 < 0.0;
                        // let b3 = s3 < 0.0;

                        // if (b1 == b2) && (b2 == b3) {
                        //     drawer.draw_point(Point::new(pt.x as i32, pt.y as i32))
                        // }

                        // drawer.draw_points(&points[..]);
                    // }
                // }

                // drawer.draw_line(
                //     Point::new((pixel_1.x * width) as i32, (pixel_1.y * height) as i32), 
                //     Point::new((pixel_2.x * width) as i32, (pixel_2.y * height) as i32));
            }

            // let edges = &mut ob.geometry.edges;
            // for edge in edges {
                
            //     let one = &ob.geometry.vertices[edge[0]];
            //     let two = &ob.geometry.vertices[edge[1]];

            //     let mut p1 = Vector3d::new(one.x, one.y, one.z);
            //     let mut p2 = Vector3d::new(two.x, two.y, two.z);

            //     let x_axis = Vector3d::get_x_axis(ob.rotation.x, ob.rotation.y, ob.rotation.z);
            //     let y_axis = Vector3d::get_y_axis(ob.rotation.x, ob.rotation.y, ob.rotation.z);
            //     let z_axis = Vector3d::get_y_axis(ob.rotation.x, ob.rotation.y, ob.rotation.z);

            //     p1.rotate_on_axis(x_axis.x, x_axis.y, x_axis.z, ob.rotation.x);
            //     p1.rotate_on_axis(y_axis.x, y_axis.y, y_axis.z, ob.rotation.y);
            //     p1.rotate_on_axis(z_axis.x, z_axis.y, z_axis.z, ob.rotation.y);

            //     p2.rotate_on_axis(x_axis.x, x_axis.y, x_axis.z, ob.rotation.x);
            //     p2.rotate_on_axis(y_axis.x, y_axis.y, y_axis.z, ob.rotation.y);
            //     p2.rotate_on_axis(z_axis.x, z_axis.y, z_axis.z, ob.rotation.y);

            //     p1.add(ob.position.x, ob.position.y, ob.position.z);
            //     p2.add(ob.position.x, ob.position.y, ob.position.z);

            //     let pixel_1 = self.camera.project_to_2d(p1);
            //     let pixel_2 = self.camera.project_to_2d(p2);
            //     let width = self.screen.width as f64;
            //     let height = self.screen.height as f64;

            //     drawer.draw_line(
            //         Point::new((pixel_1.x * width) as i32, (pixel_1.y * height) as i32), 
            //         Point::new((pixel_2.x * width) as i32, (pixel_2.y * height) as i32));
                
            // }
        }

        drawer.present();
    }

    fn reset_entities(&mut self) {
    
    }

    pub fn restart(&mut self) {
        self.reset_entities();
        self.paused = true;
    }

    pub fn reset(&mut self) {
        self.reset_entities();
        self.paused = true;
    }

    pub fn pause(&mut self) {
        self.paused = ! self.paused;
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}