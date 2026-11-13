# Bundles Beyond for Horizon

A Horizon native mod (horizon-mixins bundle) that brings the behaviour of
[Bundles Beyond](https://github.com/O7410/Bundles-Beyond) by O7410 to
[Horizon](https://github.com/horizon-rust/horizon), the Rust rewrite of
Minecraft: Java Edition 26.3. Client side; works on any server.

- A bundle's tooltip shows **every** item, not 12 and a `+N`: the grid is at
  least 4 wide and grows about square (`max(4, ceil(sqrt n))` columns).
- The mouse wheel selects any item in it, so a click takes out the one you picked.
- Over a bundle in an inventory, the **arrow keys** and your **movement keys**
  (W A S D by default) move the selection through the grid.
- The bar is as wide as the grid and reads `n/64`.

Not ported yet: the Space key click, slot size `+`/`-`, the scroll-axis modes,
reverse view, container-style slots and the on/off key.

## Install

The bundle is `mixins/bundles_beyond/`. Build it with the launcher (it compiles
the game with the mod on the pinned Nightly; native code is not sandboxed, read
it first):

```sh
horizon-launcher native-build mixins/bundles_beyond --trust-native-code
```

Pinned to Nightly 2026-11-13 (`9f42064c11efe7ef3e5ed2baad7ddc2a3b36eea0`). Two
hooks stand in for whole function bodies (`draw_stack_tooltip_with`,
`draw_bundle_tooltip`), so moving the pin means re-checking them against the
new revision (0.1.1 re-copied `draw_stack_tooltip_with`: the tooltip pre-render
hook, the empty bundle's description lines and `item_tooltip::bundle_tooltip`).

A ready `.hnmod` is attached to the GitHub release; drop it into
`horizon-native-mods/` and build in the launcher.

## Licence

A port of Bundles Beyond's behaviour and its grid navigation
(`BundleTooltipAdditions.offsetVertical/offsetHorizontal`), so it keeps that
mod's licence: [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/).
The two drawing helpers copy Horizon's own code (MIT, Horizon contributors).
