// The held map's atlas access: `map_rect` looks up a warmed map's rect
// (the entity pass cannot upload); `map_quads` draws the map picture and its
// vanilla decorations at an already-computed pose. `paper_quads` draws just
// the paper for a map without data.
use crate::game::Game;
use crate::game::maps::{shown_on_frame, decoration_pose};
use crate::hud::Rect;
use crate::mesher::Vertex;
use glam::Mat4;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    /// The atlas rect `map_texture` gave each map id this frame. The warm pass
    /// runs at the start of `entity_geometry`, so the entity pass can read it
    /// without uploading.
    static MAP_RECTS: RefCell<HashMap<i32, Rect>> = RefCell::new(HashMap::new());
}

/// Remember the rect `map_texture` returned for `id`.
pub(crate) fn remember_rect(id: i32, rect: Rect) {
    MAP_RECTS.with(|m| m.borrow_mut().insert(id, rect));
}

/// Drop last frame's rects: the warm pass refills them each frame.
pub(crate) fn clear_frame() {
    MAP_RECTS.with(|m| m.borrow_mut().clear());
}

/// One quad of a map face, at `z` under the pose `m`.
fn push_quad(
    m: Mat4,
    corners: [[f32; 3]; 4],
    uv: [[f32; 2]; 4],
    light: u32,
    out: &mut Vec<Vertex>,
) {
    for (c, uv) in corners.into_iter().zip(uv) {
        out.push(Vertex {
            normal: [0; 4],
            pos: m.transform_point3(glam::Vec3::from_array(c)).to_array(),
            uv,
            layer: 0,
            color: 0xFFFF_FFFF,
            light,
            glint_uv: [0.0, 0.0],
        });
    }
}

/// The reverse-wound back face: mirrors U so the paper reads correctly from
/// behind.
fn push_back_quad(
    m: Mat4,
    corners: [[f32; 3]; 4],
    uv: [[f32; 2]; 4],
    u0: f32,
    u1: f32,
    light: u32,
    out: &mut Vec<Vertex>,
) {
    for i in [3, 2, 1, 0] {
        let c = corners[i];
        let [u, v] = uv[i];
        out.push(Vertex {
            normal: [0; 4],
            pos: m.transform_point3(glam::Vec3::from_array(c)).to_array(),
            uv: [u0 + u1 - u, v],
            layer: 0,
            color: 0xFFFF_FFFF,
            light,
            glint_uv: [0.0, 0.0],
        });
    }
}

/// The atlas rect of map `id` once the warm pass uploaded it: the entity pass
/// draws on `&self` and cannot upload.
pub(crate) fn map_rect(game: &Game<'_>, id: i32) -> Option<Rect> {
    if !game.maps.data.contains_key(&id) {
        return None;
    }
    MAP_RECTS.with(|m| m.borrow().get(&id).copied())
}

/// A static third-person hand transform: the body base plus the hand point,
/// with no arm-pose interpolation, so a newly selected item takes its own pose
/// right away instead of briefly carrying the previous item's (the game's
/// `held_item_transform` lerps the arm pose across the swap).
pub(crate) fn static_hand(
    mob: &horizon_client_lib::mobs::Mob,
    alpha: f32,
    camera: glam::DVec3,
    left: bool,
) -> glam::Mat4 {
    let base = crate::mobs::render::base_transform(mob, alpha, camera);
    let side = if left { -1.0 } else { 1.0 };
    base * glam::Mat4::from_translation(glam::Vec3::new(
        side * 0.3125,
        -0.3125,
        -0.625,
    ))
}

/// The two paper faces, front at `-0.005` and back at `+0.005`, for an
/// already-computed pose.
fn paper_quads(game: &Game<'_>, m: Mat4, light: u32, out: &mut Vec<Vertex>) {
    let Some(r) = game.gui.atlas.sprites.get("map/map_background").copied() else {
        return;
    };
    let corners = [
        [-7.0, 135.0, -0.005],
        [135.0, 135.0, -0.005],
        [135.0, -7.0, -0.005],
        [-7.0, -7.0, -0.005],
    ];
    let uv = [
        [r.u0, r.v1],
        [r.u1, r.v1],
        [r.u1, r.v0],
        [r.u0, r.v0],
    ];
    push_quad(m, corners, uv, light, out);
    push_back_quad(m, corners.map(|c| [c[0], c[1], 0.005]), uv, r.u0, r.u1, light, out);
}

