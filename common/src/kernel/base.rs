use hashers::oz::DJB2Hasher;
use ordered_float::NotNan;
use std::hash::{BuildHasher, BuildHasherDefault};
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum GraphicType {
    None,
    Line,
    Arrow,
    Box,
    Circle,
    Label,
    LocalArrow, // Located entirely within the cell boundaries, coordinates in the range [0., 1.]
    LocalLine,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum GraphicStyle {
    Grid,
    Frame,    // Static "frame". Contrast between STYLE_INACTIVE and STYLE_ACTIVE
    Hidden,   // Only display when object is selected or highlighted
    Inactive, // Render using low-contrast color
    Active,   // Render using high-contast color

    // UI highlight groups
    Highlighted0,
    Highlighted1,
    Highlighted2,
    Highlighted3,
    Highlighted4,
    Highlighted5,
    Highlighted6,
    Highlighted7,

    Selected,
    Hover,
    Max,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct GraphicElement {
    graphic_type: GraphicType,
    style: GraphicStyle,
    // TODO: This should use fixed point decimal precision at the sub-nanometer-ish scale
    // if it has any bearing on the simulation and routing aspects.
    x1: NotNan<f32>,
    y1: NotNan<f32>,
    x2: NotNan<f32>,
    y2: NotNan<f32>,
    z: NotNan<f32>,
    text: String,
}
//     GraphicElement(){};
//     GraphicElement(type_t type, style_t style, float x1, float y1, float x2, float y2, float z)
//             : type(type), style(style), x1(x1), y1(y1), x2(x2), y2(y2), z(z){};
impl GraphicElement {
    pub const fn default() -> Self {
        // This unsafe block just lets us const initialize the defaults.
        // I hope 0.0 is a not NaN.
        unsafe {
            Self {
                graphic_type: GraphicType::None,
                style: GraphicStyle::Frame,
                x1: NotNan::new_unchecked(0.0),
                y1: NotNan::new_unchecked(0.0),
                x2: NotNan::new_unchecked(0.0),
                y2: NotNan::new_unchecked(0.0),
                z: NotNan::new_unchecked(0.0),
                text: String::new(),
            }
        }
    }
    pub const fn new(
        graphic_type: GraphicType,
        style: GraphicStyle,
        x1: NotNan<f32>,
        y1: NotNan<f32>,
        x2: NotNan<f32>,
        y2: NotNan<f32>,
        z: NotNan<f32>,
    ) -> Self {
        Self {
            graphic_type,
            style,
            x1,
            x2,
            y1,
            y2,
            z,
            text: String::new(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct Loc {
    x: i32,
    y: i32,
    z: i32,
}
impl Loc {
    pub const fn default() -> Self {
        Self {
            x: -1,
            y: -1,
            z: -1,
        }
    }
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    pub fn get_hash(&self) -> u64 {
        let mut hasher = BuildHasherDefault::<DJB2Hasher>::default().build_hasher();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for Loc {
    fn default() -> Self {
        Loc::default()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ArcBounds {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl ArcBounds {
    /// Default values for an [`ArcBounds`].
    pub const fn default() -> Self {
        Self {
            x0: -1,
            y0: -1,
            x1: -1,
            y1: -1,
        }
    }
    /// Creates a new [`ArcBounds`].
    pub const fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        Self { x0, y0, x1, y1 }
    }

    /// Calculates the point wise distance from the associated [`ArcBounds`] and a [`Loc`].
    pub const fn distance(&self, loc: Loc) -> i32 {
        let mut dist: i32 = 0;

        if loc.x < self.x0 {
            dist += self.x0 - loc.x;
        }
        if loc.x > self.x1 {
            dist += loc.x - self.x1;
        }
        if loc.y < self.y0 {
            dist += self.y0 - loc.y;
        }
        if loc.y > self.y1 {
            dist += loc.y - self.y1;
        }
        dist
    }

    /// Check if a 2d point is contained within the boundary defined by the [`ArcBounds`].
    pub const fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x0 && y >= self.y0 && x <= self.x1 && y <= self.y1
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum PlaceStrength {
    None = 0,
    Weak = 1,
    Strong = 2,
    Placer = 3,
    Fixed = 4,
    Locked = 5,
    User = 6,
}
