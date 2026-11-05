// Cancellable HEAD of `screen_key_pressed`: over a bundle in an inventory
// screen, the arrow keys and the movement keys move its selection
// (`BundleTooltipAdditions.handleKeybindsInBundleGui`) and go no further.
fn bundles_beyond_key(game: &Game<'_>, key: KeyCode) -> std::ops::ControlFlow<bool> {
    use horizon_client::bundle::bundles_beyond::{self as bb, Nav};
    let Some(name) = keybinds::key_name(key) else {
        return std::ops::ControlFlow::Continue(());
    };
    if !super::bundles::takes_bundle_wheel(game.screen) {
        return std::ops::ControlFlow::Continue(());
    }
    let Some(slot) = game.inventory_target().0 else {
        return std::ops::ControlFlow::Continue(());
    };
    if bb::hovered() != Some(slot) {
        return std::ops::ControlFlow::Continue(());
    }
    let binds = &game.settings.keys;
    let nav = if name == "key.keyboard.up" || name == binds.get("key.forward") {
        Nav::Up
    } else if name == "key.keyboard.down" || name == binds.get("key.back") {
        Nav::Down
    } else if name == "key.keyboard.left" || name == binds.get("key.left") {
        Nav::Left
    } else if name == "key.keyboard.right" || name == binds.get("key.right") {
        Nav::Right
    } else {
        return std::ops::ControlFlow::Continue(());
    };
    bb::push(slot, nav);
    std::ops::ControlFlow::Break(true)
}
