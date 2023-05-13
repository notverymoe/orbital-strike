// // Copyright 2023 Natalie Baker // AGPLv3 //

use std::ops::{Index, IndexMut};

use bevy::prelude::UVec2;

pub struct Array2D<T> {
    width: usize,
    data:  Box<[T]>,
}

impl<T: Copy> Array2D<T> {
    pub fn new(width: usize, height: usize, value: T) -> Self {
        let size = width*height;
        let mut v = Vec::with_capacity(size);
        v.resize(size, value);
        Self { 
            width, 
            data: v.into_boxed_slice()
        }
    }
}

impl<T> Array2D<T> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.len()/self.width)
    }
}

impl<T> Array2D<T> {

    pub fn set(&mut self, at: impl IntoU2, value: T) -> T {
        std::mem::replace::<T>(&mut self.data[at.into_index(self.width)], value)
    }

    pub fn get(&self, at: impl IntoU2) -> &T {
        &self.data[at.into_index(self.width)]
    }

    pub fn get_mut(&mut self, at: impl IntoU2) -> &mut T {
        &mut self.data[at.into_index(self.width)]
    }

}

impl<T> Array2D<T> {

    pub fn try_set(&mut self, at: impl IntoU2, value: T) -> Option<T> {
        if self.is_inside(at) {
            self.data[at.into_index(self.width)] = value;
            None
        } else {
            Some(value)
        }
    }

    pub fn try_get(&self, at: impl IntoU2) -> Option<&T> {
        if self.is_inside(at) {
            Some(&self.data[at.into_index(self.width)])
        } else {
            None
        }
    }

    pub fn try_get_mut(&mut self, at: impl IntoU2) -> Option<&mut T> {
        if self.is_inside(at) {
            Some(&mut self.data[at.into_index(self.width)])
        } else {
            None
        }
    }

}

impl<T> Array2D<T> {

    pub fn is_inside(&self, at: impl IntoU2) -> bool {
        let (x, y) = at.into_u2();
        (x < self.width) && (y*self.width < self.data.len())
    }

}

impl<T, V: IntoU2> Index<V> for Array2D<T> {
    type Output = T;

    fn index(&self, index: V) -> &Self::Output {
        self.get(index)
    }
}

impl<T, V: IntoU2> IndexMut<V> for Array2D<T> {
    fn index_mut(&mut self, index: V) -> &mut Self::Output {
        self.get_mut(index)
    }
}


pub trait IntoU2 : Copy {
    fn into_u2(&self) -> (usize, usize);

    fn into_index(&self, width: usize) -> usize {
        let (x, y) = self.into_u2();
        x + y*width
    }
}

impl IntoU2 for (usize, usize) {
    fn into_u2(&self) -> (usize, usize) {
        *self
    }
}

impl IntoU2 for [usize; 2] {
    fn into_u2(&self) -> (usize, usize) {
        (self[0], self[1])
    }
}

impl IntoU2 for UVec2 {
    fn into_u2(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}