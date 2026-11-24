// A filled map in a third-person hand is drawn as a reading board: a flat sheet
// whose near edge sits by the palm, tilted up toward the player's face the way
// a first-person map is read. Its frame comes from the body yaw alone, so the
// board never tilts with the arm swing and never drifts when the camera turns.
// A two-handed board also follows the head's pitch: look down and it comes up
// to the face, look up and it sinks away.
use glam::{Mat4, Vec3, Vec4};

/// The board's width in blocks. A map texture is 128 pixels square. The
/// two-handed board spans the body; a one-handed one is smaller, as held.
const TWO_HAND_WIDTH: f32 = 0.63;
const ONE_HAND_WIDTH: f32 = 0.42;
/// How far the one-handed board leans up from level, in degrees.
const ONE_HAND_TILT: f32 = 45.0;
/// The two-handed board rides up toward the face as the head tilts off level
/// (in either direction), at most this many blocks.
const TWO_HAND_LIFT: f32 = 0.08;
/// How far the board tips toward the face at a full up or down tilt, in
/// degrees.
const TWO_HAND_TILT_MAX: f32 = 45.0;
/// The near edge sits relative to the palm: the two-handed board is carried up
/// to the hands, the one-handed board hangs a little below. Negative sits the
/// board below the palm, clear of the chin.
const TWO_HAND_ABOVE: f32 = 0.08;
const ONE_HAND_ABOVE: f32 = 0.05;
/// The board sits at the palms; while looking down it is carried a little
/// further forward so it clears the face instead of passing through the head.
const AHEAD_OF_PALM: f32 = 0.0;
const TWO_HAND_AHEAD_DOWN: f32 = 0.06;
/// How far the board is pulled from the palm toward the body's centre line:
/// a one-handed board sits near the middle of the hand, a two-handed one
/// spans the body.
const ONE_HAND_PULL: f32 = 0.0;
const TWO_HAND_PULL: f32 = 0.34;
/// How far two maps (one in each hand) sit outboard of their palms.
const DOUBLE_MAP_SPREAD: f32 = 0.5;

/// The board pose for one hand. `hand` is the game's held-item matrix, whose
/// translation is the palm. `two_handed` centres a single board on the body,
/// so the empty off hand reaches its far side, and the board rises with the
/// head's `pitch` (positive looking down). `double_map` spreads two maps (one
/// per hand) apart, outboard of their palms. `left` is the arm; `yaw` is the
/// interpolated body yaw in degrees. `strapped` (a one-handed map swimming,
/// elytra, falling) straps the board to the hand like any held item, so the
/// arm keeps its own animation and the map stays fixed in the palm.
pub fn held_map_matrix(
    hand: Mat4,
    two_handed: bool,
    double_map: bool,
    left: bool,
    yaw: f32,
    pitch: f32,
    strapped: bool,
) -> Mat4 {
    let yaw = yaw.to_radians();
    let _strapped = strapped;
    // The body's right and forward on the ground plane. Yaw 0 faces +Z, and a
    // player's right hand is then at -X, so `right` points to -X there.
    let right = Vec3::new(-yaw.cos(), 0.0, -yaw.sin());
    let forward = Vec3::new(-yaw.sin(), 0.0, yaw.cos());
    let palm = hand.w_axis.truncate();
    if strapped {
        // Like on land, but with the hand's own axes: the board keeps its
        // usual lean and offsets, and turns with the stroke. A two-handed
        // board keeps its full width and rides the main hand, so it never
        // drifts off while the arms are busy (swimming, elytra, falling).
        let x = hand.x_axis.truncate().normalize_or_zero();
        let y = hand.y_axis.truncate().normalize_or_zero();
        let z = hand.z_axis.truncate().normalize_or_zero();
        let pull = if left { ONE_HAND_PULL } else { -ONE_HAND_PULL };
        let centre = palm + x * pull + z * ONE_HAND_ABOVE;
        let basis = Mat4::from_cols(
            x.extend(0.0),
            (-y).extend(0.0),
            (-z).extend(0.0),
            Vec4::W,
        );
        let width = if two_handed { TWO_HAND_WIDTH } else { ONE_HAND_WIDTH };
        let scale = width / 128.0;
        return Mat4::from_translation(centre)
            * basis
            * Mat4::from_rotation_x(-ONE_HAND_TILT.to_radians())
            * Mat4::from_scale(Vec3::splat(scale))
            * Mat4::from_translation(Vec3::new(-64.0, -112.0, 0.0));
    }
    let pull = if two_handed {
        if left { TWO_HAND_PULL } else { -TWO_HAND_PULL }
    } else if double_map {
        if left { -DOUBLE_MAP_SPREAD } else { DOUBLE_MAP_SPREAD }
    } else if left {
        ONE_HAND_PULL
    } else {
        -ONE_HAND_PULL
    };
    let mag = pitch.abs().min(90.0) / 90.0;
    let down = pitch.max(0.0) / 90.0;
    let (width, tilt, lift) = if two_handed {
        // Level like a pancake at level gaze; the board tips up and rises
        // toward the face the further the head tilts off level, in either
        // direction, so the player keeps reading it.
        (
            TWO_HAND_WIDTH,
            mag * TWO_HAND_TILT_MAX,
            mag * TWO_HAND_LIFT,
        )
    } else {
        (ONE_HAND_WIDTH, ONE_HAND_TILT, 0.0)
    };
    // The pivot is the near edge's middle, the part the hands hold, so leaning
    // the board up raises only its far edge. Looking down alone pushes the
    // board a little forward, clear of the face.
    let above = if two_handed { TWO_HAND_ABOVE } else { ONE_HAND_ABOVE };
    let ahead = if two_handed { down * TWO_HAND_AHEAD_DOWN } else { 0.0 };
    let near = palm + right * pull + forward * (AHEAD_OF_PALM + ahead) + Vec3::Y * (above + lift);
    // Local X runs to the body's right and local Y toward the reader, so the
    // picture reads the right way round from behind the player. Local Z points
    // down: the map quad's written face is its -Z side, so that face looks up
    // and, once leaned back, toward the player.
    let basis = Mat4::from_cols(
        right.extend(0.0),
        (-forward).extend(0.0),
        Vec3::NEG_Y.extend(0.0),
        Vec4::W,
    );
    // The map quad starts at its top-left pixel; shift it so the board
    // reaches forward from the hands, pulled back a little so it touches them.
    let scale = width / 128.0;
    Mat4::from_translation(near)
        * basis
        * Mat4::from_rotation_x(tilt.to_radians())
        * Mat4::from_scale(Vec3::splat(scale))
        * Mat4::from_translation(Vec3::new(-64.0, -112.0, 0.0))
}
