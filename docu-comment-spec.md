# Redscript Documentation Comment Specification

Format is a mashup between **JavaDoc/JSDoc/Doxygen**-style tags and **Swift-flavored Markdown**'s
plain markdown syntax. It is aimed to be computer-readable enough to output smart documentation, but
simple enough to be clean and human-readable.
*(I'm looking at you C# and your bloated XML comments)*

Content outside of named tag parameters is treated as Markdown (specifically [Python-Markdown][1])
and is written verbatim to the outputted markdown file(s), with the exception of the trimmed comment
prefixes as described for each comment type below.

[1]: https://python-markdown.github.io/


## Supported Comment Styles

### Multi-line Comment Block
```swift
/**
 * Double-star documentation comment block
 *
 * Either asterisk prefixed

or with no prefix

*/
```
For each line in a comment block, if it starts with:
  - Any amount of whitespace,
  - A single asterisk,
  - At least one space character

Those characters will be stripped from the comment before being output. For this reason, if you're
writing an unordered list in your comments and not prefixing each line, you should use hyphen `-` or
plus `+` prefixed lists

### Single-line Comment
```swift
/// Triple-slash single-line documentation comment
```
For each single-line comment, if there is one or more space characters between the three
slashes `///` and any number of non-space characters, the first of those whitespace characters
will be trimmed from the output.

## Supported Block Tags

Block tags define the relevant scope of the documentation comments.

- `@file`
- `@module [name]`
- `@enum [name]`
- `@class [name]`
- `@func [name]`
- `@field [name]`

All scopes are implicitly defined if the comment preceeds the matching keyword, with the exception
of `@file` which always implicitly refers the file it's written in.

## Supported Inline Tags

Within the `@file` or `@module` scopes:
 - `@author [...]`
 - `@copyright [...]`
 - `@license [...]`
 - `@version [...]`

Within the `@enum`, `@class` or `@func` scopes:
 - `@group [name]`
   - Adds this scope to the named group.
   - This tag exists specifically to help with documenting the decompiled Cyberpunk scripts by
     enabling manual grouping of related objects. You probably won't have a need to use it.

Within the `@func` scope:
 - `@param [name] [description]`
   - Use to add any additional information about a function parameter.
 - `@returns [description]`
   - Use to add any additional information about what the function returns.

Within any scope:
 - `@section [name]`
   - A documentation block within the current scope (assumes `@file` if in no scope)
   - Adds a link to the relevant block's navigation section
 - `@see [name]`
   - Adds a link to a related class/enum/function
   - Supports `scope:name` syntax
   - Supports `class.member` syntax
