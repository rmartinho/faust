# what

Silphium is a tool that generates static sites visualizing the unit stats for Rome: Total War mods (for both the remastered and the original game).

# how

Start by creating a subfolder called `faust` inside your mod's folder. In there you should create a couple of files before generating anything.
(These are optional, but you will probably want them anyway. If you're in a hurry to see results, you can skip this and move on to obtaining a release.)
Create an image file named `banner.png`, with dimensions 512x256 pixels; that will be used as the banner for the mod on the generated site.
Then create a text file name `faust.yml` and enter the following as the contents:

```yaml
id: mod_id
name: The Name of the Mod
```

You can replace `mod_id` with whatever you want to be used in the site URLs to identify the mod, and `The Name of the Mod` with whatever text you want
to be used to identify the mod in the site's contents.

(Once a release is available) Grab a release build from the releases page. (Until then, you will have to clone this repo and run `cargo build` to build the binary yourself.) Then, open a terminal in the mod folder and run `faust`. This will parse the mod folder and output all of the site files into the `faust/dist` folder.

You can now upload the contents of that folder to your favourite static site hoster (e.g. GitHub Pages) and visit the site in your browser.

# when

This project is currently under development, and made available only for testing purposes. It will be ready when it is ready.
