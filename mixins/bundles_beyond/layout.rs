// Cancellable HEAD of `tooltip_layout`: a bundle with items gets the grid
// that shows them all; anything else keeps vanilla's layout.
fn bundles_beyond_layout(
    items: &Items,
    stack: &ItemStack,
    x: f32,
    y: f32,
    box_width: f32,
) -> std::ops::ControlFlow<Option<TooltipLayout>> {
    match bundles_beyond::layout(items, stack, x, y, box_width) {
        Some(layout) => std::ops::ControlFlow::Break(Some(layout)),
        None => std::ops::ControlFlow::Continue(()),
    }
}
