This tool finds all TPF binders in a given directory, extracts all the internal DDS files, and converts them to TGA format. These files are then written to an output directory (`./images` by default).  

The main use for this is to have images available for JPEXS flash decompiler to load. Simply run this tool, and then open any gfx files you want to edit from within the `images` folder.  

## Usage:
  
```
tga_dumper "path/to/menu" "path/to/output" --sprites=yes
```

If no output field is given, it writes to `.\images`. `sprites` is disabled by default. If enabled, it looks for layouts in the same directory and crops the atlases to their subtextures.  
