#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
pub enum AnimationId {
    BulletImpact,
    Die,
    Explode,
    Jump,
    Move,
    Idle,
    Shoot,
    Walk,
    Flying,
}
