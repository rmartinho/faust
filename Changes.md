## 0.3.0

- Estimate and display speed
- Reduce size of internal data structures by 80%; this saves bandwidth and improves loading speed

## 0.2.2 2025-08-02

- Parse EDB for Medieval II mods
- Include some abilities in the output for Medieval II mods
- Skip mod list page if there is only one mod (currently only one mod is supported anyway)
- Unused factions can be specified in the manifest to be excluded from the output
- Horde units are now shown, with a button to select between horde and settled units
- Generalized horde/non-horde and era filters to be a single filtering system; this will make adding future filters easier
- The size of the icon files has been reduced significantly
- Manifest file can now include `unit_info_images: false` in order to use unit images from data/ui/unit_info/ instead of data/ui/units/

## 0.2.1 2025-07-29

- Firefox support
- Fixed generic images bug
- Parse descr_strat for faction order
- Groups units in general categories
- Sort units by minimum settlement level required
- Nicer URLs with faction aliases
- Add support for "eras"
