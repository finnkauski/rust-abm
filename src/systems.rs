use crate::data::*;
use rand::prelude::*;
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct Output;

impl<'a> System<'a> for Output {
    type SystemData = (ReadStorage<'a, Demographics>, ReadStorage<'a, Money>);

    fn run(&mut self, data: Self::SystemData) {
        let (demo, money) = data;
        for (d, m) in (&demo, &money).join() {
            println!(
                "Person {} - {:?} - {:?}: Income: {} Savings {}",
                d.person_id, d.location, d.grade, m.income, m.savings
            )
        }
    }
}

pub struct MoneySystem;

impl<'a> System<'a> for MoneySystem {
    type SystemData = (ReadStorage<'a, Demographics>, WriteStorage<'a, Money>);

    fn run(&mut self, data: Self::SystemData) {
        let (demo, mut money) = data;

        for (_, mut entity) in (&demo, &mut money).join() {
            entity.savings += entity.income - entity.expenses;
            entity.savings_growth = entity.savings - entity.initial_savings
        }
    }
}

pub struct PromotionSystem;
impl<'a> System<'a> for PromotionSystem {
    type SystemData = (WriteStorage<'a, Demographics>, WriteStorage<'a, Money>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut demo, mut money) = data;
        // TODO: having this here is inefficient
        let mut rng = rand::thread_rng();

        for (d, m) in (&mut demo, &mut money).join() {
            let pass = rng.gen::<f32>() > 0.80;
            let productive = d.productivity >= 0.50;
            if productive & pass {
                d.grade = d.grade.promote();
                m.income = Income::from((&d.location, &d.grade)).0
            }
        }
    }
}
