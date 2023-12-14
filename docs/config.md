# Config

> 按字母排序。

## autoCSSModules

- 类型：`boolean`
- 默认值：`false`

是否自动开启 CSS Modules。

如果不开启，只有以 `.module.css` 或 `.module.less` 的文件会被当成 CSS Modules 处理；如果开启，`import styles from './a.css'` 这类具名的 import 也会被当成 CSS Modules 处理。

## clean

- 类型：`boolean`
- 默认值：`true`

是否在 build 前清理输出目录。

## codeSplitting

- 类型：`"none" | "auto"`
- 默认值：`"none"`

拆包策略。

注：多 entry 的拆包策略还在开发中。

## copy

- 类型：`string[]`
- 默认值：`["public"]`

需要拷贝的文件或目录，默认会拷贝 `public` 目录下的文件到输出目录。

## define

- 类型：`Record<string, string>`
- 默认值：`{ NODE_ENV: "development" | "production }`

定义的全局变量。

比如。

```ts
{
  define: {
    "FOO": "foo",
  },
}
```

注：目前的 define 会自动处理 `process.env` 前缀。

## devEval

> 注：已废弃，待移除。

## devtool

- 类型：`"source-map" | "inline-source-map" | "none"`
- 默认值：`"source-map"`

Source Map 类型。

注：`"none"` 类型后续会改成 `false`。

## dynamicImportToRequire

- 类型：`boolean`
- 默认值：`false`

是否将动态 import 转换成 require。

配置后，比如。

```ts
import("./a.js")
// => require("./a.js")
```

## emotion

- 类型：`boolean`
- 默认值：`false`

是否开启 emotion 支持。

## entry

- 类型：`Record<string, string>`
- 默认值：`{}`

入口文件的配置。比如。

```ts
{
  entry: {
    index: "./src/index.js",
    login: "./src/login.js",
  },
}
```

注：多 entry 的 code splitting 支持还在开发中。

## externals

- 类型：`Record<string, string>`
- 默认值：`{}`

外部依赖的配置。比如。

```ts
{
  externals: {
    react: "React",
    "react-dom": "ReactDOM",
  },
}
```

注：external 配置的值还有高级的配置模式，通常用不到，所以这里不展开，有需要可查看源码。

## flexBugs

- 类型：`boolean`
- 默认值：`false`

是否修复 flexbugs。

## hash

- 类型：`boolean`
- 默认值：`false`

是否生成 hash 文件名。

注：后续会改成 `object` 格式，以方便用户做更多控制。

## hmr

- 类型：`boolean`
- 默认值：`true`

是否开启热更新。

## hmrHost

- 类型：`string`
- 默认值：`"127.0.0.1"`

热更新的 host。

注：后续会改成 `hmr` 的子配置。

## hmrPort

- 类型：`number`
- 默认值：`3000`

热更新的端口。

注：后续会改成 `hmr` 的子配置。

## ignoreCSSParserErrors

> 注：已废弃，待移除。

## ignores

- 类型：`string[]`
- 默认值：`[]`

需要忽略的文件。忽略的文件会输出空模块。

## inlineLimit

- 类型：`number`
- 默认值：`10000`

小于 `inlineLimit` 大小的 assets 文件会被转换成 `base64` 格式。

## less

> 注：已废弃，待移除。

## manifest

- 类型：`boolean`
- 默认值：`false`

是否生成 `manifest.json` 文件。

## manifestConfig

- 类型：`{ fileName: string, basePath: string }`
- 默认值：`{ fileName: "asset-manifest.json", basePath: "" }`

`manifest.json` 文件的配置。（注：此配置后续会合并到 `manifest` 配置里）

## mdx

- 类型：`boolean`
- 默认值：`false`

是否开启 `mdx` 支持。

## minify

- 类型：`boolean`
- 默认值：mode 为 development 时为 `false`，production 时为 `true`

是否压缩代码。

注：后续会改成 `Object` 类型，支持更多子配置用于控制压缩参数。

## mode

- 类型：`"development" | "production"`
- 默认值：`"development"`

构建模式，`"development"` 或 `"production"`。

## moduleIdStrategy

- 类型：`"named" | "hashed"`
- 默认值：mode 为 development 时是 `"named"`，production 时是 `"hashed"`

moduleId 的生成策略。

## nodePolyfill

- 类型：`boolean`
- 默认值：`true`，但 platform 为 `"node"` 时为 `false`

