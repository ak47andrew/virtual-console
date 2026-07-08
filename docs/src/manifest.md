# Manifest

`manifest.toml` is the actual brain of your cartridge that ties everything together and let's compiler figure out locations of every other file. Fully filled up manifest would look something like this:

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

This is a completely optional piece of data. It's designed to tell minimal info about your game. It's used by launcher or game download software (when there will be one), but console itself doesn't use it whatsoever

- `name` *(optional)* - name of your game
- `version` *(optional)* - version of your game in [SemVer format](https://semver.org/) (tho it's purely conventional, it's not checked anywhere as of now)

### Settings

This is the runner/console setup you need to make your game work

- `ram_size` - the amount of bytes allocated for RAM. See [Memory](./memory.md)
- `stack_size` - the amount of bytes allocated for Stack. See [Memory](./memory.md)

### Resources

Section that tells compiler where does different external files are located

- `entry` - path (relative to the core folder) where `.vea` file with code is located. See [Assembly language](./assembler.md)
- `palette` - path where your `JASC Palette` file is located. See [Palette](./palette.md)
- `bg` - HashMap where keys are non-negative integers and values are their corresponding background images. Used for `bg` command. See [Images](./images.md) for more info
- `img` - HashMap where keys are non-negative integers and values are their corresponding sprites. Used for `img` command. See [Images](./images.md) for more info

## Logo

There's also a single hardcoded path (other then `manifest.toml` itself) and it's `logo.png`. This is a file (not necessary palette-indexed) which represents logo of your game. The launcher, when scanning game's folder, is gonna take `logo.png` (if exist) and use it as logo for the game. Otherwise it's just gonna use blue rectangle instead
