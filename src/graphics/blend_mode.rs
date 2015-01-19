/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

//! Available blending modes for drawing

pub use self::BlendMode::{BlendAlpha, BlendAdd, BlendMultiply, BlendNone};

///Available Blending modes for drawing.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Show, Copy)]
pub enum BlendMode {
    /// Pixel = Source * Source.a + Dest * (1 - Source.a)
    BlendAlpha = 0,
    /// Pixel = Source + Dest.
    BlendAdd = 1,
    /// Pixel = Source * Dest.
    BlendMultiply = 2,
    /// Pixel = Source.
    BlendNone = 3
}