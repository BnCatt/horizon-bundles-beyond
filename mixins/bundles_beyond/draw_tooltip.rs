// Cancellable HEAD of `draw_stack_tooltip_with`, standing in for its body
// (OVERWRITE refuses a body with closures): the game's body, except that the
// box is at least as wide as the bundle grid (vanilla: 96), which grows with
// the bundle. Always breaks. Pinned to the revision in horizon-mixin.toml.
fn bundles_beyond_draw_stack_tooltip_with(
    game: &mut Game<'_>,
    batch: &mut HudBatch,
    stack: &ItemStack,
    creative: bool,
    x: f32,
    y: f32,
    after_name: &Vec<(horizon_client::text::Text, u32)>,
) -> std::ops::ControlFlow<()> {
    use std::ops::ControlFlow::Break;
    let mut lines = game.stack_tooltip_lines(stack, false, creative);
    if lines.is_empty() {
        return Break(());
    }
    lines.splice(1..1, after_name.iter().cloned());
    let lines = Game::resolved(&lines);
    let probe = horizon_client::bundle::tooltip_layout(game.items(), stack, 0.0, 0.0, 96.0);
    let bundle_extra = probe.as_ref().map_or(0.0, |l| l.height + 2.0);
    let mut component = None;
    game.native_tooltip_component(stack, &mut component);
    let component = tooltip_component_size(component);
    let extra = bundle_extra + component.map_or(0.0, |size| size[1] + 2.0);
    let min_width = probe
        .as_ref()
        .map_or(0.0_f32, |l| l.width)
        .max(component.map_or(0.0, |size| size[0]));
    let atlas = &game.gui.atlas;
    let (x, y, w, h) = Game::tooltip_geometry(
        atlas,
        &lines,
        (x, y),
        (batch.width, batch.height),
        [min_width, extra],
    );
    game.tooltip_box(batch, Some(stack), x, y, w, h);
    let text_h = h - extra;
    let atlas = &game.gui.atlas;
    Game::tooltip_text(batch, atlas, &lines, x, y);
    if let Some(layout) =
        horizon_client::bundle::tooltip_layout(game.items(), stack, x, y + text_h + 2.0, w)
    {
        game.draw_bundle_tooltip(batch, &layout);
        game.draw_bundle_selected_name(batch, &layout, x, y + text_h + 2.0, w);
    }
    if let Some(size) = component {
        game.native_tooltip_component_draw(
            stack,
            batch,
            [x, y + text_h + bundle_extra + 2.0],
            size,
        );
    }
    Break(())
}
