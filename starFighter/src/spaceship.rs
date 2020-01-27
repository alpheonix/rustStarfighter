pub struct Spaceship {
    pub _id: String,
    pub name: String,
    pub speed: i64,
    pub hp: i64,
    pub damage: i64,
    pub adventage: bool,
    pub miss : i64


}


pub fn create_spaceship_template() -> Spaceship {
    Spaceship{
    _id: "1".to_string(),
    name: "X-wing".to_string(),
    speed: 100,
    hp: 92000,
    damage: 24000,
    adventage: false,
    miss: 100
    }
}