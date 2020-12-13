use super::*;

#[deny(dead_code)]

pub type Sdfr = (f32,Vector3<f32>);

#[inline]
/// convert a 2d primitive to a 3d primitve
pub fn op_extrusion(p :&Vector3<f32>, h:f32, sdf2d:fn(&Vector2<f32>) -> Sdfr) -> Sdfr {
    let d = sdf2d(&p.xy());
    let over = p.z.abs()-(h/2.0);
    if over > 0.0 {
        (Vector2::new(d.0,over).norm(),d.1)
    } else {
        d
    }
}

#[inline]
pub fn op_rotation(p :&Vector3<f32>, r: f32, sdf2d:fn(&Vector2<f32>) -> Sdfr) -> Sdfr {
    let q = Vector2::new(p.xy().norm() - r, p.z);
    sdf2d(&q)
}

#[inline]
/// twists an object on the z axsis by apu radains / unit
pub fn op_twist(p :&Vector3<f32>, apu:f32, sdf:fn(&Vector3<f32>) -> Sdfr) -> Sdfr {
    sdf(&(Rotation3::new(Vector3::z() * (apu*p.z))*p))
}

#[inline]
pub fn op_rep(p :&Vector3<f32>, c: &Vector3<f32>, sdf:fn(&Vector3<f32>) -> Sdfr) -> Sdfr {
    fn r(c: f32,r: f32) -> f32 {
        ((c+r/2.0).abs())%r-(r/2.0)
    }
    sdf(&Vector3::new(r(p.x,c.x),r(p.y,c.y),r(p.z,c.z)))
}

#[inline]
pub fn op_rep_lim(p :&Vector3<f32>, d: &Vector3<f32>, a: &Vector3<f32>, sdf:fn(&Vector3<f32>) -> Sdfr) -> Sdfr {
    fn get_idx(x:f32,y:f32,z:f32) -> f32 {
        (x/y).round().clamp(0.0,z-1.0)
    }
    sdf(&(Vector3::new(
        p.x-d.x*get_idx(p.x,d.x,a.x),
        p.y-d.y*get_idx(p.y,d.y,a.y),
        p.z-d.z*get_idx(p.z,d.z,a.z)
    )))
}

#[inline]
pub fn op_disp(p: &Vector3<f32>, disp:fn(&Vector3<f32>) -> f32, sdf:fn(&Vector3<f32>) -> Sdfr) -> Sdfr {
    let s = sdf(&p);
    (s.0+disp(&p),s.1)
}

#[inline]
pub fn op_thicken(p: &Vector3<f32>, x: f32, sdf:fn(&Vector3<f32>) -> Sdfr) -> Sdfr {
    let s = sdf(&p);
    (s.0+x,s.1)
}

#[inline]
pub fn op_move(p: &Vector3<f32>, o :&Vector3<f32>, sdf:fn(&Vector3<f32>) -> Sdfr) -> Sdfr {
    sdf(&(*p+*o))
}

#[inline]
pub fn op_scale(p: &Vector3<f32>, o :f32, sdf:fn(&Vector3<f32>) -> Sdfr) -> Sdfr {
    let x = sdf(&(*p/o));
    (x.0*o,x.1)
}

#[inline]
/// coputes the boundry of an object
pub fn op_holow(p: &Vector3<f32>, sdf:fn(&Vector3<f32>) -> Sdfr) -> Sdfr {
    let x = sdf(&p);
    (x.0.abs(),x.1)
}

pub fn op_union(p :&Vector3<f32>, sdf:fn(&Vector3<f32>) -> Sdfr,sdf2:fn(&Vector3<f32>) -> Sdfr) -> Sdfr {
    let x1 = sdf(p);
    let x2 = sdf2(p);
    if x1.0 > x2.0 {
        x2
    } else {
        x1
    }
}

pub fn sdf_torus(p :&Vector3<f32>, size: &Vector2<f32>) -> Sdfr {
    let q: Vector2<f32> = Vector2::new(p.xz().norm()-size.x,p.y);
    (q.norm()-size.y,Vector3::new(1.0,1.0,1.0))
}
