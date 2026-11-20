# Install Snake

After [setting up PrimitiveOS](https://github.com/PrimitiveOS/kindle-primitive-launcher/blob/main/docs/INSTALL.md#first-time-setup-over-usb), open **Store** from Launcher.
Find **Snake** under **Apps**, tap **Get**, then open it from the Launcher grid.

## Build a local package

Run from this checkout with Tools, Launcher, and Build next to it.

```sh
cargo zigbuild --offline --locked --release -p snake-kindle --target armv7-unknown-linux-musleabihf
sh scripts/stage.sh armhf
python3 ../doannt-kindle-build/scripts/package.py --id primitive-snake --dir snake \
  --version 0.1.0 --arch armhf --stage build/stage --license LICENSE \
  --docs-license LICENSE-DOCS --install INSTALL.md --out dist
```
