# Documentation File/Folder Structure
  - `index.md` _(Contains links to each sub-folder and a listing of their groups)_

  - `enum/`             _(All global enums)_
    - `index.md`        _(Alphabetical listing of each enum)_
    - `AICommandState.md`   _(Full definition of enum `AICommandState`)_
    - `...`
    - `group/`          _(Enums grouped by purpose or name)_
      - `_index.md`     _(Alphabetical listing of enum groups)_
      - `ai.md`         _(AI related enums)_
      - `anim.md`       _(Animation related enums)_
      - `move.md`       _(Movement related enums)_
      - `...`

  - `func/`             _(All global functions)_
    - `index.md`        _(Alphabetical listing of each global function)_
    - `OperatorLess.md` _(Full definition of global func `OperatorLess`)_
    - `...`
    - `group/`          _(Functions grouped by purpose or name)_
      - `_index.md`     _(Alphabetical listing of func groups)_
      - `operators.md`  _(Operator funcs)_
      - `log.md`        _(Logging funcs)_

  - `class/`
    - `index.md`        _(Alphabetical listing of each enum)_
    - `IScriptable.md`  _(Full definition of class `IScriptable`)_
    - `group/`
      - `_index.md`     _(Alphabetical listing of enum groups)_
      - `abstract.md`   _(List of `abstract` classes)_

  - `files/`            _(All files either as read or from bundle information)_
    - `index.md`        _(List of files in this dir)_
    - `subdir/`         _(Generated sub-directories)_
