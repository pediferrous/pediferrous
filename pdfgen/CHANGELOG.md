# Changelog

## [0.3.0](https://github.com/pediferrous/pediferrous/compare/pdfgen-v0.2.0...pdfgen-v0.3.0) (2025-08-13)


### Features

* implement `Text` PDF object ([#56](https://github.com/pediferrous/pediferrous/issues/56)) ([33fc735](https://github.com/pediferrous/pediferrous/commit/33fc735895b99f981efe9931976d36cdc9be521c)), closes [#39](https://github.com/pediferrous/pediferrous/issues/39)
* implement basic support for the `Stream` object ([#49](https://github.com/pediferrous/pediferrous/issues/49)) ([7949d9c](https://github.com/pediferrous/pediferrous/commit/7949d9cea9dbf18b60d790ba092ec8d55667ad21)), closes [#35](https://github.com/pediferrous/pediferrous/issues/35)
* implement color management ([#71](https://github.com/pediferrous/pediferrous/issues/71)) ([4f87ffd](https://github.com/pediferrous/pediferrous/commit/4f87ffd0cc49ed04d7878efa810e9a5e0380bdb8)), closes [#40](https://github.com/pediferrous/pediferrous/issues/40)
* implement font support ([#52](https://github.com/pediferrous/pediferrous/issues/52)) ([92cdce0](https://github.com/pediferrous/pediferrous/commit/92cdce0589be53378db1ff24296a603ceb2a3490)), closes [#37](https://github.com/pediferrous/pediferrous/issues/37)
* implement parsing of `Identifier` ([#70](https://github.com/pediferrous/pediferrous/issues/70)) ([77b25b1](https://github.com/pediferrous/pediferrous/commit/77b25b1ec56b105631b24516768d3ff2b0778423)), closes [#66](https://github.com/pediferrous/pediferrous/issues/66)
* implement support for basic UTF8 encoded PDF String ([#50](https://github.com/pediferrous/pediferrous/issues/50)) ([5cd87a9](https://github.com/pediferrous/pediferrous/commit/5cd87a9dda2a82fe104d10d15a5f3e9a64933bca)), closes [#36](https://github.com/pediferrous/pediferrous/issues/36)
* implement support for raster images ([#51](https://github.com/pediferrous/pediferrous/issues/51)) ([a000468](https://github.com/pediferrous/pediferrous/commit/a00046823e080a176d05b7d144f394b520afe4e4)), closes [#41](https://github.com/pediferrous/pediferrous/issues/41)


### Bug Fixes

* re-use variable instead of direct env var check in snap macro ([#46](https://github.com/pediferrous/pediferrous/issues/46)) ([9cf99f4](https://github.com/pediferrous/pediferrous/commit/9cf99f4cea53bcf6e83baf5ba7c1eeeec4ccb199))
* use absolute paths in `snap_test` macro ([#48](https://github.com/pediferrous/pediferrous/issues/48)) ([43944b4](https://github.com/pediferrous/pediferrous/commit/43944b4c691851523ddbe76dd448443b27dd7397))

## [0.2.0](https://github.com/pediferrous/pediferrous/compare/pdfgen-v0.1.0...pdfgen-v0.2.0) (2024-12-07)


### Features

* implement `Catalog` and PDF generation flow ([#31](https://github.com/pediferrous/pediferrous/issues/31)) ([2506644](https://github.com/pediferrous/pediferrous/commit/25066449af1f87140bee9cfb6e1f74e6fe610382)), closes [#6](https://github.com/pediferrous/pediferrous/issues/6) [#12](https://github.com/pediferrous/pediferrous/issues/12)
* implement `CrossReferenceTable` generation ([#28](https://github.com/pediferrous/pediferrous/issues/28)) ([0df708c](https://github.com/pediferrous/pediferrous/commit/0df708c886ae1758152dfa476d6ed6c94cde7155)), closes [#9](https://github.com/pediferrous/pediferrous/issues/9)
* implement `Page` object ([#27](https://github.com/pediferrous/pediferrous/issues/27)) ([bea129b](https://github.com/pediferrous/pediferrous/commit/bea129baa3941b7c05151143bd28f98cdc8330fc)), closes [#8](https://github.com/pediferrous/pediferrous/issues/8)
* implement `PageTree` PDF object ([#30](https://github.com/pediferrous/pediferrous/issues/30)) ([0e5993f](https://github.com/pediferrous/pediferrous/commit/0e5993fa2f2f5d08a8a695868ddc7fb001acfc73)), closes [#7](https://github.com/pediferrous/pediferrous/issues/7)
* implement `trailer` section generation based on the CRT contents ([#32](https://github.com/pediferrous/pediferrous/issues/32)) ([845d215](https://github.com/pediferrous/pediferrous/commit/845d215713759859f5f523ebc4dfd1d13afdcca3)), closes [#29](https://github.com/pediferrous/pediferrous/issues/29)
* implement PDF header; `Document::write` method ([#22](https://github.com/pediferrous/pediferrous/issues/22)) ([9b65ddc](https://github.com/pediferrous/pediferrous/commit/9b65ddcff8e3ddeb676793857f6ac50340bf0570))
* implement PDFs EOF ([#23](https://github.com/pediferrous/pediferrous/issues/23)) ([862f380](https://github.com/pediferrous/pediferrous/commit/862f3806d8b98b63cb3895a817bb44a38ca4c3ef)), closes [#11](https://github.com/pediferrous/pediferrous/issues/11)
* implement the `Name` object ([#26](https://github.com/pediferrous/pediferrous/issues/26)) ([c9598d8](https://github.com/pediferrous/pediferrous/commit/c9598d82be29e6aebc270d028446390e5fd5b4c3)), closes [#24](https://github.com/pediferrous/pediferrous/issues/24)


### Bug Fixes

* remove unused variable and non-existent import ([#16](https://github.com/pediferrous/pediferrous/issues/16)) ([1046c08](https://github.com/pediferrous/pediferrous/commit/1046c08c29d886d2cbaa35ab1e4435a9b544de07)), closes [#15](https://github.com/pediferrous/pediferrous/issues/15)
