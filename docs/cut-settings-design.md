# Cut settings and layers redesign

LightBurn's Cut Settings Editor and Cuts/Layers panel both work. Operators learn
them. The layout still fights the task. This note records what is wrong and the
design LaserCraft uses instead. The mocks are `preview/cut-settings-redesign.png`
and `preview/layers-panel-redesign.png`.

## What is wrong today

The editor hides its own shape. A dropdown picks the mode and silently rewrites
the bottom half of the form. A separate "Show Timings" button swaps the whole
lower panel for a timings page. The two most structural choices in the dialog are
the two least visible.

The Cuts/Layers panel edits the mode inline on each row. A mode change on its own
is never a real edit. Every mode wants a different set of parameters. The whole
layer has to change with it. The quick-edit fields for the row sit in a cramped
strip at the bottom of the window, far from the row they belong to. The panel
wastes its vertical space. It spends it on oversized toolbar buttons.

The three tools also disagree on names. The same knob is "Mark Speed" in EZCAD
and "Speed" in LightBurn and here. Nothing on screen tells an operator that these
are the same control. `docs/terminology-ezcad-lightburn.md` is the full map.

## The editor

Mode is a visible segmented selector across the top. Line, Fill, Concentric and
Image stay in view at all times. The active one is obvious. Each segment carries
its LightBurn and EZCAD alias. A Fill then reads as "hatch, Type 2" to someone
arriving from EZCAD.

The parameters every mode shares live in one card that never moves. The card
holds speed, power, pulse frequency, pulse width, passes and global passes. Below
it, tab navigation replaces the Show Timings button. The Cut tab holds the
mode-specific controls. The Timings tab holds the per-layer timing overrides. The
Advanced tab holds the rest. Tabs let an operator move between the cut and its
timings while the shared parameters stay in view above.

Every field shows its LightBurn and EZCAD name as quiet subtext. The panel is its
own Rosetta. No one has to keep a translation table open beside the screen.

## The layers panel

Mode is a read-only badge on each row, coloured by kind. A mode change
reconfigures the whole layer. It is not an in-place edit. A double-click on the
row opens the full editor, where that change belongs.

A row shows the layer colour, the number, the mode badge, the speed, the power,
the output toggle and the show toggle. The cramped bottom strip is gone. The rows
are denser and fill the panel. The toolbar shrinks to small icons.

## How this maps to the model

The editor's fields are the fields of `CutSetting` in `lasercraft-laser-doc`. The
mode selector is `FillMode`. The Cut tab reads the fill or line parameter group
for the active mode. The Timings tab reads the optional per-layer timing override.
That override mirrors the device defaults in `lasercraft-profile`. The aliases
come from the terminology map and ride along as field metadata. The interface and
the documentation then never drift apart.
