// Cancellable HEAD of `screen_key_pressed`: over a bundle in an inventory
// screen, the arrow keys and the movement keys move its selection
// (`BundleTooltipAdditions.handleKeybindsInBundleGui`) and go no further.
fn bundles_beyond_key(game: &Game<'_>, key: KeyCode) -> ::core::ops::ControlFlow<bool> {
    use ::core::ops::ControlFlow::{Break, Continue};
    let Some(name) = keybinds::key_name(key) else {
        return Continue(());
    };
    if !crate::game::bundles::takes_bundle_wheel(game.screen) {
        return Continue(());
    }
    let Some(slot) = game.inventory_target().0 else {
        return Continue(());
    };
    if mod_bundles_beyond::hovered() != Some(slot) {
        return Continue(());
    }
    let binds = &game.settings.keys;
    let Some(nav) = mod_bundles_beyond::nav_for(
        &name,
        &binds.get("key.forward"),
        &binds.get("key.back"),
        &binds.get("key.left"),
        &binds.get("key.right"),
    ) else {
        return Continue(());
    };
    mod_bundles_beyond::push(slot, nav);
    Break(true)
}
