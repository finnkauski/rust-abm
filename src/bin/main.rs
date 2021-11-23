use abby::{*, data::*};
use specs::{Builder, DispatcherBuilder, World, WorldExt};

fn main() {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::MoneySystem, "income_system", &[])
        .with(systems::PromotionSystem, "promotion_system", &["income_system"])
        .with_thread_local(systems::Output)
        .build();

    dispatcher.setup(&mut world);

    let employees = abby::io::load_people(std::path::Path::new("./data/people.csv")).unwrap();
    for e in employees {
        world
            .create_entity()
            .with(Money::from(&e))
            .with(Demographics::from(&e))
            .build();
    }

    for i in 0..10 {
        println!("Epoch {}", i);
        dispatcher.dispatch(&mut world);
        world.maintain();
    }
}
