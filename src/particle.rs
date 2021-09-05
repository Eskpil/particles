#[derive(Debug, Clone)]
pub struct Particle {
    pub pos: Vec<f32>,
    pub size: f32,
    pub lifetime: f32,
    pub colour: [f32; 3],
    pub max_speed: f32,
    pub max_acc: f32,
    pub height: f32,
    pub width: f32,
    pub vel: Vec<f32>,
    pub acc: Vec<f32>,
}

impl Particle {
    pub fn new(pos: Vec<f32>, vel: Vec<f32>, acc: Vec<f32>, height: f32, width: f32) -> Particle {
        return Particle {
            pos: pos,
            lifetime: 1.0,
            size: 0.5,
            colour: [0.255, 0.1, 0.255],
            max_speed: 10.0,
            max_acc: 0.5,
            vel,
            height: height,
            width: width,
            acc: acc,
        };
    }

    fn check_limits(&mut self) {
        if self.vel[0] > self.max_speed {
            self.vel[0] = self.max_speed;
        }
        if self.vel[1] > self.max_speed {
            self.vel[1] = self.max_speed;
        }
        if self.acc[0] > self.max_acc {
            self.acc[0] = self.max_acc;
        }
        if self.acc[1] > self.max_acc {
            self.acc[1] = self.max_acc;
        }
    }

    fn edges(&mut self) {
        if self.pos[1] >= self.height || self.pos[1] <= 0.0 {
            self.vel[1] = self.vel[1] * -1.0;
        }
        if self.pos[0] >= self.width || self.pos[0] <= 0.0 {
            self.vel[0] = self.vel[0] * -1.0;
        }
    }

    pub fn update(&mut self, x: f32, y: f32) {
        self.pos[0] = x + self.vel[0];
        self.vel[0] = self.vel[0] + self.acc[0];
        self.pos[1] = y + self.vel[1];
        self.vel[1] = self.vel[1] + self.acc[1];
        self.check_limits();
        self.edges();
        self.lifetime = self.lifetime - 0.010;
    }

    pub fn show(&self) -> [f32; 4] {
        return [self.pos[0], self.pos[1], self.size, self.size];
    }

    pub fn finished(self) -> bool {
        return self.lifetime <= 0.0;
    }
}
