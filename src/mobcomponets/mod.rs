
type Durability = u32;

pub struct Health(Durability);
pub enum Armor {
    Block(Durability),
    Reduce(f32),
}
pub struct Attack(Durability);
