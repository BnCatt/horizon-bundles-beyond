// RETURN of `Contents::shown_count` (`BundleContents.getNumberOfItemsToShow`):
// every stack is shown, so the wheel reaches them all and no "+N" is left.
fn bundles_beyond_shown_count(value: usize, contents: &Contents) -> usize {
    contents.items.len().max(value)
}
