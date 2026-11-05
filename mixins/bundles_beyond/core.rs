// Bundles Beyond for Horizon: the shared logic, staged as the public module
// `horizon_items::bundle::bundles_beyond` (`horizon_client::bundle::bundles_beyond`
// from the game). It is a child of `bundle`, so it reads `Contents`' private
// fields. Behaviour follows O7410's Bundles Beyond (CC BY-NC-SA 4.0) with its
// default settings: every item shown, a grid at least 4 wide and about square,
// the wheel and the arrow / movement keys select any item, "n/64" on the bar.

use super::{Contents, TooltipLayout};
use crate::items::{ItemStack, Items};
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicI64, Ordering};

/// A slot of the tooltip grid, as vanilla draws it.
pub const SLOT: f32 = 24.0;

/// `getModifiedBundleTooltipColumns`: at least 4, else the ceiling of the square root.
pub fn columns(size: usize) -> usize {
    let mut root = 0usize;
    while root * root < size {
        root += 1;
    }
    root.max(4)
}

/// `getModifiedBundleTooltipRows`.
pub fn rows(size: usize, columns: usize) -> usize {
    size.div_ceil(columns.max(1))
}

/// "n/64": the bundle's weight in 64ths, as Bundles Beyond labels the bar.
fn weight_label(contents: &Contents) -> &'static str {
    static LABELS: OnceLock<Vec<&'static str>> = OnceLock::new();
    let labels = LABELS.get_or_init(|| {
        (0..=64)
            .map(|n| &*Box::leak(format!("{n}/64").into_boxed_str()))
            .collect()
    });
    let w = contents.weight;
    let n = if w.d == 0 { 0 } else { (w.n * 64 / w.d).clamp(0, 64) } as usize;
    labels[n]
}

/// Whether a label is one of ours (drawn as is) rather than a translation key.
pub fn is_weight_label(label: &str) -> bool {
    !label.contains('.')
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
    let size = contents.items.len();
    if size == 0 {
        return None;
    }
    let cols = columns(size);
    let rows = rows(size, cols);
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
            if usize::try_from(contents.selected) == Ok(index) {
                selected = Some(slots.len());
            }
            slots.push((at, contents.items[index].clone()));
            slot += 1;
        }
    }
    let bar_inner = width - 2.0;
    Some(TooltipLayout {
        width,
        height: grid_height + 13.0 + 8.0,
        slots,
        selected,
        overflow: None,
        bar: [x + offset, y + grid_height + 4.0],
        fill: (contents.weight_fraction() * bar_inner)
            .trunc()
            .clamp(0.0, bar_inner),
        full: contents.is_full(),
        label: Some(weight_label(&contents)),
        empty_description: None,
    })
}

/// A move of the selection with the arrow or movement keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nav {
    Up,
    Down,
    Left,
    Right,
}

/// `BundleTooltipAdditions.offsetVertical` (reverse view off): the selection
/// one row up or down, wrapping, skipping the empty cells at the grid's start.
/// -1 is no selection.
pub fn offset_vertical(size: i32, width: i32, height: i32, selected: i32, offset: i32) -> i32 {
    if height == 1 {
        return if selected == -1 { 0 } else { -1 };
    }
    let selected = if selected == -1 { 0 } else { selected };
    let empty = width * height - size;
    let grid_index = selected + empty;
    let grid_x = grid_index % width;
    let mut grid_y = grid_index / width + offset;
    if grid_y < 0 {
        grid_y += height;
    }
    if grid_y >= height {
        grid_y -= height;
    }
    if grid_y == 0 && grid_x < empty {
        if height == 2 {
            return -1;
        }
        grid_y += offset;
        if grid_y < 0 {
            grid_y += height;
        }
    }
    grid_y * width + grid_x - empty
}

/// `BundleTooltipAdditions.offsetHorizontal` (reverse view off): the selection
/// one cell left or right within its row, wrapping.
pub fn offset_horizontal(size: i32, width: i32, height: i32, selected: i32, offset: i32) -> i32 {
    if size == 1 {
        return if selected == -1 { 0 } else { -1 };
    }
    let empty = width * height - size;
    let mut grid_index = selected + empty;
    let first_row = selected < width - empty;
    let row_width = if first_row { width - empty } else { width };
    if row_width == 1 {
        return if selected == 0 { -1 } else { 0 };
    }
    let mut in_row = if first_row { selected } else { grid_index % width };
    in_row += offset;
    if in_row < 0 {
        in_row = row_width - 1;
    }
    if in_row >= row_width {
        in_row = 0;
    }
    let grid_x = if first_row { in_row + empty } else { in_row };
    // Java's `/` truncates toward zero, as Rust's does.
    grid_index = grid_index / width * width + grid_x;
    grid_index - empty
}

/// The next selection after `nav` in a bundle of `size` stacks; -1 when there is none.
pub fn navigate(size: usize, selected: i32, nav: Nav) -> i32 {
    if size == 0 {
        return -1;
    }
    let cols = columns(size);
    let (s, w, h) = (size as i32, cols as i32, rows(size, cols) as i32);
    match nav {
        Nav::Up => offset_vertical(s, w, h, selected, -1),
        Nav::Down => offset_vertical(s, w, h, selected, 1),
        Nav::Left => offset_horizontal(s, w, h, selected, -1),
        Nav::Right => offset_horizontal(s, w, h, selected, 1),
    }
}

// Keys arrive in a `&self` hook; the selection changes in the next tick, which
// has the game mutably. The tick also says which slot holds a hovered bundle.
static PENDING: Mutex<Vec<(usize, Nav)>> = Mutex::new(Vec::new());
static HOVERED: AtomicI64 = AtomicI64::new(-1);

pub fn push(slot: usize, nav: Nav) {
    if let Ok(mut pending) = PENDING.lock() {
        pending.push((slot, nav));
    }
}

pub fn take() -> Vec<(usize, Nav)> {
    PENDING
        .lock()
        .map(|mut pending| std::mem::take(&mut *pending))
        .unwrap_or_default()
}

pub fn set_hovered(slot: Option<usize>) {
    HOVERED.store(slot.map_or(-1, |s| s as i64), Ordering::Relaxed);
}

pub fn hovered() -> Option<usize> {
    usize::try_from(HOVERED.load(Ordering::Relaxed)).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grid_grows_square_from_four_wide() {
        assert_eq!(columns(1), 4);
        assert_eq!(columns(16), 4);
        assert_eq!(columns(17), 5);
        assert_eq!(columns(64), 8);
        assert_eq!(rows(13, 4), 4);
        assert_eq!(rows(17, 5), 4);
    }

    #[test]
    fn keys_walk_the_grid_like_bundles_beyond() {
        // 6 stacks: 4 wide, 2 high, the top row's first 2 cells empty.
        // grid: [_ _ 0 1] [2 3 4 5]
        assert_eq!(navigate(6, -1, Nav::Right), 0);
        assert_eq!(navigate(6, 0, Nav::Right), 1);
        assert_eq!(navigate(6, 1, Nav::Right), 0);
        assert_eq!(navigate(6, 2, Nav::Left), 5);
        assert_eq!(navigate(6, 0, Nav::Down), 4);
        assert_eq!(navigate(6, 5, Nav::Up), 1);
        // Up from an item under an empty cell, on a 2 high grid: nowhere.
        assert_eq!(navigate(6, 2, Nav::Up), -1);
        // One row: the first key selects, the next clears.
        assert_eq!(navigate(3, -1, Nav::Down), 0);
        assert_eq!(navigate(3, 0, Nav::Down), -1);
    }
}
