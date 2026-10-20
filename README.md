This tool finds all TPF binders in a given directory, extracts all the internal DDS files, and converts them to TGA format. These files are then written to an output directory (`./images` by default).  

The main use for this is to have images available for JPEXS flash decompiler to load. Simply run this tool, and then open any gfx files you want to edit from within the `images` folder.  

Note that this dumps full atlas textures. It does not automatically crop subtextures. This means that it is mostly useful for games pre-layout-system, e.g Bloodborne or Dark Souls 3. If you need to dump subtextures as well, for example, for Elden Ring, then write yourself a simple script to crop .tga bytes in place. You can use [SoulsFormats](https://github.com/soulsmods/SoulsFormatsNEXT) or [Ferrisouls](https://github.com/dash512/Ferrisouls) as this tool does.
