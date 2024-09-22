#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum Many<T> {
    NoOne,
    One(T),
    Many(Vec<T>),
}

impl<T> Default for Many<T> {
    fn default() -> Self {
        Self::NoOne
    }
}

impl<T> Many<T> {
    pub fn iter(&self) -> ManyIter<T> {
        match self {
            Many::NoOne => ManyIter::NoOne,
            Many::One(value) => ManyIter::One(Some(value)),
            Many::Many(vec) => ManyIter::Many(vec.iter()),
        }
    }

    pub fn iter_mut(&mut self) -> ManyIterMut<T> {
        match self {
            Many::NoOne => ManyIterMut::NoOne,
            Many::One(ref mut value) => ManyIterMut::One(Some(value)),
            Many::Many(ref mut vec) => ManyIterMut::Many(vec.iter_mut()),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Many::NoOne => true,
            Many::One(_) => false,
            Many::Many(vec) => vec.is_empty(),
        }

    }
}

pub enum ManyIter<'a, T> {
    NoOne,
    One(Option<&'a T>),
    Many(std::slice::Iter<'a, T>),
}

impl<'a, T> Iterator for ManyIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ManyIter::NoOne => None,
            ManyIter::One(option) => option.take(),
            ManyIter::Many(iter) => iter.next(),
        }
    }
}

pub enum ManyIterMut<'a, T> {
    NoOne,
    One(Option<&'a mut T>),
    Many(std::slice::IterMut<'a, T>),
}

impl<'a, T> Iterator for ManyIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ManyIterMut::NoOne => None,
            ManyIterMut::One(option) => option.take(),
            ManyIterMut::Many(iter) => iter.next(),
        }
    }
}