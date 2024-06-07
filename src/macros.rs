// Copyright (c) 2024 Jacob R. Green
// All rights reserved.

macro_rules! vulkan_handle {
    ($name:tt, $ty:tt) => {
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name {
            handle: $ty,
        }

        impl $name {
            pub const fn from_raw(handle: $ty) -> Self {
                Self { handle }
            }

            pub const fn as_raw(&self) -> $ty {
                self.handle
            }
        }

        // impl From<$ty> for $name {
        //     fn from(handle: $ty) -> Self {
        //         Self::from_raw(handle)
        //     }
        // }
        //
        // impl Into<$ty> for $name {
        //     fn into(self) -> $ty {
        //         self.as_raw()
        //     }
        // }
        //
        // impl Into<$ty> for &$name {
        //     fn into(self) -> $ty {
        //         self.as_raw()
        //     }
        // }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("handle", &self.handle)
                    .finish()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

        unsafe impl Sync for $name {}
        unsafe impl Send for $name {}

        assert_eq_size!($name, $ty);
    };
}

macro_rules! vulkan_create_info_lifetime {
    ($name:tt, $ty:tt, $stype:expr) => {
        #[derive(Clone)]
        pub struct $name<'a> {
            inner: $ty,
            phantom: std::marker::PhantomData<&'a u32>,
        }

        impl $name<'_> {
            pub const fn new() -> Self {
                Self {
                    inner: $ty {
                        sType: $stype,
                        ..unsafe { std::mem::zeroed() }
                    },
                    phantom: std::marker::PhantomData,
                }
            }

            pub const fn as_raw(&self) -> &$ty {
                &self.inner
            }
        }

        // impl From<$ty> for $name<'_> {
        //     fn from(inner: $ty) -> Self {
        //         Self {
        //             inner,
        //             phantom: std::marker::PhantomData,
        //         }
        //     }
        // }
        //
        // impl AsRef<$ty> for $name<'_> {
        //     fn as_ref(&self) -> &$ty {
        //         &self.inner
        //     }
        // }

        impl std::fmt::Debug for $name<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.inner.fmt(f)
            }
        }

        unsafe impl Send for $name<'_> {}
        unsafe impl Sync for $name<'_> {}

        assert_eq_size!($name, $ty);
    };
}

macro_rules! vulkan_create_info {
    ($name:tt, $ty:tt, $stype:expr) => {
        #[derive(Clone)]
        pub struct $name {
            inner: $ty,
        }

        impl $name {
            pub const fn new() -> Self {
                Self {
                    inner: $ty {
                        sType: $stype,
                        ..unsafe { std::mem::zeroed() }
                    },
                }
            }

            pub const fn as_raw(&self) -> &$ty {
                &self.inner
            }
        }

        // impl From<$ty> for $name {
        //     fn from(inner: $ty) -> Self {
        //         Self { inner }
        //     }
        // }
        //
        // impl AsRef<$ty> for $name {
        //     fn as_ref(&self) -> &$ty {
        //         &self.inner
        //     }
        // }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.inner.fmt(f)
            }
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        assert_eq_size!($name, $ty);
    };
}

macro_rules! vulkan_struct {
    ($name:tt, $ty:tt) => {
        pub struct $name {
            inner: $ty,
        }

        impl $name {
            pub const fn new() -> Self {
                Self {
                    inner: unsafe { std::mem::zeroed() },
                }
            }

            pub const fn from_raw(inner: $ty) -> Self {
                Self { inner }
            }

            pub const fn as_raw(&self) -> &$ty {
                &self.inner
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.inner.fmt(f)
            }
        }

        assert_eq_size!($name, $ty);
    };
}

macro_rules! vulkan_struct_no_new {
    ($name:tt, $ty:tt) => {
        pub struct $name {
            inner: $ty,
        }

        impl $name {
            pub const fn from_raw(inner: $ty) -> Self {
                Self { inner }
            }

            pub const fn as_raw(&self) -> &$ty {
                &self.inner
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.inner.fmt(f)
            }
        }

        assert_eq_size!($name, $ty);
    };
}

pub(crate) use vulkan_struct_no_new;

macro_rules! vulkan_struct_custom {
    ($name:tt, $ty:tt) => {
        impl $name {
            pub const fn from_raw(inner: $ty) -> Self {
                unsafe { std::mem::transmute(inner) }
            }

            pub const fn as_raw(&self) -> &$ty {
                unsafe { std::mem::transmute(self) }
            }
        }
        assert_eq_size!($name, $ty);
    };
}

macro_rules! vulkan_struct_lifetime {
    ($name:tt, $ty:tt) => {
        pub struct $name<'a> {
            inner: $ty,
            phantom: std::marker::PhantomData<&'a u32>,
        }

        impl $name<'_> {
            pub const fn new() -> Self {
                Self {
                    inner: $ty {
                        ..unsafe { std::mem::zeroed() }
                    },
                    phantom: std::marker::PhantomData,
                }
            }

            pub const fn from_raw(inner: $ty) -> Self {
                Self {
                    inner,
                    phantom: std::marker::PhantomData,
                }
            }

            pub const fn as_raw(&self) -> &$ty {
                &self.inner
            }
        }

        //  impl From<$ty> for $name {
        //      fn from(inner: $ty) -> Self {
        //          Self { inner }
        //      }
        //  }
        //
        //  impl AsRef<$ty> for $name {
        //      fn as_ref(&self) -> &$ty {
        //          &self.inner
        //      }
        //  }

        impl std::fmt::Debug for $name<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.inner.fmt(f)
            }
        }

        assert_eq_size!($name, $ty);
    };
}

pub(crate) use vulkan_create_info;
pub(crate) use vulkan_create_info_lifetime;
pub(crate) use vulkan_handle;
pub(crate) use vulkan_struct;
pub(crate) use vulkan_struct_custom;
pub(crate) use vulkan_struct_lifetime;
