// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

// pub trait MapInto<U> {
//     type Output;
//     fn map_into(self) -> Self::Output;
// }
//
// pub trait MapAsRef<U> {
//     type Output;
//     fn map_as_ref(self) -> Self::Output;
// }
//
// impl<T, U> MapInto<U> for std::option::Option<T>
// where
//     T: Into<U>,
// {
//     type Output = std::option::Option<U>;
//
//     fn map_into(self) -> Self::Output {
//         self.map(Into::into)
//     }
// }
//
// impl<'a, T, U> MapAsRef<U> for std::option::Option<&'a T>
// where
//     T: AsRef<U>,
//     U: 'a,
// {
//     type Output = std::option::Option<&'a U>;
//
//     fn map_as_ref(self) -> Self::Output {
//         self.map(AsRef::as_ref)
//     }
// }
//
// impl<T, U, E> MapInto<U> for std::result::Result<T, E>
// where
//     T: Into<U>,
// {
//     type Output = std::result::Result<U, E>;
//
//     fn map_into(self) -> Self::Output {
//         self.map(Into::into)
//     }
// }

// pub trait MapIterInto<'a, U, F> {
//     type Output;
//     fn map_into(&'a mut self) -> Self::Output;
// }
//
// impl<'a, TIt, T, U, F> MapIterInto<'a, U, F> for TIt
// where
//     TIt: Iterator<Item = T> + 'a,
//     T: Into<U>,
//     F: Fn(T) -> U,
// {
//     type Output = std::iter::Map<&'a mut TIt, Into<U>::into>;
//     fn map_into(&'a mut self) -> Self::Output {
//         self.map(Into::into)
//     }
// }
