---
description: "Oak is the agentic substrate for software development: the version-control and storage layer autonomous coding agents build on. Mount large repos without a full clone, branch per task, snapshot up to 95% faster than git, and bring your own agent: Claude Code, Codex, Cursor."
title: Oak — the agentic substrate
image: https://oak.space/social-card-217ab603.png
---

the agentic substrate

# Oak is version control built for people and agents working together.

What are the odds git is the fastest way for agents to work? We don't think it is, so we're building something that is. In our benchmarks, snapshots and status checks run up to 95% faster than git. You don't have to change your model or your IDE. Just tell your agents to use Oak.

`curl -fsSL oak.space/install | sh` Copy Copy agent setup

then oak login oak init · [Get started](https://oak.space/signup) · source at [oak.space/oak/oak](https://oak.space/oak/oak)

![](https://oak.space/oak-stipple.svg)

## Git is awesome, but it wasn't built for agents.

We think git is great, but it's not perfect. Building our own version control lets us fix agent workflows directly instead of working around git.

01

### Mount instead of clone.

`oak mount` gives you a working tree without cloning the repo. It downloads the file list up front and fetches each file the first time something reads it. Each task gets its own mount on its own branch, and an `oak space` holds the mounts for every repo a task touches.

With git you'd clone the full history first, which takes minutes on a big repo, then add a worktree per task. Every worktree shares one `.git`, and if that gets corrupted, every task is stuck.

$ oak mount acme/app ./fix-auth fix-auth--mount

src/auth/login.rs4.1Klocal

src/auth/session.rs6.8Klazy

src/main.rs2.0Klocal

api/12 fileslazy

Cargo.lock186Klazy

02

### Commit without writing messages.

Commits on a branch don't need a message, so agents can checkpoint as often as they like. You write one description for the branch, and `oak merge` uses it as the message on the single squash commit that lands on `main`. Oak refuses direct pushes to `main`, so a misconfigured agent can't push there by accident.

Git wants a message on every commit, which mostly gets you a history full of "wip" and "fix", plus the tokens spent writing them.

$ oak switch -c feat/oauth $ oak commit # checkpoint, no message $ oak commit $ oak desc "Replace REST auth with OAuth + PKCE" $ oak merge # one squash on main

03

### Large files work without LFS.

Oak splits files into content-defined chunks and stores each chunk once. If you change one tensor in a 4 GB checkpoint and push again, only the chunks that changed get uploaded.

Git needs LFS for this, which has its own quota and setup, and it re-uploads the whole file when any part of it changes.

In our benchmarks, snapshotting a task with multi-GB binaries takes 321 ms in Oak and 5,218 ms in git.

$ oak commit --push # 4 GB checkpoint $ oak commit --push # after an edit, uploads only the changed chunks

## You can see what's in the file.

Agents commit meshes, textures, audio, model weights and datasets as readily as code, and somebody still has to look at them. Oak's file view previews 231 formats in the browser. Everything below is a real file rendered by the same code as the repo file view, so please poke at it.

acme/game main drag to orbit

art/oak.glb3D model art/acorn.voxvoxel scene art/sunset.exrHDR image audio/theme.wavaudio audio/theme.midMIDI maps/park.geojsonmap models/acorn-nano.ggufLLM weights shaders/canopy.wgslWebGPU shader

Pick a file to preview it.

Try to stump us. Type a file extension.

. Surprise me

#### 3D 19

.glb.gltf.vrm.fbx.obj.stl.ply.3ds.dae.3mf.x3d.usd.usda.step.stp.ifc.vox.bvh.mtl

#### Game engines 16

.uasset.umap.scn.res.tscn.tres.godot.gdshader.unity.prefab.anim.uproject.uplugin.blend.usdc.unitypackage

#### Images 42

.psd.psb.exr.hdr.dds.ktx.ktx2.tga.tif.tiff.xcf.qoi.pcx.sgi.miff.xpm.pbm.pgm.ppm.pam.dng.cr2.cr3.nef.arw.raf.orf.rw2.pef.srw.jp2.jxl.avif.heic.gif.png.jpg.jpeg.webp.bmp.svg.ai

#### Audio & video 29

.wav.mp3.flac.ogg.opus.m4a.aac.aiff.caf.ac3.wma.mp4.mov.webm.mkv.avi.flv.wmv.mpeg.m2ts.3gp.ogv.mid.midi.swf.ttf.otf.woff.woff2

#### ML models 10

.gguf.safetensors.onnx.pt.pth.tflite.pb.pbtxt.pkl.joblib

#### Data & science 33

.csv.tsv.xlsx.ods.sqlite.parquet.arrow.feather.avro.orc.h5.hdf5.nc.fits.npy.npz.mat.rds.sas7bdat.dta.sav.dcm.nii.edf.bson.cbor.msgpack.duckdb.grib.mseed.jsonl.ndjson.ipynb

#### Maps & point clouds 13

.geojson.topojson.kml.gpx.shp.gpkg.mbtiles.pmtiles.las.laz.e57.pcd.dxf

#### Documents 23

.pdf.epub.fb2.mobi.azw.chm.fig.sketch.kra.pptx.ppt.doc.xls.md.mmd.dot.org.ics.vcf.srt.vtt.eml.mbox

#### Archives & disk images 27

.zip.tar.gz.tgz.bz2.xz.zst.7z.rar.cab.cpio.whl.nupkg.vsix.crx.xpi.apk.aar.ipa.jar.dex.iso.dmg.qcow2.vmdk.vhd.vdi

#### Everything else 19

.glsl.hlsl.metal.wgsl.proto.graphql.thrift.sql.patch.diff.pem.crt.csr.p12.pcap.pcapng.har.plist.torrent

## Git versus Oak.

Median p50 latency from the same benchmark harness, on identical repositories. Run it yourself at [oak.space/oak/benchmarks](https://oak.space/oak/benchmarks). If you find anything wrong, please let us know.

scenario / operation delta p50 time

many large binariestask snapshot−95%git443 msoak23.2 ms

wide dirty treeinitial snapshot−94%git849 msoak50.9 ms

multi-GB binariesfull diff (info recall)−93%git3,945 msoak271 ms

shared checkout · 8 workersparallel worker−72%git419 msoak116 ms

Git is still faster at cold repo init with 50k files (14.8 ms vs 42.6 ms) and at process startup (22.3 ms vs 54.2 ms). Those are small fixed costs. Over a long agent session, the snapshots and status checks add up to much more.

## We solve problems that GitHub can't.

GitHub does a hundred things Oak doesn't: issues, GitHub-style code review, an ecosystem a decade deep. But git and GitHub share a model that was never built for agents.

| for agent workflows | oak | github + git |
| --- | --- | --- |
| first working tree | ✓ lazy mount; no full clone | \~ full clone up front; minutes on big repos |
| parallel tasks on one repo | ✓ a mount per task, each its own branch | \~ git worktree; one shared, fragile .git |
| commits | ✓ messageless checkpoints; description becomes the squash message | · every commit needs a message |
| large files | ✓ native; fastcdc dedupes across versions | \~ via LFS, separate quota, extra config |
| snapshot speed | ⚡ up to 95% lower p50 | \~ re-hashes the tree; seconds on big repos |
| CI | ✓ Actions-style workflows on your own runners; gates merges | ✓ native, mature, a decade deep |
| issues, GitHub-style code review, ecosystem | · not yet; bring your own, or it's on the roadmap | ✓ native, mature, a decade deep |

## Questions you might have.

### Do I have to change my agent?

No. Any coding agent that can run shell commands can use the `oak` CLI, including Claude Code, Codex, and Cursor. The "Copy agent setup" button at the top copies onboarding instructions (install, sign-in, the Oak agent skill, and the rules that differ from git) to paste straight into your agent.

### How is Oak faster than git?

Oak stores content by hash, splits large files into chunks, and only fetches files when they're read, so it doesn't re-hash the whole tree or transfer data it already has. Git is still faster at cold repo init and process startup.

### Does Oak run CI?

Yes, on your own machines. Register them as runners, put GitHub Actions–style workflows in `.oak/workflows/*.yml`, and they run when you push or merge, or when you trigger them yourself. Secrets are encrypted per repo and logs stream live. Oak won't squash a branch onto `main` while its CI is red or still running, and `oak merge --wait` waits for it to finish.

### Can I export back to git?

`oak export \<dest\>` replays your full branch history into a standard git repo, keeping the author, email, and timestamp of every commit. Stock `git clone` also works against a read-only endpoint that serves the current `main` tree.

### Does Oak train on my code?

No. Oak trains no models on user code and makes no AI calls on your behalf. Whatever coding agent you run alongside Oak is its own separate integration with its own privacy posture.

### What platforms does Oak support?

The CLI runs on macOS (Apple Silicon and Intel), Linux (x86\_64), and Windows (x86\_64). Lazy mounts use FSKit on macOS, FUSE on Linux, and ProjFS on Windows. Install with `curl -fsSL oak.space/install | sh`, or `irm https://oak.space/install.ps1 | iex` on Windows. Linux ARM64 binaries ship on the GitHub releases page; installer support for them is on the roadmap.

### get updates by email

Notes when something ships. Roughly monthly. No spam, unsubscribe anytime.

Subscribe

or join [discord](https://discord.gg/UUPfUaeDnS) · read the [dev blog](https://oak.space/blog) · follow on [x.com](https://x.com/oakdotspace)

[\<h3\>about oak</h3>Oak is built by Zach Geier with Adam Morse shaping the product and visual system. It is the version control stack we wanted once agents started doing real work: fast branches, lazy mounts, clean history, and no training on your code.Read the full note →](https://oak.space/blog/git-is-forever)

### file feedback from the CLI

Your agents hit the friction before you do. `oak feedback -m "…" --json` files a bug or feature request straight from the terminal. To have them do it automatically, add this to your CLAUDE.md or AGENTS.md. We know it costs your agents some extra tokens, and we really appreciate it.

```
Watch for Oak friction as you work: bugs, anything that costs extra tool calls, tokens, or wall-clock on an end-to-end flow, or a missing capability you had to work around. File it immediately, then continue:
oak feedback -m "bug:|friction:|feature: <goal in one sentence; exact commands and output; the counted cost or expected-vs-actual; the concrete fix>" --json
One report per issue.
```

Copy prompt

```json
{
      "@context": "https://schema.org",
      "@type": "SoftwareApplication",
      "name": "Oak",
      "alternateName": "Oak VCS",
      "applicationCategory": "DeveloperApplication",
      "applicationSubCategory": "Version Control System",
      "operatingSystem": "macOS, Linux, Windows",
      "softwareHelp": "https://oak.space/docs",
      "downloadUrl": "https://oak.space/install",
      "offers": { "@type": "Offer", "price": "0", "priceCurrency": "USD", "description": "Free during the beta, with 100GB of storage per organization." },
      "description": "Oak is the agentic substrate for software development: the version-control and storage layer autonomous coding agents build on. Mount remote repositories without a full clone, run a branch per task in parallel, snapshot up to 95% faster than git, and bring your own agent: Claude Code, Codex, or Cursor.",
      "featureList": [
        "Lazy mounts: usable working tree without a full clone",
        "Branch-per-session model with branch-description-as-commit-message",
        "Content-addressed storage with content-defined chunking for large files",
        "Up to 95% lower p50 latency than git on snapshots, dirty status, and large-binary diffs",
        "git export: replay full history into a standard git repo at any time",
        "Does not train on your code and makes no AI calls on your behalf"
      ],
      "url": "https://oak.space",
      "sameAs": [
        "https://discord.gg/UUPfUaeDnS",
        "https://www.linkedin.com/company/oakvcs",
        "https://x.com/oakdotspace"
      ]
    }
{
      "@context": "https://schema.org",
      "@type": "Organization",
      "name": "Oakspace Inc.",
      "url": "https://oak.space",
      "logo": "https://oak.space/logo.png",
      "sameAs": [
        "https://discord.gg/UUPfUaeDnS",
        "https://www.linkedin.com/company/oakvcs",
        "https://x.com/oakdotspace"
      ]
    }
```

