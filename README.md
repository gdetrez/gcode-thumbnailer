# gcode-thumbnailer

Extract thumbnails from gcode produced by prusaslicer.

## Debugging thumbnailer issues

Remove thumnail cache:
```
rm -rf ~/.cache/thumbnails 
```

Start nautilus with debug message enabled:
```
G_MESSAGES_DEBUG=all nautilus ~/Prints
```

Make sure there's no existing instance of nautilus first, otherwise this will
exit immediately!
