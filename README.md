<div align="center">

<img src="icon.png" alt="Snake icon" width="110">

# Snake

**Keep moving and beat your best score**

[![CI](https://img.shields.io/github/actions/workflow/status/PrimitiveOS/kindle-primitive-snake/ci.yml?branch=main&label=CI)](https://github.com/PrimitiveOS/kindle-primitive-snake/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/PrimitiveOS/kindle-primitive-snake?label=release)](https://github.com/PrimitiveOS/kindle-primitive-snake/releases)
[![Tag](https://img.shields.io/github/v/tag/PrimitiveOS/kindle-primitive-snake?label=tag)](https://github.com/PrimitiveOS/kindle-primitive-snake/tags)
[![Licence: PolyForm Noncommercial 1.0.0](https://img.shields.io/badge/licence-PolyForm%20Noncommercial%201.0.0-blue)](LICENSE)
[![Stars](https://img.shields.io/github/stars/PrimitiveOS/kindle-primitive-snake?style=social)](https://github.com/PrimitiveOS/kindle-primitive-snake/stargazers)
[![Store downloads](https://img.shields.io/endpoint?url=https%3A%2F%2Fprimitiveos.github.io%2Fkindle-primitive-catalog%2Fv1%2Fbadges%2Fprimitive-snake.json)](https://primitiveos.github.io/kindle-primitive-catalog/v1/index.json)

[**Download**](https://github.com/PrimitiveOS/kindle-primitive-snake/releases/latest) · [**Install guide**](INSTALL.md) · [**Changelog**](CHANGELOG.md)

</div>

---

Choose a speed from 1 to 9 and steer a continuously moving snake. Each bite adds a segment and awards points equal to your chosen speed. Fill the board for a 100-point bonus. A wall or body collision ends the round.

Swipe or tap the arrow buttons to steer. Pause at any time and beat your saved best score.

## Quick start

After setting up PrimitiveOS, open **Store** from Primitive Launcher. Choose **Snake** under **Apps**,
tap **Get**, and open it from the Launcher grid. The [install guide](INSTALL.md) also covers the standalone USB package.

## Screenshots

<table>
<tr>
<td align="center"><img src="docs/images/snake.png" width="250" alt="The starting snake and its first piece of food"><br><sub>Ready to play</sub></td>
<td align="center"><img src="docs/images/snake-speed.png" width="250" alt="Nine selectable speeds in the setup screen"><br><sub>Speed selection</sub></td>
</tr>
<tr>
<td align="center"><img src="docs/images/snake-playing-dark.png" width="250" alt="The snake turns toward food in dark mode"><br><sub>Dark mode</sub></td>
<td align="center"><img src="docs/images/snake-game-over.png" width="250" alt="A wall collision ends the game and offers a new round"><br><sub>End of a round</sub></td>
</tr>
</table>

Desktop simulation captures at 1264 × 1680 pixels.

## Develop locally

Keep the Tools and Launcher checkouts next to this repository.

```sh
cargo test --offline --locked --workspace
cargo run --offline --locked -p snake-desktop
```

The desktop run exercises the game timer, steering, pause, and game over. It writes previews to `build/previews/`.

## Highlights

- Choose a speed from 1 to 9 and steer a continuously moving snake.
- Each bite adds a segment and awards points equal to your chosen speed.
- Fill the board for a 100-point bonus.

## Status

| Device | Status |
|---|---|
| Kindle Oasis 3 (1264 × 1680, firmware 5.18.2) | Tested on hardware |

**Current version: 0.1.1.** Changes are in [CHANGELOG.md](CHANGELOG.md).

## Downloads

The badge at the top shows the number of Store downloads. It counts GitHub downloads of this app's release packages, across all versions and architectures, and is refreshed daily from the [Store catalog](https://primitiveos.github.io/kindle-primitive-catalog/v1/index.json).

## Documentation

[Install guide](INSTALL.md) · [Changelog](CHANGELOG.md)

## Licence

Copyright (C) 2026 Doan Nguyen.

Code is licensed under [PolyForm Noncommercial 1.0.0](LICENSE). The documentation and original art are
licensed under [CC BY-NC-SA 4.0](LICENSE-DOCS).
