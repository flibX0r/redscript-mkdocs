# Documentation File/Folder Structure

There _should_ be no collision between the enum/func/class names and each `group.md` index file, as there aren't any enums, funcs or classes named `group` (as of Hotfix 1.22)

------
  - `index.md` _(Contains links to each sub-folder and a listing of their groups)_

  - `enum.md`           _(Alphabetical listing of each enum)_
  - `enum/`             _(All global enums)_    
    - `AICommandState.md`   _(Full definition of enum `AICommandState`)_
    - `...`
    - `group.md`        _(Alphabetical listing of enum groups)_
    - `group/`          _(Enums grouped by purpose or name)_      
      - `ai.md`         _(AI related enums)_
      - `anim.md`       _(Animation related enums)_
      - `move.md`       _(Movement related enums)_
      - `...`

  - `func.md`        _(Alphabetical listing of each global function)_
  - `func/`             _(All global functions)_
    - `OperatorLess.md` _(Full definition of global func `OperatorLess`)_
    - `...`
    - `group.md`        _(Alphabetical listing of func groups)_
    - `group/`          _(Functions grouped by purpose or name)_      
      - `operators.md`  _(Operator funcs)_
      - `log.md`        _(Logging funcs)_
      - `...`

  - `class.md`          _(Alphabetical listing of each class)_
  - `class/`    
    - `IScriptable.md`  _(Full definition of class `IScriptable`)_
    - `...`
    - `group.md`        _(Alphabetical listing of class groups)_
    - `group/`      
      - `abstract.md`   _(List of `abstract` classes)_
      - `...`

  - `files.md`          _(List of files in the base dir)_
  - `files/`            _(All files either as read or from bundle information)_
    - `[subdir].md`     _(Generated sub-directory listing)_
    - `[subdir]/`       _(Generated sub-directory)_
      - `[file].md`     _(Generated file)_
      - `...`
    - `...`
