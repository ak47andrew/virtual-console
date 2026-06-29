# Manifest

`manifest.toml` is the actual brain of your cartridge that ties everything together and let's compiler figure out where is everything. Fully filled up manifest would look something like this:

```toml
[metadata]
name = "Test Game"
version = "v1.0"

[settings]
ram_size = 2000
stack_size = 1000

[resources]
entry = "source.vea"
palette = "palette.pal"

[resources.bg]
0 = "bg/bg.png"

[resources.img]
0 = "img/char.png"
```

So let's talk about it chapter by chapter

### Metadata

This is a completely optional and (at least for now) unused piece of data, but it's designed to tell minimal info about your game overall. This is later can be used by launcher or game download software 

- `name` *(optional)* - name of your game
- `version` *(optional)* - version of your game in [SemVer format](https://semver.org/)

### Settings

This is the runner/console setup you need to make your game work

- `ram_size` - the amount of bytes allocated for RAM. See [Memory](./memory.md)
- `stack_size` - the amount of bytes allocated for Stack. See [Memory](./memory.md)

### Resources

Section that tells compiler where does different external files are located

- `entry` - path (relative to the core folder) where `.vea` file with code is located. See [Assembler](./assembler.md)
- `palette` - path where your `JASC Palette` file is located. See [Palette](./palette.md)
- `bg` - HashMap where keys are non-negative integers used for `bg` command and their corresponding background images. See [Images](./images.md) for more info
- `img` - HashMap where keys are non-negative integers used for `img` command and their corresponding sprites. See [Images](./images.md) for more info

## Logo

There's also a single hardcoded path (other then `manifest.toml` itself) and it's `logo.png`. This is a file (not necessary palette-indexed) that's setup logo for your game. The launcher, when scanning game's folder, is gonna take `logo.png` (if exist) and use it as logo for the game. Otherwise it's just gonna use blue rectangle instead
