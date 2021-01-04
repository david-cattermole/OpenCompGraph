use petgraph;

pub type GraphIdx = usize;
pub type NodeWeight = u64;
pub type EdgeWeight = u8;
pub type NodeIdx = petgraph::graph::NodeIndex<GraphIdx>;
pub type EdgeIdx = petgraph::graph::EdgeIndex<GraphIdx>;
pub type HashValue = u64;
pub type Identifier = u64;

// Color Bars Texture, for debug.
//
// https://en.wikipedia.org/wiki/SMPTE_color_bars
//
// ------------------------------
//  R  -  G  -  B   - COLOR NAME
// ------------------------------
// 235 - 235 - 235  - 100% White
// 180 - 180 - 180  - 75% White
// 235 - 235 - 16   - Yellow
// 16  - 235 - 235  - Cyan
// 16  - 235 - 16   - Green
// 235 - 16  - 235  - Magenta
// 235 - 16  - 16   - Red
// 16  - 16  - 235  - Blue
// 16  - 16  - 16   - Black
// ------------------------------
//
// The texture block (below) starts at the lower-left (zeroth index)
// and continues the upper-right (last index).
//
// Note: To make things even (only 8 entries), we skip the "75% white"
// value.
//

// static const float color_bars_f32_8x8_[] = {
//     // Row 0
//     //
//     // 235, 16, 235  - Magenta
//     0.9215f, 0.0627f, 0.9215f,

//     // 235, 16, 16   - Red
//     0.9215f, 0.0627f, 0.0627f,

//     // 16, 16, 235   - Blue
//     0.0627f, 0.0627f, 0.9215f,

//     // 16, 16, 16    - Black
//     0.0627f, 0.0627f, 0.0627f,

//     // Row 1
//     //
//     // 235, 16, 235  - Magenta
//     0.9215f, 0.0627f, 0.9215f,

//     // 235, 16, 16   - Red
//     0.9215f, 0.0627f, 0.0627f,

//     // 16, 16, 235   - Blue
//     0.0627f, 0.0627f, 0.9215f,

//     // 16, 16, 16    - Black
//     0.0627f, 0.0627f, 0.0627f,

//     // Row 2
//     //
//     // 235, 235, 235 - 100% White
//     0.9215f, 0.9215f, 0.9215f,

//     // 235, 235, 16  - Yellow
//     0.9215f, 0.9215f, 0.0627f,

//     // 16, 235, 235  - Cyan
//     0.0627f, 0.9215f, 0.9215f,

//     // 16, 235, 16   - Green
//     0.0627f, 0.9215f, 0.0627f,

//     // Row 3
//     //
//     // 235, 235, 235 - 100% White
//     0.9215f, 0.9215f, 0.9215f,

//     // 235, 235, 16  - Yellow
//     0.9215f, 0.9215f, 0.0627f,

//     // 16, 235, 235  - Cyan
//     0.0627f, 0.9215f, 0.9215f,

//     // 16, 235, 16   - Green
//     0.0627f, 0.9215f, 0.0627f
// };
// static const int color_bars_f32_8x8_count_ = 16;
