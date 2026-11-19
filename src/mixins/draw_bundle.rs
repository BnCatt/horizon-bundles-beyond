// Cancellable HEAD of `draw_bundle_tooltip`, standing in for its body: the
// game's body (Nightly 0b14f8a) with the progress bar as wide as the grid
// (vanilla: 96) and Bundles Beyond's "n/64" label drawn as is rather than
// translated. Always breaks.
fn bundles_beyond_draw_bundle_tooltip(
    game: &mut Game<'_>,
    batch: &mut HudBatch,
    layout: &horizon_client::bundle::TooltipLayout,
) -> ::core::ops::ControlFlow<()> {
    for (i, (at, stack)) in layout.slots.iter().enumerate() {
        let selected = layout.selected == Some(i);
        let sprite = |name: &str| format!("gui/sprites/container/bundle/{name}");
        horizon_client::ui::nine_slice(
            batch,
            &game.gui.atlas,
            &sprite(if selected {
                "slot_highlight_back"
            } else {
                "slot_background"
            }),
            at[0],
            at[1],
            24.0,
            24.0,
            u32::MAX,
        );
        let stack = stack.clone();
        let [x, y] = *at;
        game.draw_item(batch, &stack, x + 4.0, y + 4.0);
        if selected {
            horizon_client::ui::nine_slice(
                batch,
                &game.gui.atlas,
                &sprite("slot_highlight_front"),
                x,
                y,
                24.0,
                24.0,
                u32::MAX,
            );
        }
    }
    let atlas = &game.gui.atlas;
    if let Some((at, hidden)) = layout.overflow {
        let label = horizon_client::text::Text::literal(format!("+{hidden}"));
        let w = atlas.font.width_text(&label);
        batch.text_styled(
            atlas,
            &label,
            at[0] + 12.0 - (w / 2.0).floor(),
            at[1] + 10.0,
            u32::MAX,
            true,
        );
    }
    if let Some((at, description)) = layout
        .empty_description
        .zip(Some("item.minecraft.bundle.empty.description"))
    {
        let text = horizon_client::text::Text::translate(description, &[]);
        let mut ty = at[1];
        for line in atlas.font.split_lines(&text, layout.width) {
            batch.text_styled(atlas, &line.0, at[0], ty, 0xFFAA_AAAA, false);
            ty += 9.0;
        }
    }
    let [bx, by] = layout.bar;
    let fill_sprite = if layout.full {
        "gui/sprites/container/bundle/bundle_progressbar_full"
    } else {
        "gui/sprites/container/bundle/bundle_progressbar_fill"
    };
    horizon_client::ui::nine_slice(batch, atlas, fill_sprite, bx + 1.0, by, layout.fill, 13.0, u32::MAX);
    // The one change: the grid's width, not vanilla's 96.
    let bar_width = layout.width.max(96.0);
    horizon_client::ui::nine_slice(
        batch,
        atlas,
        "gui/sprites/container/bundle/bundle_progressbar_border",
        bx,
        by,
        bar_width,
        13.0,
        u32::MAX,
    );
    if let Some(key) = layout.label {
        let label = if mod_bundles_beyond::is_weight_label(key) {
            horizon_client::text::Text::literal(key)
        } else {
            horizon_client::text::Text::translate(key, &[])
        };
        let w = atlas.font.width_text(&label);
        batch.text_styled(
            atlas,
            &label,
            bx + (bar_width / 2.0).floor() - (w / 2.0).floor(),
            by + 3.0,
            u32::MAX,
            true,
        );
    }
    ::core::ops::ControlFlow::Break(())
}
