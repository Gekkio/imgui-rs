use super::{ImVec2, ImVec4, Ui};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::os::raw::c_void;
use sys;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImTexture(usize);

impl ImTexture {
    pub fn id(self) -> usize {
        self.0
    }
}

impl From<usize> for ImTexture {
    fn from(id: usize) -> Self {
        ImTexture(id)
    }
}

impl From<*mut c_void> for ImTexture {
    fn from(ptr: *mut c_void) -> Self {
        ImTexture(ptr as usize)
    }
}

/// Represent an image about to be drawn.
/// See [`Ui::image`].
///
/// Create your image using the builder pattern then [`Image::build`] it.
#[must_use]
pub struct Image<'ui> {
    texture_id: ImTexture,
    size: ImVec2,
    uv0: ImVec2,
    uv1: ImVec2,
    tint_col: ImVec4,
    border_col: ImVec4,
    _phantom: PhantomData<&'ui Ui<'ui>>,
}

impl<'ui> Image<'ui> {
    pub fn new<S>(_: &Ui<'ui>, texture_id: ImTexture, size: S) -> Self
    where
        S: Into<ImVec2>,
    {
        const DEFAULT_UV0: ImVec2 = ImVec2 { x: 0.0, y: 0.0 };
        const DEFAULT_UV1: ImVec2 = ImVec2 { x: 1.0, y: 1.0 };
        const DEFAULT_TINT_COL: ImVec4 = ImVec4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        const DEFAULT_BORDER_COL: ImVec4 = ImVec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        Image {
            texture_id,
            size: size.into(),
            uv0: DEFAULT_UV0,
            uv1: DEFAULT_UV1,
            tint_col: DEFAULT_TINT_COL,
            border_col: DEFAULT_BORDER_COL,
            _phantom: PhantomData,
        }
    }
    /// Set size (default based on texture)
    pub fn size<T: Into<ImVec2>>(mut self, size: T) -> Self {
        self.size = size.into();
        self
    }
    /// Set uv0 (default `[0.0, 0.0]`)
    pub fn uv0<T: Into<ImVec2>>(mut self, uv0: T) -> Self {
        self.uv0 = uv0.into();
        self
    }
    /// Set uv1 (default `[1.0, 1.0]`)
    pub fn uv1<T: Into<ImVec2>>(mut self, uv1: T) -> Self {
        self.uv1 = uv1.into();
        self
    }
    /// Set tint color (default: no tint color)
    pub fn tint_col<T: Into<ImVec4>>(mut self, tint_col: T) -> Self {
        self.tint_col = tint_col.into();
        self
    }
    /// Set border color (default: no border)
    pub fn border_col<T: Into<ImVec4>>(mut self, border_col: T) -> Self {
        self.border_col = border_col.into();
        self
    }
    /// Draw image where the cursor currently is
    pub fn build(self) {
        unsafe {
            sys::igImage(
                self.texture_id.0 as *mut c_void,
                self.size,
                self.uv0,
                self.uv1,
                self.tint_col,
                self.border_col,
            );
        }
    }
}

/// Generic texture mapping for use by renderers.
pub struct Textures<T> {
    textures: HashMap<usize, T>,
    next: usize,
}

impl<T> Textures<T> {
    pub fn new() -> Self {
        Textures {
            textures: HashMap::new(),
            next: 0,
        }
    }

    pub fn insert(&mut self, texture: T) -> ImTexture {
        let id = self.next;
        self.textures.insert(id, texture);
        self.next += 1;
        ImTexture(id)
    }

    pub fn replace(&mut self, id: ImTexture, texture: T) -> Option<T> {
        self.textures.insert(id.0, texture)
    }

    pub fn remove(&mut self, id: ImTexture) -> Option<T> {
        self.textures.remove(&id.0)
    }

    pub fn get(&self, id: ImTexture) -> Option<&T> {
        self.textures.get(&id.0)
    }
}
