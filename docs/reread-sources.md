# Reread sources (sibling research repositories)

This file records the exact versions of the three sibling repositories that
were read at implementation time. They are cloned **read-only** into the
working folder and are **git-ignored**: they are not part of the `vole-camera`
crate, its history, or its build. They exist solely to ground the architecture
and interoperability boundary decisions in the actual prior art.

Every VOLE-Camera claim that depends on these sources is pinned to these
commits; do not assume a later upstream change applies (see brief §98).

| Repository | Purpose | Remote | Commit | Date (UTC) |
|------------|---------|--------|--------|------------|
| `vole` | Direct downstream parent; general video representation / interop boundary (`vole-video`) | `https://github.com/infinityabundance/vole.git` | `dcf9f8cec44af9da3ccc6df4317238d96224cc1f` | 2026-09-05 01:48:45 +0100 |
| `entropyfs` | Optional persistence substrate (Phase-P analogue; never required for `.volecam` decode) | `https://github.com/infinityabundance/entropyfs.git` | `cdb4ee3bed6c6ce8ed2ef6fbc882d65989a498ab` | 2026-08-29 00:09:11 +0100 |
| `dsfb` | Zero-authority encoder search governance (later, optional; never normative decode) | `https://github.com/infinityabundance/dsfb.git` | `aa6acd4fdbf1bb30c05ae95bbb41124441191a12` | 2026-05-29 19:34:02 +0100 |

## Prior-art paper

The prior-art paper is kept locally at `research/vole_camera_prior_art.pdf`
(LaTeX source `research/vole_camera_prior_art.tex`), directory `research/` is
also git-ignored (large, non-normative). The paper is:

> de Beer, R. (2026). *VOLE-Camera: Sensor-Native Procedural Entropy
> Factorization and Direct Visual Materialization*, DOI
> `10.5281/zenodo.22312959` (v1.0, dated 5 September 2026).

The same directory holds the implementation brief (`vole-camera prompt.txt`).
