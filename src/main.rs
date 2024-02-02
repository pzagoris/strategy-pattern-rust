// Define the SoundStrategy trait
trait SoundStrategy {
    fn make_sound(&self);
}

// Define concrete implementations of the SoundStrategy trait
struct Cat;
struct Dog;
struct SmallDog;

impl SoundStrategy for Cat {
    fn make_sound(&self) {
        println!("Meow");
    }
}

impl SoundStrategy for Dog {
    fn make_sound(&self) {
        println!("Woof");
    }
}

impl SoundStrategy for SmallDog {
    fn make_sound(&self) {
        println!("Woof");
    }
}

// Define the Animal struct
struct Animal {
    sound_strategy: Box<dyn SoundStrategy>, // Using a trait object
}

impl Animal {
    // Constructor for Animal
    fn new(strategy: Box<dyn SoundStrategy>) -> Self {
        Self { sound_strategy: strategy }
    }

    // Method to make sound
    fn make_sound(&self) {
        self.sound_strategy.make_sound();
    }
}

fn main() {
    // Create a cat with CatSound strategy
    let cat = Animal::new(Box::new(Cat));
    cat.make_sound(); // Meow

    // Create a dog with DogSound strategy
    let dog = Animal::new(Box::new(Dog));
    dog.make_sound(); // Woof

    let dog = Animal::new(Box::new(SmallDog));
    dog.make_sound(); // Woof

}
