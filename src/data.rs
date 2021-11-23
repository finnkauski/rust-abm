use serde::Deserialize;
use specs::{Component, VecStorage};

#[derive(Clone, Component, Debug, Deserialize)]
#[storage(VecStorage)]
pub enum Location {
    London,
    Newport,
}

#[derive(Clone, Component, Debug, Deserialize)]
#[storage(VecStorage)]
pub enum Grade {
    HEO,
    SEO,
    G7,
}

impl Grade {
    pub fn promote(&self) -> Self {
        match self {
            Self::HEO => Self::SEO,
            Self::SEO => Self::G7,
            Self::G7 => Self::G7,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Income(pub i32);

impl From<(&Location, &Grade)> for Income {
    fn from(info: (&Location, &Grade)) -> Self {
        let val = match info {
            (London, G7) => 3400,
            (London, SEO) => 2000,
            (London, HEO) => 1600,
            (Newport, G7) => 3200,
            (Newport, SEO) => 1800,
            (Newport, HEO) => 1400,
        };
        Income(val)
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Money {
    pub expenses: i32,
    pub income: i32,
    pub savings: i32,
    pub initial_savings: i32,
    pub savings_growth: i32,
}

impl From<&Employee> for Money {
    fn from(e: &Employee) -> Self {
        Money {
            expenses: e.monthly_expenses,
            income: e.monthly_income.0,
            savings: e.savings,
            initial_savings: e.savings,
            savings_growth: 0,
        }
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Demographics {
    pub person_id: i32,
    pub location: Location,
    pub productivity: f32,
    pub grade: Grade,
}

impl From<&Employee> for Demographics {
    fn from(e: &Employee) -> Self {
        Demographics {
            person_id: e.person_id,
            location: e.location.clone(),
            productivity: e.productivity,
            grade: e.grade.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Employee {
    pub person_id: i32,
    pub location: Location,
    pub productivity: f32,
    pub grade: Grade,
    pub monthly_expenses: i32,
    pub monthly_income: Income,
    pub savings: i32,
}
