// HEAD of `native_render_tooltip_gather_components`, before any mod hears it:
// the bundle image is as big as the grid that shows every item (the game sized
// it for vanilla's: 96 wide, at most 3 rows).
fn bundles_beyond_image_size(game: &Game<'_>, stack: &ItemStack, elements: &mut Vec<TooltipElement>) {
    let Some(layout) = crate::game::bundles::bundles_beyond::layout(game.items(), stack, 0.0, 0.0, 96.0)
    else {
        return;
    };
    for element in elements.iter_mut() {
        if let TooltipElement::Image(image) = element
            && matches!(image.source, mod_api::TooltipImageSource::Bundle(_))
        {
            image.size = [layout.width.max(96.0), layout.height];
        }
    }
}
