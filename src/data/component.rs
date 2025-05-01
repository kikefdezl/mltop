use super::system::System;

pub trait Component {
    fn read(sys: &System) -> Self;
}
