# what

Silphium is a tool that generates static sites visualizing the unit stats for Rome: Total War mods (for both the remastered and the original game).

![Terminal demo](demo.gif)

# how

Start by creating a subfolder called `faust` inside your mod's folder (alongside the `data`, not inside). In there you should create a couple of files before generating anything.

- Create an image file named `banner.png`. The dimensions should be 512x256 pixels. This file will be used as the banner for the mod on the generated site.
- Create a text file named `faust.yml` and enter the following as the contents:

  ```yaml
  id: mod_id
  name: The Name of the Mod
  ```

  You can replace `mod_id` with whatever you want to be used in the site URLs to identify the mod, and `The Name of the Mod` with whatever text you want
  to be used to identify the mod in the site's pages.

(Once a release is available) Grab a release build from the releases page. (Until then, you will have to clone this repo and run `cargo build` to build the binary yourself; obviously, this means you need a Rust dev environment setup) Then, run `faust path/to/faust.yml` in a terminal (obviously replace that with the correct path to the `faust.yml` file). This will parse the mod folder and output all of the site files into a folder named `site` next to the `faust.yml` file.
(If you're not keen on messing with the terminal, you can also drag-and-drop the `faust.yml` file onto the `faust.exe` file.)

You can now upload the contents of that folder to your favourite static site hoster (e.g. GitHub Pages) and visit the site in your browser.

# when

This project is currently under development, and made available only for testing purposes. It will be ready when it is ready.
