trait GameObject {
    fn is_active(&self) -> bool;
    fn update(&mut self, w: &mut three::Window);
}

fn update_game_object_if_is_active<T: GameObject>(go: &mut T, w: &mut three::Window) {
    if go.is_active() {
        go.update(w);
    }
}

pub struct Bullet {
    active: bool,
    position: mint::Point3<f32>,
    mesh: three::Mesh,
    time: f32,
}

impl Bullet {
    fn new(w: &mut three::Window) -> Bullet {
        let x = 0.0;
        let y = 0.0;
        let z = 0.0;
        let mesh = w.factory.mesh(
            three::Geometry::cylinder(0.02, 0.02, 0.0, 8),
            three::material::Basic {
                color: 0x00_00_ff,
                map: None,
            },
        );
        w.scene.add(&mesh);
        let bullet = Bullet {
            active: true,
            position: [x, y, z].into(),
            mesh,
            time: 0.0,
        };
        use crate::three::Object;
        {
            let cq: cgmath::Quaternion<f32> = cgmath::Rotation3::from_angle_z(cgmath::Rad(1.57));
            println!("{:?}", cq);
            let tmp: [f32; 4] = cq.into();
            println!("{:?}", tmp);
            bullet.mesh.set_orientation(tmp);
        }

        bullet
    }
}

impl GameObject for Bullet {
    fn is_active(&self) -> bool {
        self.active
    }
    fn update(&mut self, w: &mut three::Window) {
        use crate::three::Object;
        let delta = w.input.delta_time();
        self.time += delta;
        if self.time > 1.0 {
            self.active = false;
            return;
        }
        self.position.y += delta * 5.0;
        self.mesh.set_position(self.position);
    }
}

pub struct Player {
    position: mint::Point3<f32>,
    mesh: three::Mesh,
    bullets: Vec<Bullet>,
    wait: f32,
}

impl Player {
    fn new(w: &mut three::Window) -> Player {
        let x = 0.0;
        let y = 0.0;
        let z = 0.0;
        let mesh = w.factory.mesh(
            three::Geometry::with_vertices(vec![
                [x - 0.05, y - 0.05, z].into(),
                [x + 0.05, y - 0.05, z].into(),
                [x, y + 0.05, z].into(),
            ]),
            three::material::Basic {
                color: 0x00_ff_00,
                map: None,
            },
        );
        w.scene.add(&mesh);
        Player {
            position: [x, y, z].into(),
            mesh,
            bullets: vec![],
            wait: 0.0,
        }
    }
    fn shoot(&mut self, w: &mut three::Window) {
        if self.wait > 0.0 {
            return;
        }
        if self.bullets.iter().filter(|&e| e.active).count() >= 10 {
            return;
        }
        for bullet in &mut self.bullets {
            if !bullet.is_active() {
                bullet.active = true;
                bullet.time = 0.0;
                bullet.position = self.position;
                self.wait += 0.08;
                return;
            }
        }
        let mut bullet = Bullet::new(w);
        bullet.position = self.position;
        self.bullets.push(bullet);
        self.wait += 0.08;
    }
}

impl GameObject for Player {
    fn is_active(&self) -> bool {
        true
    }
    fn update(&mut self, w: &mut three::Window) {
        if self.wait > 0.0 {
            self.wait -= w.input.delta_time();
        }
        use crate::three::Object;
        if let Some(lr) = w.input.timed(three::AXIS_LEFT_RIGHT) {
            self.position.x += lr;
        }
        if let Some(du) = w.input.timed(three::AXIS_DOWN_UP) {
            self.position.y += du;
        }
        self.mesh.set_position(self.position);

        if w.input.hit(three::Key::Z) {
            self.shoot(w);
        }
    }
}

pub fn run() {
    let mut window = three::Window::new("shootem_up");
    window.scene.background = three::Background::Color(0x00_00_00);
    let camera = window
        .factory
        .orthographic_camera([0.0, 0.0], 1.0, -1.0..1.0);

    let mut p = Player::new(&mut window);

    while window.update() {
        update_game_object_if_is_active(&mut p, &mut window);
        for e in &mut p.bullets {
            update_game_object_if_is_active(e, &mut window)
        }
        window.render(&camera);
    }
}
