# dprint-plugin-mesonbuild

meson.build code formatting plugin for dprint.

[Grammar](https://mesonbuild.com/Syntax.html#grammar)

## Install

See [Release](https://github.com/TiceCosmos/dprint-plugin-mesonbuild/releases/latest)

## Configuration

| Name               | Type | Default | description                             |
| :----------------- | :--- | ------: | :-------------------------------------- |
| indentWidth        | u8   |       2 | indent width                            |
| alignColon         | bool |   false | align at `:`                            |
| spaceBeforeColon   | bool |   false | spaces before `:`                       |
| spaceInnerBracket  | bool |   false | spaces before `( [ {` and after `) ] }` |
| wrapCloseBrace     | bool |    true | wrap before `) ] }`                     |
| nowrap_before_name | bool |    true | nowrap before name argument             |
