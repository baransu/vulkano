use std::mem;
use std::sync::Arc;

use VulkanObject;
use buffer::Buffer;
use formats::Format;
use vk;

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum VertexInputRate {
    Vertex = vk::VERTEX_INPUT_RATE_VERTEX,
    Instance = vk::VERTEX_INPUT_RATE_INSTANCE,
}

/// Describes an individual `Vertex`. More precisely, a collection of attributes that can be read
/// from a vertex shader.
pub unsafe trait Vertex {
    /// Returns the characteristics of a vertex attribute.
    fn attrib(name: &str) -> Option<VertexAttribute>;
}

pub struct VertexAttribute {
    pub offset: usize,
    pub format: Format,
}

/// Trait for types that contain the layout of a collection of vertex buffers.
pub unsafe trait MultiVertex {
    fn attrib(name: &str) -> Option<(u32, VertexAttribute)>;

    /// Returns the number of buffers in this collection.
    fn num_buffers() -> u32;

    fn buffer_info(buffer_id: u32) -> (u32, VertexInputRate);

    // TODO: hacky
    fn ids(&self) -> Vec<u64>;
}

unsafe impl<T, M> MultiVertex for Arc<Buffer<T, M>> where T: Vertex {
    #[inline]
    fn attrib(name: &str) -> Option<(u32, VertexAttribute)> {
        T::attrib(name).map(|attr| (0, attr))
    }

    #[inline]
    fn num_buffers() -> u32 {
        1
    }

    #[inline]
    fn buffer_info(buffer_id: u32) -> (u32, VertexInputRate) {
        assert_eq!(buffer_id, 0);
        (mem::size_of::<T>() as u32, VertexInputRate::Vertex)
    }

    fn ids(&self) -> Vec<u64> {
        vec![self.internal_object()]
    }
}

unsafe impl<T, M> MultiVertex for Arc<Buffer<[T], M>> where T: Vertex {
    #[inline]
    fn attrib(name: &str) -> Option<(u32, VertexAttribute)> {
        T::attrib(name).map(|attr| (0, attr))
    }

    #[inline]
    fn num_buffers() -> u32 {
        1
    }

    #[inline]
    fn buffer_info(buffer_id: u32) -> (u32, VertexInputRate) {
        assert_eq!(buffer_id, 0);
        (mem::size_of::<T>() as u32, VertexInputRate::Vertex)
    }

    fn ids(&self) -> Vec<u64> {
        vec![self.internal_object()]
    }
}

macro_rules! impl_mv {
    ($t1:ident, $t2:ty) => (
        unsafe impl<$t1, M> MultiVertex for Arc<Buffer<$t2, M>> where T: Vertex {
            #[inline]
            fn attrib(name: &str) -> Option<(u32, VertexAttribute)> {
                T::attrib(name).map(|attr| (0, attr))
            }

            #[inline]
            fn num_buffers() -> u32 {
                1
            }

            #[inline]
            fn buffer_info(buffer_id: u32) -> (u32, VertexInputRate) {
                assert_eq!(buffer_id, 0);
                (mem::size_of::<T>() as u32, VertexInputRate::Vertex)
            }

            fn ids(&self) -> Vec<u64> {
                vec![self.internal_object()]
            }
        }
    );
}

impl_mv!(T, [T; 1]);
impl_mv!(T, [T; 2]);
impl_mv!(T, [T; 3]);
impl_mv!(T, [T; 4]);
impl_mv!(T, [T; 5]);
impl_mv!(T, [T; 6]);
impl_mv!(T, [T; 7]);
impl_mv!(T, [T; 8]);
impl_mv!(T, [T; 9]);
impl_mv!(T, [T; 10]);
impl_mv!(T, [T; 11]);
impl_mv!(T, [T; 12]);
impl_mv!(T, [T; 13]);
impl_mv!(T, [T; 14]);
impl_mv!(T, [T; 15]);
impl_mv!(T, [T; 16]);
impl_mv!(T, [T; 32]);
impl_mv!(T, [T; 64]);
impl_mv!(T, [T; 128]);
impl_mv!(T, [T; 256]);
impl_mv!(T, [T; 512]);
impl_mv!(T, [T; 1024]);
impl_mv!(T, [T; 2048]);
impl_mv!(T, [T; 4096]);


#[macro_export]
macro_rules! impl_vertex {
    ($out:ident $(, $member:ident)*) => (
        unsafe impl $crate::pipeline::vertex::Vertex for $out {
            #[inline(always)]
            fn attrib(name: &str) -> Option<$crate::pipeline::vertex::VertexAttribute> {
                $(
                    if name == stringify!($member) {
                        return Some($crate::pipeline::vertex::VertexAttribute {
                            offset: unsafe {
                                let dummy = 0usize as *const $out;
                                let member = (&(&*dummy).$member) as *const _;
                                member as usize
                            },

                            format: unsafe {
                                #[inline] fn f<T: $crate::pipeline::vertex::Attribute>(_: &T) -> $crate::formats::Format { T::format() }
                                let dummy = 0usize as *const $out;
                                f(&(&*dummy).$member)
                            },
                        });
                    }
                )*

                None
            }
        }
    )
}

pub unsafe trait Attribute {
    fn format() -> Format;
}

unsafe impl Attribute for f32 {
    #[inline]
    fn format() -> Format {
        Format::R32Sfloat
    }
}

unsafe impl Attribute for [f32; 1] {
    #[inline]
    fn format() -> Format {
        Format::R32Sfloat
    }
}

unsafe impl Attribute for [f32; 2] {
    #[inline]
    fn format() -> Format {
        Format::R32G32Sfloat
    }
}

unsafe impl Attribute for [f32; 3] {
    #[inline]
    fn format() -> Format {
        Format::R32G32B32Sfloat
    }
}

unsafe impl Attribute for [f32; 4] {
    #[inline]
    fn format() -> Format {
        Format::R32G32B32A32Sfloat
    }
}