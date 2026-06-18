# Quickstart

## Step 1. Run the game

Seriously! And grab all you need from [github's release page](https://github.com/ak47andrew/virtual-console/releases): compiler, runtime and example.vec

Once everything is done - move everything in one seperate folder and just drag `example.vec` over runtime (or pull up terminal and run `./runtime.exe example.vec` if you're that guy). And play! Use arrows to move around and clap in excitement, understanding how complex the underyeilding technology is. And don't worry - you're gonna make game like this in no time.

## Step 2. It's actually time to make a game

Start with creating a folder and calling it something like "BestGameEver". Inside of it create `manifest.toml` with the following content:

```toml
[settings]
ram_size = 1000
stack_size = 1000

[resources]
entry = "source.vea"
palette = "palette.pal"
```

A little note on what you're actually typing (you can find full info at [Manifest page](./manifest.md)):
- `ram_size` - Number of bytes in the RAM sector. The game you're about to write doesn't use any so setting 2000 is enough
- `stack_size` - Number of bytes in the stack sector of the memory. Same as RAM
- `entry` - Path to the file with an actual code
- `palette` - Path to the file with palette data

## Step 3. The code

Game will be the same you played at Step 1, but with a pixel and black screen instead of images (I'm not doing "Draw your assets" step. If you want to look into how this one was done - check out `example_cartridge` in `examples` and the rest of the docs)

So, create file `source.vea`, open it up in some text editor and let's start doing it step by step
