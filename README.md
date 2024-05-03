# vulkan-rs

A "safe" Rust wrapper around the Vulkan API.

## Safety

This crate does not provide any Vulkan object lifetime safety.
It is up to the user to ensure that the lifetimes of objects are correct.
This is to provide a more flexible API and to avoid unnecessary runtime overhead.