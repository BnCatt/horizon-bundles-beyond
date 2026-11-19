// Cancellable HEAD of `bundle_scroll`, standing in for its body: the game's
// body (Nightly 0b14f8a) except that the wheel walks every item the bundle
// holds, as the grid shows them all (vanilla: at most 11 or 12). Always breaks.
fn bundles_beyond_scroll(game: &mut Game<'_>, x: f64, y: f64) -> ::core::ops::ControlFlow<bool> {
    use ::core::ops::ControlFlow::Break;
    let Some(slot) = game.inventory_target().0 else {
        return Break(false);
    };
    if !game.bundle_slot(slot) {
        return Break(false);
    }
    let stack = game.slot_mut(slot).clone();
    let items = game.items();
    let Some(bundle) = stack.filter(|s| items.get(&s.item).is_some_and(|i| i.bundle)) else {
        return Break(false);
    };
    // The one change: every stack, not `Contents::shown_count`.
    let shown = Contents::of(items, &bundle).map_or(0, |c| c.items().len());
    if shown == 0 {
        return Break(false);
    }
    let (wheel_x, wheel_y) = game.bundle_mouse.wheel.on_mouse_scroll(x, y);
    let wheel = if wheel_y == 0 { -wheel_x } else { wheel_y };
    if wheel != 0 {
        let selected = bundle.data().map_or(-1, |d| d.bundle_selected.0);
        let next = horizon_client::mouse::ScrollWheelHandler::next_selection(
            f64::from(wheel),
            selected,
            shown as i32,
        );
        if next != selected {
            game.select_bundle(slot, next);
        }
    }
    Break(true)
}
