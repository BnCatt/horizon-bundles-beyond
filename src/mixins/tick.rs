// HEAD of `tick_bundle_hover`: note which slot holds the hovered bundle (the
// key mixin reads it) and apply the key moves queued since the last tick.
fn bundles_beyond_tick(game: &mut Game<'_>) {
    let hovered = if takes_bundle_wheel(game.screen) {
        game.inventory_target().0
    } else {
        None
    };
    let bundle = hovered.filter(|&slot| {
        if !game.bundle_slot(slot) {
            return false;
        }
        let stack = game.slot_mut(slot).clone();
        stack.is_some_and(|s| Contents::of(game.items(), &s).is_some_and(|c| !c.items().is_empty()))
    });
    mod_bundles_beyond::set_hovered(bundle);
    for (slot, nav) in mod_bundles_beyond::take() {
        if Some(slot) != bundle {
            continue;
        }
        let Some(stack) = game.slot_mut(slot).clone() else {
            continue;
        };
        let Some((size, selected)) =
            Contents::of(game.items(), &stack).map(|c| (c.items().len(), c.selected()))
        else {
            continue;
        };
        let next = mod_bundles_beyond::navigate(size, selected, nav);
        if next >= 0 {
            game.select_bundle(slot, next);
        }
    }
}