是否开启 node polyfill。

## output

- 类型：`{ path: string, mode: "bundle" | "bundless", esVersion: "es3" | "es5" | "es2015" | "es2016" | "es2017" | "es2018" | "es2019" | "es2020" | "es2021" | "es2022" | "esnext", meta: boolean, asciiOnly: boolean, chunkLoadingGlobal: string, preserveModules: boolean, preserveModulesRoot: string }`
- 默认值：`{ path: "dist", mode: "bundle", esVersion: "es2022", meta: false, asciiOnly: true, chunkLoadingGlobal: "", preserveModules: false, preserveModulesRoot: "" }`

和输出相关的配置。

- `path`，输出目录
- `mode`，输出模式，`"bundle"` 或 `"bundless"`，默认为 `"bundle"`
- `esVersion`，输出的 `js` 版本（注：Bundless Only）
- `meta`，是否生成 `meta.json` 文件（注：Bundless Only）
- `asciiOnly`，是否只输出 `ascii` 字符（注：好像没生效，待排查）
- `chunkLoadingGlobal`，`chunk loading` 的全局变量名
- `preserveModules`，是否保留模块目录结构（注：Bundless Only）
- `preserveModulesRoot`，保留模块目录结构的根目录（注：Bundless Only）

## optimizePackageImports

- 类型：`boolean`
- 默认值：`false`

是否优化 package imports。

注：实现属性，暂时勿用。

## platform

- 类型：`"browser" | "node"`
- 默认值：`"browser"`

构建平台，`"browser"` 或 `"node"`。

注：使用 `"node"` 时，目前还需设置 `dynamicImportToRequire` 为 `true`，因为 runtime 还不支持 node 方式的 chunk 加载。

## providers

- 类型：`Record<string, [string, string]>`
- 默认值：`{}`

提供者配置，用于替换代码中的标识符为 require 模块的方式。

比如。

```ts
{
  providers: {
    process: ["process", ""],
    Buffer: ["buffer", "Buffer"],
  },
}
```

以上配置会在遇到 `process` 和 `Buffer` 标识符时将其替换为 require 对应模块的代码。

```ts
process
// => require("process")
Buffer
// => require("buffer").Buffer
```

## publicPath

- 类型：`string`
- 默认值：`"/"`

publicPath 配置。注：有个特殊值 `"runtime"`，表示会切换到 runtime 模式，使用运行时的 `window.publicPath` 作为 publicPath。

## px2rem

- 类型：`boolean`
- 默认值：`false`

是否开启 px2rem 转换。

## px2remConfig

- 类型：`{ root: number, propBlackList: string[], propWhiteList: string[], selectorBlackList: string[], selectorWhiteList: string[] }`
- 默认值：`{ root: 100, propBlackList: [], propWhiteList: [], selectorBlackList: [], selectorWhiteList: [] }`

px2rem 的配置。

注：后续会合并到 px2rem 配置里，并将其改成 `Object` 类型。

- `root`，根节点的字体大小
- `propBlackList`，属性黑名单
- `propWhiteList`，属性白名单
- `selectorBlackList`，选择器黑名单
- `selectorWhiteList`，选择器白名单

## resolve

- 类型：`{ alias: Record<string, string>, extensions: string[] }`
- 默认值：`{ alias: {}, extensions: ["js", "jsx", "ts", "tsx"] }`

`resolve` 相关配置。

- `alias`，别名配置
- `extensions`，文件扩展名配置

比如：

```ts
{
  resolve: {
    alias: {
      "@": "./src",
    },
    extensions: ["js", "jsx", "ts", "tsx"],
  },
}
```

## stats

- 类型：`boolean`
- 默认值：`false`

是否生成 stats.json 文件。

## transformImport

> TODO: @辟殊。

## treeShake

- 类型：`"basic"`
- 默认值：`"basic"`

注：1）配置名后续会改成 `treeShaking`；2）配置值也会做调整，目前只支持 `"basic"` 模式，应该也不会同时支持多种 treeShaking 模式。

注：目前只在 mode 为 "production" 时生效。

## umd

- 类型：`"none" | string`
- 默认值：`"none"`

是否输出 umd 格式的代码。

注：1）后续会改成 `Object` 类型，支持更多子配置用于控制 umd 参数；2）`"none"` 会改成 `false` 类型。

## writeToDisk

- 类型：`boolean`
- 默认值：`true`

是否在 mode 为 development 时将构建结果写入磁盘。