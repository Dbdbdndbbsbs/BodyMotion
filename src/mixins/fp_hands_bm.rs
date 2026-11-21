// A loaded crossbow in the off hand hides the main hand's item, as a loaded
// crossbow in the main hand hides the off hand's (the vanilla
// evaluateWhichHandsToRender, mirrored). A shield never hides, as the off hand
// hiding setting keeps it, and a crossbow never hides another crossbow.
fn bm_fp_hands(value: [bool; 2], game: &crate::game::Game<'_>) -> [bool; 2] {
    use bodymotion::glue as glue;
    if value != [true, true] || !glue::settings().offhand_hiding {
        return value;
    }
    let off = game.offhand.as_ref().is_some_and(|s| {
        &*s.item == "minecraft:crossbow" && s.data().is_some_and(|d| !d.charged.is_empty())
    });
    let main = game.stack_in(game.selected).is_some_and(|s| {
        &*s.item == "minecraft:crossbow" || &*s.item == "minecraft:shield"
    });
    if off && !main {
        [false, true]
    } else {
        value
    }
}
