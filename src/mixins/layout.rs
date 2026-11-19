// Around `item_tooltip::bundle_tooltip` in `draw_tooltip_image`: where the game
// would draw a bundle's tooltip (it keeps `minecraft:tooltip_display`'s say),
// a bundle with items gets the grid that shows them all.
fn bundles_beyond_layout(
    items: &horizon_client::items::Items,
    stack: &ItemStack,
    x: f32,
    y: f32,
    box_width: f32,
    lines: usize,
    operation: impl FnOnce(&horizon_client::items::Items, &ItemStack, f32, f32, f32, usize) -> Option<horizon_client::bundle::TooltipLayout>,
) -> Option<horizon_client::bundle::TooltipLayout> {
    let vanilla = operation(items, stack, x, y, box_width, lines)?;
    Some(
        crate::game::bundles::bundles_beyond::layout(items, stack, x, y, box_width)
            .unwrap_or(vanilla),
    )
}
