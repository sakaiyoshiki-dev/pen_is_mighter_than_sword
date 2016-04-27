// Proverb-Programming: "The pen is mighter than the sword"

// 
trait Weapon {
  fn get_killing_power(&self) -> u32;
  fn get_influence(&self) -> u32;
  // Weapon can fight with another weapon.
  fn can_beaten<T: Weapon>(&self, another:& T) -> bool{
    self.get_killing_power() > another.get_killing_power()
  }
  // Weapon can compare its influence power with another weapon.
  fn mighter_than<T: Weapon>(&self, another:& T) -> bool{
    self.get_influence() > another.get_influence()
  }
}

// Pen class
pub struct Pen {
}

impl Pen {
  fn new() -> Pen {
      Pen{}
  }
  fn write(&self, something: &str){
    println!("{}",something); 
  }
}

impl Weapon for Pen {
  fn get_killing_power(&self) -> u32{
    0
  }
  fn get_influence(&self) -> u32{
    1000
  }
}

pub struct Sword {
}
impl Sword {
  fn new() -> Sword {
      Sword{}
  }
  fn slash(&self){
    println!("Zip!");
  }
}

impl Weapon for Sword {
  fn get_killing_power(&self) -> u32{
    100
  }
  fn get_influence(&self) -> u32{
    50
  }
}

fn main() {
  let the_pen: Pen = Pen::new();
  let the_sword: Sword = Sword::new();

  the_pen.write("This is a pen.");
  the_sword.slash();

  if the_sword.can_beaten(&the_pen) {
    println!("The sword can beaten the pen.");
  }
  if the_pen.mighter_than(&the_sword) {
    println!("The pen is mighter than the sword.");
  }
}
