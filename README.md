# LaserCraft (provisional name)

A ground-up Rust reimplementation of laser-machine control software, built to reach feature parity with LightBurn and then surpass it. LaserCraft joins the Craft app family as a headless engine plus an egui desktop GUI plus a CLI plus a WebAssembly build, standing on the shared `craft-libs` workspace.

## Why

LightBurn is the capable incumbent for hobby and prosumer laser work. It is also closed, single-vendor, and weak on localization. It carries a node-locked license layer on top of that. LaserCraft aims for a clean, open, hardened alternative that keeps the workflow laser operators already know while fixing the parts that frustrate them.

## Goals

1. Feature parity with LightBurn's core workflow: import art, lay out a job, assign cut/engrave/scan settings per layer, plan toolpaths, stream to the controller.
2. Full i18n localization from the first commit, following the Craft suite translation-table convention.
3. Hardened security posture: no credential baked into the binary, network access gated through CloudCraft, a reviewable update channel.
4. A WebAssembly build target so a job can be opened and previewed in a browser, matching the Craft web story.
5. Reuse of `craft-libs` for the UI shell, cloud, collaborative editing, and fonts. None of that gets reinvented.

## Boundary: clean-room reimplementation

This project reimplements laser-control **functionality and behavior**. It does not copy LightBurn's source code, its compiled bytes, or its assets into this tree. REA (Reverse Engineer Anything) is used to understand how LightBurn behaves at its interfaces: how it plans motion, how it frames GCode, Ruida, and Marlin controller traffic, and how it structures job data. LightBurn's public documentation is a first-class reference. The license and activation surface (`LexActivator`, the `crypto*` DLLs) is out of scope and is not analyzed.

## Status

See `docs/PLAN.md` for the phased plan and `docs/rea-workflow.md` for the reverse-engineering toolchain. REA evidence lands in `rea-evidence/`.
