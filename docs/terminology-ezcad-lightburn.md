# Galvo terminology: LaserCraft, LightBurn and EZCAD

A galvo fiber setup speaks three dialects for the same knobs. EZCAD (the JCZ
software most fiber machines ship with) names things one way. LightBurn renames
many of them. LaserCraft uses a plain name and shows the other two in the
interface so an operator coming from either tool recognises the control.

This table is the source of truth for those names. The LaserCraft column is the
field name in `lasercraft-laser-doc`. Confidence is marked where EZCAD labels
vary between controller cards and software versions.

## Operation and mode

| LaserCraft | LightBurn | EZCAD | What it does |
|:--|:--|:--|:--|
| Run | Cut / Run | Mark | Fire the laser along the geometry. |
| Line | Line | Mark (outline) | Trace the outline of each shape. |
| Fill | Fill | Hatch, bidirectional (Type 2) | Sweep back and forth to fill a region. |
| Concentric Fill | Offset Fill | Hatch, loop / ring (Type 3) | Fill inward along offsets of the outline. |
| Image | Image | Bitmap / Dot mark | Engrave a raster by dot or scan line. |

The EZCAD "hatch type" number is the usual source of confusion. Type 2 is
LightBurn's Fill. Type 3 is LightBurn's Offset Fill.

## Power and pulse

| LaserCraft | LightBurn | EZCAD | Unit | What it does |
|:--|:--|:--|:--|:--|
| Speed | Speed | Mark Speed | mm/s | Marking travel speed. |
| Power | Max Power | Power / Current | % | Laser power as a percentage of full scale. |
| Min Power | Min Power | (ramp, not a pen field) | % | Power at corners and low speed. |
| Pulse Frequency | Frequency | Frequency / Freq | kHz | Pulse repetition rate. |
| Pulse Width | Q-Pulse Width | Q Pulse Width | ns | Pulse duration, MOPA and UV only. |
| Passes | Number of Passes | Mark Count / Loop Count | count | Repeats of this layer's geometry. |
| Global Passes | Global Passes | Total loop count | count | Repeats of every sub-layer in sequence. |

## Fill shaping

| LaserCraft | LightBurn | EZCAD | Unit | What it does |
|:--|:--|:--|:--|:--|
| Line Interval | Line Interval | Line Space / Hatch spacing | mm | Gap between fill lines. |
| Lines per Inch | Lines per Inch | (derived) | 1/in | The same spacing as a density. |
| Hatch Angle | Angle | Hatch Angle | deg | Direction of the fill lines. |
| Angle Increment | Angle Increment | Angle increment per pass | deg | Angle added to each successive pass. |
| Auto Rotate | Auto-Rotate | Rotate hatch each mark | toggle | Advance the angle every time the job runs. |
| Dot Width Adjust | Dot Width Adjust | (beam compensation) | mm | Shorten scan lines to offset beam width. |
| Ramp Length | Ramp Length | (slope / grade) | mm | Taper the edge over this distance. |

## Line shaping

| LaserCraft | LightBurn | EZCAD | Unit | What it does |
|:--|:--|:--|:--|:--|
| Kerf Offset | Kerf offset | Edge offset / compensation | mm | Shift the cut to hold final size. |
| Perforation | Perforation Mode | (dashed line / skip) | mm | Cut a dashed line as cut and skip lengths. |
| Tabs / Bridges | Tabs / Bridges | (not native) | mm | Leave small uncut bridges that hold the part. |
| Wobble | Wobble | Wobble / Dithering | mm | Trace small circles to widen the line. |

## Timing

The galvo timing controls use the same "TC" names in current EZCAD and in
LightBurn. These carry over directly. TC is a dwell in microseconds that
compensates for the lag between the commanded position and the mirror position.

| LaserCraft | LightBurn | EZCAD | Unit | What it does |
|:--|:--|:--|:--|:--|
| Laser On Delay | Laser On TC | Laser On TC | us | Wait after the mirrors start before firing. |
| Laser Off Delay | Laser Off TC | Laser Off TC | us | Wait after a mark ends before the laser stops. |
| End Delay | End TC | End TC / Mark Delay | us | Wait at the end of a mark segment. |
| Corner Delay | Polygon TC | Polygon TC | us | Pause at a corner between two segments. |
| Jump Speed | Jump Speed | Jump Speed | mm/s | Travel speed between marks, laser off. |
| Min Jump Delay | Min Jump Delay | Jump Delay (min) | us | Settle time after a short jump. |
| Max Jump Delay | Max Jump Delay | Jump Delay (max) | us | Settle time after a long jump. |
| Jump Distance Limit | Jump Distance Limit | Jump Dist / Limit | mm | Distance above which the max delay applies. |
| Pause Before | Pause Before | (delay before) | ms | Wait before a sub-layer starts. |
| Pause After | Pause After | (delay after) | ms | Wait after a sub-layer ends. |

## Notes on confidence

The timing TC names are confirmed against the LightBurn galvo documentation and
EZCAD delay references. The fill-type mapping (Type 2 to Fill, Type 3 to Offset
Fill) is the standard JCZ hatch convention. A few EZCAD labels differ by
controller card. The EZCAD column lists the common label with the frequent
alternate. Where EZCAD has no equivalent (Tabs, Lines per Inch) the cell says so.

**Sources.** The LightBurn galvo cut-settings and device-settings
documentation, and EZCAD delay references. See `docs/rea-findings.md` for the
device-level model.
