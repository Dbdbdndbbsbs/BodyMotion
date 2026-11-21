// The off hand's loaded crossbow turns aside like the main hand's, so with
// the main hand's item hidden it sits in the middle of the view, as a loaded
// crossbow in the main hand does. The mod API's `ItemStack` is opaque, so the
// charged check runs here against the game's own stack data.
fn bm_fp_crossbow_pose(
    render: &mut mod_api::HandRender<'_>,
    game: &crate::game::Game<'_>,
) {
    use bodymotion::glue as glue;
    if render.hand != horizon_client::first_person::Hand::Off
        || render.swing >= 0.001
        || !glue::settings().offhand_hiding
    {
        return;
    }
    let Some(stack) = game.offhand.as_ref() else {
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
    render.translation[0] = t_x + t_aside - r_x;
    render.translation[2] = t_z - r_z;
    render.rotation[1] = invert * 10.0;
}
