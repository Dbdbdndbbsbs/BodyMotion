// The off hand's loaded crossbow turns aside like the main hand's, so with
// the main hand's item hidden it sits in the middle of the view, as a loaded
// crossbow in the main hand does.
fn bm_fp_crossbow_pose(
    hand: horizon_client::first_person::Hand,
    stack: Option<&ItemStack>,
    swing: f32,
    game: &crate::game::Game<'_>,
    x: &mut f32,
    z: &mut f32,
    yaw: &mut f32,
) {
    use crate::game::client_ambient::native_bm_glue as glue;
    if hand != horizon_client::first_person::Hand::Off
        || swing >= 0.001
        || !glue::settings(game).offhand_hiding
    {
        return;
    }
    let Some(stack) = stack else {
        return;
    };
    if &*stack.item != "minecraft:crossbow"
        || !stack.data().is_some_and(|d| !d.charged.is_empty())
    {
        return;
    }
    let invert: f32 = if game.settings.main_hand_left { 1.0 } else { -1.0 };
    let t_x = invert * 0.56;
    let t_z = -0.72;
    let t_aside = invert * -0.641864;
    let theta = (invert * 10.0).to_radians();
    let cos = theta.cos();
    let sin = theta.sin();
    let r_x = t_x * cos + t_z * sin;
    let r_z = -t_x * sin + t_z * cos;
    *x = t_x + t_aside - r_x;
    *z = t_z - r_z;
    *yaw = invert * 10.0;
}
