pub struct CollisionInfo {
    pub can_go: bool,
    pub from: String
}

impl CollisionInfo {
    pub fn new(can_go: bool, from: &str) -> CollisionInfo {
        CollisionInfo {
            can_go: can_go,
            from: from.to_owned()
        }
    }
}