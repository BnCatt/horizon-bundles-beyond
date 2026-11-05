"""Pack mixins/bundles_beyond into dist/bundles_beyond.hnmod (the launcher's native mod file).

    python tools/package.py [--install GAME_DIR]

--install copies it to GAME_DIR/horizon-native-mods/ and turns it on in
GAME_DIR/horizon-mods.txt (the Mods page's switch), e.g. %APPDATA%\\horizon.
"""
import json
import pathlib
import re
import shutil
import sys
import tomllib
import zipfile

ROOT = pathlib.Path(__file__).resolve().parent.parent
MOD_ID = "bundles_beyond"
BUNDLE = ROOT / "mixins" / MOD_ID
# The horizon-mixins revision the launcher builds with (launcher/src/native_mods.rs MIXINS_REV).
MIXINS_REV = "e8cd0aa38cffef49cb2e088ef716baa107148c02"
GAME = "26.3"


def package() -> pathlib.Path:
    mixin = tomllib.loads((BUNDLE / "horizon-mixin.toml").read_text(encoding="utf-8"))
    manifest = {
        "schema": 1,
        "id": mixin["mod"]["id"],
        "version": mixin["mod"]["version"],
        "game": GAME,
        "horizon": mixin["compatibility"]["horizon_revision"],
        "mixins": MIXINS_REV,
    }
    assert manifest["id"] == MOD_ID
    out = ROOT / "dist" / f"{MOD_ID}.hnmod"
    out.parent.mkdir(exist_ok=True)
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
        z.writestr("horizon-native.json", json.dumps(manifest, indent=2) + "\n")
        for path in sorted(BUNDLE.iterdir()):
            if path.suffix in (".rs", ".toml"):
                # Forward slashes: the launcher refuses a name with a backslash.
                z.write(path, f"mixins/{MOD_ID}/{path.name}")
    return out


def install(out: pathlib.Path, game_dir: pathlib.Path) -> None:
    dest = game_dir / "horizon-native-mods"
    dest.mkdir(exist_ok=True)
    shutil.copy2(out, dest / out.name)
    mods_list = game_dir / "horizon-mods.txt"
    text = mods_list.read_text(encoding="utf-8") if mods_list.exists() else ""
    key = f"mod.{MOD_ID}"
    lines = [l for l in text.splitlines() if not re.match(rf"\s*{re.escape(key)}\s*:", l)]
    lines.append(f"{key}:true")
    mods_list.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"installed {dest / out.name}, on in {mods_list}")


if __name__ == "__main__":
    built = package()
    print(f"packed {built}")
    if len(sys.argv) == 3 and sys.argv[1] == "--install":
        install(built, pathlib.Path(sys.argv[2]))