/// [`Game::map_vertices`]' texture and decorations for a map already in
/// the atlas, drawn where only `&self` is available.
pub(crate) fn map_quads(
    game: &Game<'_>,
    id: i32,
    rect: Rect,
    show_only_frame: bool,
    m: Mat4,
    light: u32,
    out: &mut Vec<Vertex>,
) {
    // A map with drawn data shows its picture over the checkerboard; a map
    // without any is just paper.
    let has_data = game.maps.data.contains_key(&id);
    if has_data {
        if let Some(r) = game
            .gui
            .atlas
            .sprites
            .get("map/map_background_checkerboard")
            .copied()
        {
            let corners = [
                [-7.0, 135.0, -0.005],
                [135.0, 135.0, -0.005],
                [135.0, -7.0, -0.005],
                [-7.0, -7.0, -0.005],
            ];
            let uv = [
                [r.u0, r.v1],
                [r.u1, r.v1],
                [r.u1, r.v0],
                [r.u0, r.v0],
            ];
            push_quad(m, corners, uv, light, out);
            push_back_quad(
                m,
                corners.map(|c| [c[0], c[1], 0.005]),
                uv,
                r.u0,
                r.u1,
                light,
                out,
            );
        }
        let corners = [
            [0.0, 128.0, -0.04],
            [128.0, 128.0, -0.04],
            [128.0, 0.0, -0.04],
            [0.0, 0.0, -0.04],
        ];
        let uv = [
            [rect.u0, rect.v1],
            [rect.u1, rect.v1],
            [rect.u1, rect.v0],
            [rect.u0, rect.v0],
        ];
        push_quad(m, corners, uv, light, out);
    } else {
        paper_quads(game, m, light, out);
    }
    let decorations = game
        .maps
        .data
        .get(&id)
        .map_or(&[][..], |m| &m.decorations);
    let mut count = 0;
    for d in decorations {
        if show_only_frame && !shown_on_frame(d.sprite) {
            continue;
        }
        let Some(s) = game.gui.atlas.sprites.get(d.sprite).copied() else {
            continue;
        };
        let dm = decoration_pose(d, -0.06 - count as f32 * 0.001);
        let p = |x: f32, y: f32| dm.transform_point3(glam::Vec3::new(x, y, 0.0)).to_array();
        let mut q = |corners: [[f32; 3]; 4], uv: [[f32; 2]; 4]| {
            for (c, uv) in corners.into_iter().zip(uv) {
                out.push(Vertex {
                    normal: [0; 4],
                    pos: m.transform_point3(glam::Vec3::from_array(c)).to_array(),
                    uv,
                    layer: 0,
                    color: 0xFFFF_FFFF,
                    light,
                    glint_uv: [0.0, 0.0],
                });
            }
        };
        q(
            [p(-1.0, 1.0), p(1.0, 1.0), p(1.0, -1.0), p(-1.0, -1.0)],
            [[s.u0, s.v0], [s.u1, s.v0], [s.u1, s.v1], [s.u0, s.v1]],
        );
        count += 1;
    }
    // The sheet is blank paper from behind: the picture is painted on its face
    // only. Drawn last so it covers the picture where the pass does not depth
    // sort these quads.
    if has_data
        && let Some(r) = game.gui.atlas.sprites.get("map/map_background").copied()
    {
        let corners = [
            [-7.0, 135.0, 0.03],
            [135.0, 135.0, 0.03],
            [135.0, -7.0, 0.03],
            [-7.0, -7.0, 0.03],
        ];
        let uv = [
            [r.u0, r.v1],
            [r.u1, r.v1],
            [r.u1, r.v0],
            [r.u0, r.v0],
        ];
        push_back_quad(m, corners, uv, r.u0, r.u1, light, out);
    }
}
