pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector {
    pub fn new4(x: f64, y: f64, z: f64, w: f64) -> Vector {
        Vector {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn new3(x: f64, y: f64, z: f64) -> Vector {
        Vector {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }

    pub fn new2(x: f64, y: f64) -> Vector {
        Vector {
            x: x,
            y: y,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn new1(x: f64) -> Vector {
        Vector {
            x: x,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn new() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn add(&mut self, v: &Vector) -> &mut Vector {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self.w += v.w;
        self
    }

    pub fn new_add(&mut self, v: &Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
            w: self.w + v.w,
        }
    }

    pub fn sub(&mut self, v: &Vector) -> &mut Vector {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
        self.w -= v.w;
        self
    }

    pub fn new_sub(&mut self, v: &Vector) -> Vector {
        Vector {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
            w: self.w - v.w,
        }
    }

    pub fn mult(&mut self, v: &Vector) -> &mut Vector {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
        self.w *= v.w;
        self
    }

    pub fn new_mult(&mut self, v: &Vector) -> Vector {
        Vector {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
            w: self.w * v.w,
        }
    }

    pub fn div(&mut self, v: &Vector) -> &mut Vector {
        self.x /= v.x;
        self.y /= v.y;
        self.z /= v.z;
        self.w /= v.w;
        self
    }

    pub fn new_div(&mut self, v: &Vector) -> Vector {
        Vector {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
            w: self.w / v.w,
        }
    }

    pub fn mult_zero(&mut self) -> &mut Vector {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
        self.w = 0.0;
        self
    }

    pub fn mult_val(&mut self, val: f64) -> &mut Vector {
        self.x *= val;
        self.y *= val;
        self.z *= val;
        self.w *= val;
        self
    }

    pub fn div_val(&mut self, val: f64) -> &mut Vector {
        self.x /= val;
        self.y /= val;
        self.z /= val;
        self.w /= val;
        self
    }

    pub fn copy(&mut self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w,
        }
    }

    pub fn mag(&mut self) -> f64 {
        ((self.x*self.x) + (self.y*self.y) + (self.z*self.z) + (self.w*self.w)).sqrt()
    }

    pub fn norm(&mut self) -> &mut Vector {
        self.x /= self.mag();
        self.y /= self.mag();
        self.z /= self.mag();
        self.w /= self.mag();

        self
    }
}