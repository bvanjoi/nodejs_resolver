# Changelog

## 0.0.33

- do not support auto prefix '.' any more in `extensions`.

## 0.0.32

- cached `exportField` and `importsField`.

## 0.0.31

- use `CacheFile` to read `package.json`, therefore, remove `unsafe_cache` and use `cache` instead.

## 0.0.30

- do not support `alias_fields` any more, instead of `options.browser_field`.

## 0.0.29

- fix alias_filed resolve bug:

  before:

  ```package.json
  {
    "browser": {
      "./toString": "xxxx"
    }
  }
  ```

  and the file structure is:

  ```
  | xxxx.js
  ```

  then `resolve('toString')` will return `xxxx.js`, and this bug had fixed, it will throw Error now.

## 0.0.28

- support `sideEffects` in package.json, and export `load_sideeffects`.

## 0.0.27

- use `serde_json::from_str` instead of `serde_json::from_reader`.

## 0.0.26

- fix imports field redirect scope range.
- support external unsafe cache.

## 0.0.25

- do not resolve as dir when encounter an in-exists node_modules directory.
- fix a infinity loop in `AliasPlugin`.

## 0.0.24

- fix a bug under pnpm which will resolve incorrect package.json and return unexpected result.

## 0.0.23

- use `jsonc_parse` to parse `tsconfig.json`.

## 0.0.22

- fix a bug under pnpm.

## 0.0.21

- optimize `pkg_info` cache.

## 0.0.20

- fix `pkg_info` cache missing.
- introduce `tracing`.

## 0.0.19

- use `Resolver::_resolve` for tsconfig/extends.

## 0.0.18

- fix error resolve when request has scope path with exportsField.

## 0.0.17

- code optimization.
- remove node build_in detection.
- no longer support `modules` filed in options.
- no longer support node buildIn module, such as `resolve(xxx, 'fs')` will throw error when there is no `'fs'` polyfill.
- changed `Option<String>` to `AliasMap` in `Options.alias` and `PkgFileInfo.aliasField`.
- support tsconfig path mapping.

## 0.0.16

- fix a bug caused by `Path::with_extension`.

  `Path::with_extension` will replace the last string by dot sign, for example, `'a.b'.with_extension('c')` will return `'a.c'`, but we expected `'a.b.c'`.

## 0.0.15

- `forEachBail` for alias.
- fallback when `base_dir.join(target)` is not a valid path.

## 0.0.14

- support `enforce_extension` option.

## 0.0.13

- use `Arc` in cache.

## 0.0.12

- expose `is_build_in_module`.

## 0.0.11

- change the property type of `Request` from `String` to `SmolStr`.
- optimized the `Err` report.

## 0.0.10

- optimized constants in code.

## 0.0.9

- add `enable_unsafe_cache` in `ResolverOptions`, because user sometimes change the `DescriptionFile`, which can lead to some potential problems in `self.cache`.

## 0.0.8

- support `prefer_relative` feature.
- remove `with_xxx` methods, instead of manual assignment.

## 0.0.7

- public `Options`, and change it `description_file` type from `String` to `Option<String>`.

## 0.0.5 && 0.0.6

yanked

## 0.0.4

- support `Debug` trait. According to [Debuggability](https://rust-lang.github.io/api-guidelines/debuggability.html#debuggability), all public API types should be implements `Debug`.

## 0.0.3

- (fixture): use `dashmap` to implement cache.
- (fixture): change `resolver.with_base_dir(xxxx).resolve(target)` to `resolver.resolve(xxxx, target)`.
- (chore): add `Windows` and `MacOS` ci environment.
- (refactor): Add coverage test.

## 0.0.2

- support [`exports`](https://nodejs.org/api/packages.html#exports) and [`imports`](https://nodejs.org/api/packages.html#imports) in package.json.

## 0.0.1

init
