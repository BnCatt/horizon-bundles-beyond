// Bundles Beyond: the tooltip grid, a module inside the game (beside
// game/items/bundles.rs) so it can build the game's `TooltipLayout`. The grid
// math and the key state are in the mod's crate (`mod_bundles_beyond`).
use horizon_client::bundle::{Contents, TooltipLayout};
use horizon_client::items::{ItemStack, Items};

/// A slot of the tooltip grid, as vanilla draws it.
pub const SLOT: f32 = 24.0;

/// How many stacks a bundle holds: None for a stack that is no bundle.
pub fn size(items: &Items, stack: &ItemStack) -> Option<usize> {
    Contents::of(items, stack).map(|c| c.items().len())
}

/// `ClientBundleTooltip` with every item shown. None hands the stack back to
/// vanilla: no bundle, or an empty one (its description layout stays vanilla's).
pub fn layout(
    items: &Items,
    stack: &ItemStack,
    x: f32,
    y: f32,
    box_width: f32,
) -> Option<TooltipLayout> {
    let contents = Contents::of(items, stack)?;
    let stacks = contents.items();
    let size = stacks.len();
    if size == 0 {
        return None;
    }
    let cols = mod_bundles_beyond::columns(size);
    let rows = mod_bundles_beyond::rows(size, cols);
    let width = cols as f32 * SLOT;
    let grid_height = rows as f32 * SLOT;
    let offset = ((box_width - width) / 2.0).floor();
    let x_start = x + offset + width;
    let y_start = y + grid_height;
    let mut slots = Vec::with_capacity(size);
    let mut selected = None;
    // As vanilla: rows bottom-up, columns right-to-left, slot n holds item size - 1 - n.
    let mut slot = 0;
    'grid: for row in 1..=rows {
        for column in 1..=cols {
            if slot >= size {
                break 'grid;
            }
            let at = [
                x_start - column as f32 * SLOT,
                y_start - row as f32 * SLOT,
            ];
            let index = size - 1 - slot;
            if usize::try_from(contents.selected()) == Ok(index) {
                selected = Some(slots.len());
            }
            slots.push((at, stacks[index].clone()));
            slot += 1;
        }
    }
    let bar_inner = width - 2.0;
    let fraction = contents.weight_fraction();
    Some(TooltipLayout {
        width,
        height: grid_height + 13.0 + 8.0,
        slots,
        selected,
        overflow: None,
        bar: [x + offset, y + grid_height + 4.0],
        fill: (fraction * bar_inner).trunc().clamp(0.0, bar_inner),
        full: contents.is_full(),
        label: Some(mod_bundles_beyond::weight_label(fraction)),
        empty_description: None,
    })
}
