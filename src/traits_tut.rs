trait Animal{
    fn name(&self) ->  &'static str;
    fn talk(&self) {
        println!("{} cannot talk",self.name())
    }
}

struct Human{
    name: &'static str
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello",self.name())
    }
}

struct Cat{
    name: &'static str
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} Meows",self.name())
    }
}

pub fn traits()
{
    let h = Human{name: "John"};
    h.talk();

    let c = Cat{name: "Misty"};
    c.talk();

}