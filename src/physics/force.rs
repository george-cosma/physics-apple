use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Clone)]
pub struct Force<T>
where
    T: Clone,
{
    pub x_component: T,
    pub y_component: T,
}

impl<T: Add<Output = T> + Clone> Add for Force<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x_component: self.x_component + rhs.x_component,
            y_component: self.y_component + rhs.y_component,
        }
    }
}

impl<T: Add<Output = T> + Clone> AddAssign for Force<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self).clone() + rhs;
    }
}

impl<T: Sub<Output = T> + Clone> Sub for Force<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x_component: self.x_component - rhs.x_component,
            y_component: self.y_component - rhs.y_component,
        }
    }
}

impl<T: Default + Clone> Default for Force<T> {
    fn default() -> Self {
        Self {
            x_component: Default::default(),
            y_component: Default::default(),
        }
    }
}

impl<T: Clone + Div<Output = T>> Div<T> for Force<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x_component: self.x_component / rhs.clone(),
            y_component: self.y_component / rhs.clone(),
        }
    }
}

impl<T: Clone + Mul<Output = T>> Mul<T> for Force<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x_component: self.x_component * rhs.clone(),
            y_component: self.y_component * rhs.clone(),
        }
    }
}
