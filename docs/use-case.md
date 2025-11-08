# Actors (MVP)
- Artist (User) – uses the editor.
- File System – open/save project files, import images.
- Image Exporter – encodes flattened image to PNG/JPEG.
- Autosave Timer – triggers background autosave of a temp file.
- Clipboard – copy/paste pixels or selections.

# Use Cases (MVP)
- UC-01 Create Project – start a new canvas with initial layer.
- UC-02 Open Project (.ppp) – load custom project format.
- UC-03 Save Project (.ppp) – write project to disk.
- UC-04 Export PNG/JPEG – flatten layers and export.
- UC-05 Import Image as Layer – add PNG/JPEG as a new layer.
- UC-06 Manage Layers – add, delete, rename, reorder, show/hide, set opacity.
- UC-07 Draw with Tools – pencil/brush/eraser/line on active layer.
- UC-08 Color Picker – pick color from palette/canvas.
- UC-09 Selection/Move – rectangular selection, move/transform within a layer.
- UC-10 Undo/Redo – revert/apply last operations.
- UC-11 Zoom and Pan – view navigation.
- UC-13 Change Brush Size – adjust stroke radius.
- UC-15 Lock/Unlock Layer – prevent layer edits.
- UC-16 Duplicate Layer – copy an existing layer.
- UC-17 Clear Layer – erase all pixels of a layer.
- UC-18 Canvas Resize – change document dimensions.

```plantuml
@startuml
left to right direction

actor "Artist (User)" as User
actor "File System" as FS
actor "Image Exporter" as EXP
actor "Autosave Timer" as TIMER
actor Clipboard as CB

rectangle ProPaint {
  usecase "UC-01 Create Project" as UC01
  usecase "UC-02 Open Project (.ppp)" as UC02
  usecase "UC-03 Save Project (.ppp)" as UC03
  usecase "UC-04 Export PNG/JPEG" as UC04
  usecase "UC-05 Import Image as Layer" as UC05
  usecase "UC-06 Manage Layers" as UC06
  usecase "UC-07 Draw with Tools" as UC07
  usecase "UC-08 Color Picker" as UC08
  usecase "UC-09 Selection/Move" as UC09
  usecase "UC-10 Undo/Redo" as UC10
  usecase "UC-11 Zoom and Pan" as UC11
  usecase "UC-13 Change Brush Size" as UC13
  usecase "UC-15 Lock/Unlock Layer" as UC15
  usecase "UC-16 Duplicate Layer" as UC16
  usecase "UC-17 Clear Layer" as UC17
  usecase "UC-18 Canvas Resize" as UC18
}

User --> UC01
User --> UC02
User --> UC03
User --> UC04
User --> UC05
User --> UC06
User --> UC07
User --> UC08
User --> UC09
User --> UC10
User --> UC11
User --> UC13
User --> UC15
User --> UC16
User --> UC17
User --> UC18

UC02 --> FS : read file
UC03 --> FS : write file
UC04 --> EXP : encode PNG/JPEG
UC05 --> FS : import PNG/JPEG
UC03 .. TIMER : autosave trigger
UC09 --> CB : copy/paste
@enduml
```
