# GodsEye — Sovereign Polyglot Topology Report

> Scanned: 2026-03-31T23:35:19 | 242.7s

| Metric | Value |
|---|---|
| Files | 13389 |
| Lines | 2,628,274 |
| Edges | 619705 |
| Critical Path | 0 files |

## Classification

| Category | Files | Lines | Meaning |
|---|---|---|---|
| CRITICAL | 4256 | 1,066,504 | On critical path or heavily depended on |
| PROTECTED | 85 | 9,495 | Has sovereign name, may be dynamically loaded |
| ACTIVE | 2573 | 318,198 | Other files depend on this |
| SIGNIFICANT | 42 | 199,747 | Real logic but no current dependents |
| ISOLATED | 5264 | 1,032,179 | No dependencies in or out |
| MINIMAL | 1169 | 2,151 | Stub or placeholder (≤5 lines) |

### typescript.js — `CRITICAL`

**200397 lines** | Direct deps: 488 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\typescript\lib\typescript.js`

**Verdict:** 488 files directly depend on this. Transitive: 13388.

**Depends on:** AM.js, AS.js, ASCII.js, ASCII_Hex_Digit.js, Adlam.js, Ahom.js, Alias.js, Alphabetic.js
**Depended by:** 490.htmlServerMain.js, 573.htmlServerMain.js, 769.htmlServerMain.js, 920.cssServerMain.js, 962.jsonServerMain.js, AST.js, AnonymousCredentialPolicy.js, BeautifulMentionsPlugin.js
**Behaviors:** WRITES_FILES
**Classes:** DebugTypeMapper(0m), _Version(0m), _VersionRange(0m)

---

### mermaid.js — `CRITICAL`

**154624 lines** | Direct deps: 12 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\mermaid\dist\mermaid.js`

**Verdict:** 12 files directly depend on this. Transitive: 13388.

**Depends on:** AD.js, AE.js, AM.js, AS.js, ASCII.js, AST.js, Accent.js, Armenian.js
**Depended by:** bootstrap-fork.js, cli.js, components.js, devContainersSpecCLI.js, index.ts, jetskiAgent.js, main.js, mermaid.min.js
**Behaviors:** WRITES_FILES
**Classes:** _SourceLocation(0m), _Token(0m), _ParseError(0m)

---

### rollup.js — `CRITICAL`

**23940 lines** | Direct deps: 22 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\rollup\dist\shared\rollup.js`

**Verdict:** 22 files directly depend on this. Transitive: 13388.

**Depends on:** AF.js, AST.js, AbortController.js, AbortSignal.js, Any.js, ArrayExpression.js, Asset.tsx, AssignmentExpression.js
**Depended by:** cli-api.BKg19Fvw.js, d3-array.js, d3-array.min.js, d3.js, d3.min.js, default-exclude.js, fsevents-importer.js, getLogFilter.js
**Behaviors:** WRITES_FILES, DOES_ML_INFERENCE, HAS_CLASSES
**Classes:** FileEmitter(0m), if(0m), PluginDriver(0m)

---

### d3.js — `CRITICAL`

**20626 lines** | Direct deps: 133 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\d3\dist\d3.js`

**Verdict:** 133 files directly depend on this. Transitive: 13388.

**Depends on:** AM.js, Accent.js, Blues.js, BrBG.js, BuGn.js, BuPu.js, CustomEvent.js, DOMParser.js
**Depended by:** 425.heapsnapshotWorker.js, 769.htmlServerMain.js, 848.extension.web.js, 920.cssServerMain.js, _collections.js, align.js, armasm.js, attr.js
**Behaviors:** HAS_CLASSES
**Classes:** Adder(0m), InternMap(0m), InternSet(0m)

---

### katex.js — `CRITICAL`

**19092 lines** | Direct deps: 25 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\katex\dist\katex.js`

**Verdict:** 25 files directly depend on this. Transitive: 13388.

**Depends on:** AE.js, ASCII.js, Accent.js, Any.js, Armenian.js, BD.js, Bengali.js, CD.js
**Depended by:** accent.js, auto-render.js, auto-render.min.js, bootstrap-fork.js, buildCommon.js, buildHTML.js, buildMathML.js, cli.js
**Behaviors:** HAS_CLASSES
**Classes:** ParseError(0m), Settings(0m), Style(0m)

---

### sinon.js — `CRITICAL`

**18737 lines** | Direct deps: 29 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\sinon\pkg\sinon.js`

**Verdict:** 29 files directly depend on this. Transitive: 13388.

**Depends on:** AF.js, AS.js, ASCII.js, Accent.js, Agent.js, Any.js, BE.js, Blob.js
**Depended by:** bootstrap-fork.js, called-in-order.test.js, class-name.test.js, cli.js, deprecated.js, deprecated.test.js, every.test.js, fake-timers-src.js
**Behaviors:** READS_FILES

---

### jszip.js — `ACTIVE`

**11577 lines** | Direct deps: 3 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\jszip\dist\jszip.js`

**Verdict:** 3 direct dependents use this file.

**Depends on:** ASCII.js, ArrayReader.js, BS.js, Blob.js, CR.js, Call.js, Config.js, Constants.js
**Depended by:** jszip.min.js, license_header.js, zipEntries.js
**Classes:** for(0m), ZipEntries(0m), ZipEntry(0m)

---

### guide.js — `CRITICAL`

**10691 lines** | Direct deps: 143 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\@tanstack\react-router\dist\llms\rules\guide.js`

**Verdict:** 143 files directly depend on this. Transitive: 13388.

**Depends on:** AbortController.js, Any.js, Block.js, Cache.js, CatchBoundary.tsx, Common.js, Compiler.js, Components.js
**Depended by:** 769.htmlServerMain.js, Graphemer.js, HeadContent.dev.tsx, HeadContent.tsx, ScopeManager.js, SplitChunksPlugin.js, all-star-lookahead.js, array-bracket-newline.js
**Classes:** to(0m), before(0m), instances(0m)

---

### psl.js — `CRITICAL`

**9646 lines** | Direct deps: 10 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\psl\dist\psl.js`

**Verdict:** 10 files directly depend on this. Transitive: 13388.

**Depends on:** ASCII.js, Cache.js, Function.js, Get.js, Math.js, Node.js, Number.js, String.js
**Depended by:** components.js, cookie.js, permuteDomain.js, prism-psl.js, prism-psl.min.js, prism-show-language.js, prism-show-language.min.js, psl.min.js

---

### domprops.js — `ACTIVE`

**9027 lines** | Direct deps: 2 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\terser\tools\domprops.js`

**Verdict:** 2 direct dependents use this file.

**Depends on:** AbortController.js, AbortSignal.js, AbstractRange.js, Any.js, Attr.js, BarProp.js, BeforeUnloadEvent.js, Blob.js
**Depended by:** bundle.min.js, propmangle.js

---

### nise.js — `ACTIVE`

**8212 lines** | Direct deps: 3 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\nise\nise.js`

**Verdict:** 3 direct dependents use this file.

**Depends on:** AS.js, ASCII.js, Agent.js, BE.js, Blob.js, CA.js, Common.js, Control.js
**Depended by:** sandbox.js, sinon-esm.js, sinon.js
**Behaviors:** READS_FILES

---

### mappers.js — `CRITICAL`

**8100 lines** | Direct deps: 11 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\@azure\storage-blob\dist-esm\storage-blob\src\generated\src\models\mappers.js`

**Verdict:** 11 files directly depend on this. Transitive: 13388.

**Depends on:** Blob.js, Block.js, Cache.js, Control.js, Enum.js, Format.js, Generator.js, Number.js
**Depended by:** appendBlob.js, blockBlob.js, cytoscape.cjs.js, cytoscape.min.js, cytoscape.umd.js, main.js, mermaid.js, mermaid.min.js

---

### wsn.js — `ACTIVE`

**7640 lines** | Direct deps: 3 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\cytoscape-fcose\demo\samples\wsn.js`

**Verdict:** 3 direct dependents use this file.

**Depends on:** T.js, af.js, arrow.js, background.js, barrel.js, base64.js, bbox.js, be.js
**Depended by:** demo-constraint-control.js, main.js, workbench.desktop.main.js

---

### plist.js — `CRITICAL`

**7398 lines** | Direct deps: 8 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\plist\dist\plist.js`

**Verdict:** 8 files directly depend on this. Transitive: 13388.

**Depends on:** Attr.js, CDATASection.js, CharacterData.js, Comment.js, Common.js, DOMException.js, DOMImplementation.js, DOMParser.js
**Depended by:** darwin.js, envinfo.js, plist-build.js, plist-parse.js, prism-q.js, prism-q.min.js, terminalSuggestMain.js, workbench.desktop.main.js
**Behaviors:** WRITES_FILES

---

### mathematica.js — `CRITICAL`

**7360 lines** | Direct deps: 14 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\highlight.js\lib\languages\mathematica.js`

**Verdict:** 14 files directly depend on this. Transitive: 13388.

**Depends on:** Block.js, CharacterRange.js, Condition.js, ConditionalExpression.js, Constants.js, Control.js, DateString.js, Definition.js
**Depended by:** cli-api.BKg19Fvw.js, codeMirrorModule-BoWUGj0J.js, codeMirrorModule-Bucv2d7q.js, components.js, index-CLLxNdKA.js, jetskiAgent.js, mathematica.js.js, mimeType.js

---

### _data.js — `CRITICAL`

**6989 lines** | Direct deps: 35 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\es5-ext\string\#\normalize\_data.js`

**Verdict:** 35 files directly depend on this. Transitive: 13388.

**Depends on:** eslint.js, exports.js, max.js, module.js, sparse.js, strict.js, style.js
**Depended by:** 769.htmlServerMain.js, 920.cssServerMain.js, 962.jsonServerMain.js, CachedInputFileSystem.js, CharacterData-impl.js, JsonData.js, Lexical.dev.js, Node-impl.js

---

### dist.js — `CRITICAL`

**6758 lines** | Direct deps: 2530 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\vite\dist\node\chunks\dist.js`

**Verdict:** 2530 files directly depend on this. Transitive: 13388.

**Depends on:** AR.js, ASCII.js, AT.js, BD.js, BE.js, BR.js, DE.js, Document.js
**Depended by:** 2019.js, 2020.js, 430.js, 490.htmlServerMain.js, 533.cssServerMain.js, 555.js, 573.htmlServerMain.js, 6-1.js
**Behaviors:** WRITES_FILES

---

### pako.js — `CRITICAL`

**6606 lines** | Direct deps: 13 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\pako\dist\pako.js`

**Verdict:** 13 files directly depend on this. Transitive: 13388.

**Depends on:** BS.js, CR.js, Call.js, Config.js, Constants.js, IN.js, Literal.js, Matches.tsx
**Depended by:** binding.js, deflate.js, flate.js, inflate.js, jszip.js, jszip.min.js, license_header.js, pako.min.js
**Classes:** Deflate(0m), Inflate(0m)

---

### lazy.js — `CRITICAL`

**6575 lines** | Direct deps: 152 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\lazy.js\lazy.js`

**Verdict:** 152 files directly depend on this. Transitive: 13388.

**Depends on:** Any.js, Dash.js, F.js, FO.js, Function.js, Math.js, NO.js, Number.js
**Depended by:** 769.htmlServerMain.js, BinaryMiddleware.js, CacheFacade.js, ContextModule.js, Document-impl.js, Entrypoint.js, EnvUtils.js, FileMiddleware.js
**Behaviors:** DOES_ML_INFERENCE

---

### Compilation.js — `CRITICAL`

**5747 lines** | Direct deps: 116 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\webpack\lib\Compilation.js`

**Verdict:** 116 files directly depend on this. Transitive: 13388.

**Depends on:** ArrayHelpers.js, Asset.tsx, AsyncDependenciesBlock.js, AsyncParallelHook.js, AsyncQueue.js, AsyncSeriesBailHook.js, AsyncSeriesHook.js, Block.js
**Depended by:** AbstractLibraryPlugin.js, AmdLibraryPlugin.js, AssetGenerator.js, AssetModulesPlugin.js, AssignLibraryPlugin.js, AsyncModuleRuntimeModule.js, AsyncWasmLoadingRuntimeModule.js, AsyncWebAssemblyModulesPlugin.js
**Behaviors:** HAS_CLASSES
**Classes:** Compilation(0m)

---

### build2.js — `ACTIVE`

**5538 lines** | Direct deps: 2 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\vite\dist\node\chunks\build2.js`

**Verdict:** 2 direct dependents use this file.

**Depends on:** ASCII.js, BatchedHash.js, Call.js, Combinator.js, Comment.js, Function.js, Hash.js, ID.js
**Depended by:** _tsc.js, typescript.js
**Behaviors:** WRITES_FILES
**Classes:** Core(0m), name(0m), or(0m)

---

### JavascriptParser.js — `CRITICAL`

**5148 lines** | Direct deps: 104 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\webpack\lib\javascript\JavascriptParser.js`

**Verdict:** 104 files directly depend on this. Transitive: 13388.

**Depends on:** AST.js, ArrayExpression.js, AssignmentExpression.js, BasicEvaluatedExpression.js, BinaryExpression.js, Block.js, CallExpression.js, ChainExpression.js
**Depended by:** AMDDefineDependency.js, AMDDefineDependencyParserPlugin.js, AMDPlugin.js, AMDRequireArrayDependency.js, AMDRequireContextDependency.js, AMDRequireDependenciesBlockParserPlugin.js, AMDRequireDependency.js, AMDRequireItemDependency.js
**Behaviors:** HAS_CLASSES
**Classes:** extends(0m), VariableInfo(0m), JavascriptParser(0m)

---

### polyfill.js — `CRITICAL`

**5011 lines** | Direct deps: 73 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\web-streams-polyfill\dist\polyfill.js`

**Verdict:** 73 files directly depend on this. Transitive: 13388.

**Depends on:** AS.js, AbortController.js, AbortError.js, AbortSignal.js, Any.js, BE.js, CreateAsyncFromSyncIterator.js, DOMException.js
**Depended by:** AvroParser.js, GenericWorker.js, GetGlobalObject.js, GraphemerHelper.js, IsTimeZoneOffsetString.js, LexicalNodeContextMenuPlugin.dev.js, LexicalNodeContextMenuPlugin.prod.js, ModuleNotFoundError.js
**Behaviors:** READS_FILES, WRITES_FILES
**Classes:** in(0m)

---

### ponyfill.js — `CRITICAL`

**4983 lines** | Direct deps: 12 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\web-streams-polyfill\dist\ponyfill.js`

**Verdict:** 12 files directly depend on this. Transitive: 13388.

**Depends on:** AS.js, AbortController.js, AbortError.js, AbortSignal.js, Any.js, BE.js, CreateAsyncFromSyncIterator.js, DOMException.js
**Depended by:** Lexical.dev.js, babelBundleImpl.js, import-meta-resolve.js, isNative.js, lodash.min.js, main.js, memoize-one.cjs.js, memoize-one.esm.js
**Behaviors:** READS_FILES, WRITES_FILES
**Classes:** in(0m)

---

### decimal.js — `CRITICAL`

**4952 lines** | Direct deps: 145 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\decimal.js\decimal.js`

**Verdict:** 145 files directly depend on this. Transitive: 13388.

**Depends on:** Get.js, Integer.js, Math.js, Node.js, Range.js, String.js, Symbol.js, Transform.js
**Depended by:** 769.htmlServerMain.js, 920.cssServerMain.js, HTMLInputElement-impl.js, JSXTransformer.js, LexicalHashtagPlugin.dev.js, UnicodeRange.js, _tsc.js, angular.js

---

### main.js — `CRITICAL`

**4808 lines** | Direct deps: 3440 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\out\jetskiAgent\main.js`

**Verdict:** 3440 files directly depend on this. Transitive: 13388.

**Depends on:** AD.js, AE.js, AF.js, AG.js, AI.js, AL.js, AM.js, AO.js
**Depended by:** 2019.js, 2020.js, 430.js, 490.htmlServerMain.js, 533.cssServerMain.js, 555.js, 573.htmlServerMain.js, 6-1.js
**Behaviors:** READS_DATABASE, READS_FILES, WRITES_FILES, MAKES_HTTP_REQUESTS, MONITORS_HARDWARE, DOES_ML_INFERENCE
**Classes:** diagrams(0m), diagram(0m), extends(0m)

---

### esquery.js — `CRITICAL`

**4181 lines** | Direct deps: 7 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\esquery\dist\esquery.js`

**Verdict:** 7 files directly depend on this. Transitive: 13388.

**Depends on:** AS.js, AST.js, ArrayExpression.js, AssignmentExpression.js, BE.js, BY.js, BinaryExpression.js, CallExpression.js
**Depended by:** commentHandler.js, esquery.esm.min.js, esquery.lite.js, esquery.lite.min.js, esquery.min.js, iterateJsdoc.js, source-code-traverser.js
**Classes:** name(0m), to(0m), name(0m)

---

### FileSystemInfo.js — `CRITICAL`

**4090 lines** | Direct deps: 9 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\webpack\lib\FileSystemInfo.js`

**Verdict:** 9 files directly depend on this. Transitive: 13388.

**Depends on:** AsyncQueue.js, Children.js, Common.js, File.js, Hash.js, Logger.js, Math.js, Node.js
**Depended by:** Compilation.js, HttpUriPlugin.js, Module.js, NodeWatchFileSystem.js, PackFileCacheStrategy.js, ResolverCachePlugin.js, Watching.js, fs.js
**Behaviors:** HAS_CLASSES
**Classes:** SnapshotIterator(0m), SnapshotIterable(0m), Snapshot(0m)

---

### grammar.js — `CRITICAL`

**3933 lines** | Direct deps: 145 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\langium\lib\grammar\generated\grammar.js`

**Verdict:** 145 files directly depend on this. Transitive: 13388.

**Depends on:** CharacterRange.js, Condition.js, DO.js, ID.js, Type.js, TypeDefinition.js, WS.js, attributes.js
**Depended by:** 769.htmlServerMain.js, 920.cssServerMain.js, BlobServiceClient.js, _tsc.js, all-types.js, ariaPropsMap.js, ast-collector.js, ast-reflection-interpreter.js

---

### Element.js — `CRITICAL`

**3719 lines** | Direct deps: 167 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\jsdom\lib\jsdom\living\generated\Element.js`

**Verdict:** 167 files directly depend on this. Transitive: 13388.

**Depends on:** Attr.js, Node.js, Set.js, ShadowRootInit.js, Symbol.js, Window.js, after.js, append.js
**Depended by:** 425.heapsnapshotWorker.js, 769.htmlServerMain.js, 848.extension.web.js, CSSStyleDeclaration.js, DOMElement.js, DOMElementFilter.js, FlowTransformer.js, HTMLElement-impl.js
**Classes:** Element(0m)

---

### HTMLElement.js — `CRITICAL`

**3490 lines** | Direct deps: 212 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\jsdom\lib\jsdom\living\generated\HTMLElement.js`

**Verdict:** 212 files directly depend on this. Transitive: 13388.

**Depends on:** Element.js, EventHandlerNonNull.js, OnErrorEventHandlerNonNull.js, Set.js, Symbol.js, Window.js, blur.js, boolean.js
**Depended by:** FormData.js, HTMLAnchorElement-impl.js, HTMLAnchorElement.js, HTMLAreaElement-impl.js, HTMLAreaElement.js, HTMLBRElement-impl.js, HTMLBRElement.js, HTMLBaseElement-impl.js
**Classes:** HTMLElement(0m)

---

### beautify.js — `CRITICAL`

**3421 lines** | Direct deps: 10 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\js-beautify\js\lib\beautify.js`

**Verdict:** 10 files directly depend on this. Transitive: 13388.

**Depends on:** AS.js, BE.js, DO.js, IE.js, IN.js, IS.js, Matches.tsx, Module.js
**Depended by:** 769.htmlServerMain.js, 920.cssServerMain.js, beautifier.js, beautifier.min.js, beautify-css.js, beautify-html.js, bundle.min.js, compile-dots.js

---

### ast.js — `CRITICAL`

**3402 lines** | Direct deps: 452 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\terser\lib\ast.js`

**Verdict:** 452 files directly depend on this. Transitive: 13388.

**Depends on:** AS.js, AST.js, BE.js, BY.js, Block.js, Call.js, Function.js, IE.js
**Depended by:** 1-mergeAtrule.js, 2-initialMergeRuleset.js, 3-disjoinRuleset.js, 4-restructShorthand.js, 6-restructBlock.js, 7-mergeRuleset.js, 769.htmlServerMain.js, 8-restructRuleset.js
**Behaviors:** HAS_CLASSES
**Classes:** AST_Token(0m), of(0m), of(0m)

---

### csso.js — `CRITICAL`

**3323 lines** | Direct deps: 7 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\csso\dist\csso.js`

**Verdict:** 7 files directly depend on this. Transitive: 13388.

**Depends on:** AnPlusB.js, Atrule.js, AtrulePrelude.js, AttributeSelector.js, Block.js, Brackets.js, CDC.js, CDO.js
**Depended by:** 6-restructBlock.js, css-tools.js, csso.min.js, inlineStyles.js, minifyStyles.js, style.js, svgo.browser.js
**Behaviors:** DOES_ML_INFERENCE
**Classes:** simpleSelector(0m), node(0m), for(0m)

---

### mhchem.js — `CRITICAL`

**3213 lines** | Direct deps: 6 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\katex\dist\contrib\mhchem.js`

**Verdict:** 6 files directly depend on this. Transitive: 13388.

**Depends on:** AS.js, Call.js, Definition.js, F.js, Helpers.js, IS.js, Other.js, Set.js
**Depended by:** arrow.js, katex.js, mermaid.js, mhchem.min.js, stretchy.js, svgGeometry.js

---

### isbl.js — `ACTIVE`

**3206 lines** | Direct deps: 3 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\highlight.js\lib\languages\isbl.js`

**Verdict:** 3 direct dependents use this file.

**Depends on:** Assigned.js, CR.js, Common.js, Constants.js, File.js, Format.js, Other.js, Rule.js
**Depended by:** isbl.js.js, jetskiAgent.js, workbench.js

---

### SVGElement.js — `CRITICAL`

**3156 lines** | Direct deps: 30 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\jsdom\lib\jsdom\living\generated\SVGElement.js`

**Verdict:** 30 files directly depend on this. Transitive: 13388.

**Depends on:** Element.js, EventHandlerNonNull.js, OnErrorEventHandlerNonNull.js, SVGAnimatedString.js, Set.js, Symbol.js, Window.js, attribute.js
**Depended by:** HTMLElement-impl.js, SVGDescElement-impl.js, SVGDescElement.js, SVGGraphicsElement-impl.js, SVGGraphicsElement.js, SVGMetadataElement-impl.js, SVGMetadataElement.js, SVGTitleElement-impl.js
**Classes:** SVGElement(0m)

---

### gml.js — `CRITICAL`

**3131 lines** | Direct deps: 15 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\highlight.js\lib\languages\gml.js`

**Verdict:** 15 files directly depend on this. Transitive: 13388.

**Depends on:** abs.js, all.js, argument.js, bool.js, break.js, ceil.js, clamp.js, const.js
**Depended by:** cli-api.BKg19Fvw.js, components.js, gml.js.js, index-CLLxNdKA.js, jetskiAgent.js, mimeType.js, prism-autoloader.js, prism-autoloader.min.js

---

### finder.js — `ACTIVE`

**3117 lines** | Direct deps: 3 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\@asamuzakjp\dom-selector\src\js\finder.js`

**Verdict:** 3 direct dependents use this file.

**Depends on:** AST.js, Control.js, DOMException.js, Deprecated.js, Document.js, DocumentFragment.js, Element.js, ElementInternals.js
**Depended by:** mel.js, transformer.js, utility.js
**Classes:** Finder(0m), function(0m), functions(0m)

---

### hintingtt.js — `ACTIVE`

**3057 lines** | Direct deps: 1 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\opentype.js\src\hintingtt.js`

**Verdict:** 1 direct dependents use this file.

**Depends on:** Control.js, Function.js, GT.js, Get.js, LT.js, MD.js, Math.js, Number.js
**Depended by:** font.js

---

### corePlugins.js — `CRITICAL`

**3008 lines** | Direct deps: 5 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\tailwindcss\src\corePlugins.js`

**Verdict:** 5 files directly depend on this. Transitive: 13388.

**Depends on:** Deprecated.js, Screen.js, Set.js, accent.js, accessibility.js, active.js, add.js, adjust.js
**Depended by:** create-plugin-list.js, generate-types.js, getAllConfigs.js, resolveConfig.js, setupContextUtils.js

---

### lowercase.js — `CRITICAL`

**2902 lines** | Direct deps: 80 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\@babel\types\lib\builders\generated\lowercase.js`

**Verdict:** 80 files directly depend on this. Transitive: 13388.

**Depends on:** ArrayExpression.js, AssignmentExpression.js, BinaryExpression.js, BindExpression.js, CallExpression.js, ConditionalExpression.js, File.js, FunctionExpression.js
**Depended by:** 769.htmlServerMain.js, 920.cssServerMain.js, Clients.js, ContainerClient.js, CoreUtils.js, HttpManager.js, Lexical.dev.js, Lexical.prod.js

---

### validator.js — `CRITICAL`

**2891 lines** | Direct deps: 52 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\playwright-core\lib\protocol\validator.js`

**Verdict:** 52 files directly depend on this. Transitive: 13388.

**Depends on:** Control.js, Point.js, ValidationError.js, WebSocket.js, absolutePath.js, active.js, add.js, after.js
**Depended by:** 962.jsonServerMain.js, OverlappingFieldsCanBeMergedRule.js, ajv.bundle.js, ajv.min.js, attributes.js, babelBundleImpl.js, browserServerImpl.js, cascading-config-array-factory.js

---

### unix.js — `CRITICAL`

**2868 lines** | Direct deps: 42 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\cytoscape-fcose\demo\samples\unix.js`

**Verdict:** 42 files directly depend on this. Transitive: 13388.

**Depends on:** ada.js, af.js, arrow.js, background.js, barrel.js, bbox.js, bold.js, border.js
**Depended by:** ZipFileWorker.js, _tsc.js, android.js, bigIntSupport.js, bootloader.js, builds.js, cli-api.BKg19Fvw.js, cli-engine.js

---

### DefaultStatsFactoryPlugin.js — `CRITICAL`

**2691 lines** | Direct deps: 8 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\webpack\lib\stats\DefaultStatsFactoryPlugin.js`

**Verdict:** 8 files directly depend on this. Transitive: 13388.

**Depends on:** AggressiveSplittingPlugin.js, Asset.tsx, Children.js, Chunk.js, ChunkGraph.js, ChunkGroup.js, Compilation.js, Compiler.js
**Depended by:** Compilation.js, DefaultStatsPresetPlugin.js, DefaultStatsPrinterPlugin.js, MultiStats.js, Stats.js, StatsFactory.js, StatsPrinter.js, WebpackOptionsApply.js
**Behaviors:** HAS_CLASSES
**Classes:** DefaultStatsFactoryPlugin(0m)

---

### linter.js — `CRITICAL`

**2677 lines** | Direct deps: 17 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\eslint\lib\linter\linter.js`

**Verdict:** 17 files directly depend on this. Transitive: 13388.

**Depends on:** AST.js, Any.js, Block.js, Comment.js, Config.js, ESLint.js, Function.js, Get.js
**Depended by:** cli-engine.js, eslint-helpers.js, eslintrc-incompat.js, file-context.js, file-enumerator.js, file-report.js, installation.js, mathtex-script-type.js
**Behaviors:** HAS_CLASSES
**Classes:** Linter(0m)

---

### sqf.js — `CRITICAL`

**2663 lines** | Direct deps: 10 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\highlight.js\lib\languages\sqf.js`

**Verdict:** 10 files directly depend on this. Transitive: 13388.

**Depends on:** abs.js, agent.js, agents.js, all.js, append.js, apply.js, assert.js, backpack.js
**Depended by:** components.js, jetskiAgent.js, prism-autoloader.js, prism-autoloader.min.js, prism-show-language.js, prism-show-language.min.js, prism-sqf.js, prism-sqf.min.js

---

### output.js — `CRITICAL`

**2533 lines** | Direct deps: 455 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\terser\lib\output.js`

**Verdict:** 455 files directly depend on this. Transitive: 13388.

**Depends on:** AS.js, AST.js, BE.js, BY.js, Call.js, IN.js, IS.js, Math.js
**Depended by:** 769.htmlServerMain.js, 920.cssServerMain.js, 962.jsonServerMain.js, APIPlugin.js, AbstractLibraryPlugin.js, AssetGenerator.js, AssignLibraryPlugin.js, AutoPublicPathRuntimeModule.js
**Behaviors:** HAS_CLASSES
**Classes:** Rope(0m), differs(0m)

---

### iterateJsdoc.js — `CRITICAL`

**2486 lines** | Direct deps: 61 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\eslint-plugin-jsdoc\src\iterateJsdoc.js`

**Verdict:** 61 files directly depend on this. Transitive: 13388.

**Depends on:** AST.js, Any.js, Block.js, Comment.js, Declaration.js, ESLint.js, FunctionExpression.js, Integer.js
**Depended by:** alignTransform.js, checkAccess.js, checkAlignment.js, checkExamples.js, checkIndentation.js, checkLineAlignment.js, checkParamNames.js, checkPropertyNames.js

---

### Clients.js — `CRITICAL`

**2379 lines** | Direct deps: 12 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\@azure\storage-blob\dist-esm\storage-blob\src\Clients.js`

**Verdict:** 12 files directly depend on this. Transitive: 13388.

**Depends on:** AnonymousCredential.js, Any.js, Batch.js, Blob.js, BlobDownloadResponse.js, BlobLeaseClient.js, BlobQueryResponse.js, BlobSASSignatureValues.js
**Depended by:** BlobBatch.js, ContainerClient.js, authclient.js, chrome-remote-interface.js, client-request.js, clientHelpers.js, http2-session-manager.js, lib.webworker.d.ts
**Classes:** BlobClient(0m), AppendBlobClient(0m), BlockBlobClient(0m)

---

### predicates.js — `CRITICAL`

**2329 lines** | Direct deps: 34 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\robust-predicates\umd\predicates.js`

**Verdict:** 34 files directly depend on this. Transitive: 13388.

**Depends on:** Math.js, abs.js, az.js, c.js, ca.js, cat.js, cd.js, code.js
**Depended by:** ExecutableDefinitionsRule.js, KnownTypeNamesRule.js, PossibleTypeExtensionsRule.js, UniqueDirectivesPerLocationRule.js, _baseConforms.js, _baseConformsTo.js, all-star-lookahead.js, assertRecord.js

---

### parseAst.js — `CRITICAL`

**2318 lines** | Direct deps: 6 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\rollup\dist\shared\parseAst.js`

**Verdict:** 6 files directly depend on this. Transitive: 13388.

**Depends on:** ArrayExpression.js, AssignmentExpression.js, BinaryExpression.js, CallExpression.js, ChainExpression.js, Chunk.js, ConditionalExpression.js, Config.js
**Depended by:** cli-api.BKg19Fvw.js, loadConfigFile.js, node-entry.js, rollup.js, watch-cli.js, watch.js
**Classes:** extends(0m)

---

### _collections.js — `CRITICAL`

**2169 lines** | Direct deps: 23 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\svgo\plugins\_collections.js`

**Verdict:** 23 files directly depend on this. Transitive: 13388.

**Depends on:** TR.js, accent.js, adjust.js, align.js, all.js, anchor.js, animation.js, any.js
**Depended by:** AsymmetricMatcher.js, DOMCollection.js, Immutable.js, _applyTransforms.js, cleanupIDs.js, collapseGroups.js, convertColors.js, convertPathData.js

---

### ElementInternals.js — `CRITICAL`

**2151 lines** | Direct deps: 8 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\jsdom\lib\jsdom\living\generated\ElementInternals.js`

**Verdict:** 8 files directly depend on this. Transitive: 13388.

**Depends on:** Set.js, Symbol.js, Window.js, aria.js, const.js, context.js, conversions.js, convert.js
**Depended by:** HTMLElement-impl.js, bundle.min.js, domprops.js, finder.js, lib.dom.d.ts, main.js, ts-morph-common.js, workbench.desktop.main.js
**Classes:** ElementInternals(0m)

---

### endpoints.js — `CRITICAL`

**2107 lines** | Direct deps: 26 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\@octokit\plugin-rest-endpoint-methods\dist-src\generated\endpoints.js`

**Verdict:** 26 files directly depend on this. Transitive: 13388.

**Depends on:** activity.js, add.js, annotations.js, app.js, archive.js, base.js, blocks.js, branch.js
**Depended by:** 769.htmlServerMain.js, 920.cssServerMain.js, bundle.min.js, chrome-remote-interface.js, customClient.js, cytoscape.cjs.js, cytoscape.min.js, cytoscape.umd.js

---

### ConcatenatedModule.js — `CRITICAL`

**2081 lines** | Direct deps: 5 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\webpack\lib\optimize\ConcatenatedModule.js`

**Verdict:** 5 files directly depend on this. Transitive: 13388.

**Depends on:** ArrayHelpers.js, CachedSource.js, ChunkGraph.js, CodeGenerationResults.js, Compilation.js, ConcatSource.js, ConcatenationScope.js, Dependency.js
**Depended by:** ConcatenationScope.js, EvalSourceMapDevToolPlugin.js, ModuleConcatenationPlugin.js, ModuleLibraryPlugin.js, internalSerializables.js
**Behaviors:** HAS_CLASSES
**Classes:** properties(0m), ConcatenatedModule(0m), expression(0m)

---

### fontMetricsData.js — `CRITICAL`

**2078 lines** | Direct deps: 5 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\katex\src\fontMetricsData.js`

**Verdict:** 5 files directly depend on this. Transitive: 13388.

**Depends on:** DO.js, Math.js, default.js, export.js, file.js, is.js
**Depended by:** delimiter.js, fontMetrics.js, katex.js, macros.js, mermaid.js

---

### CoSELayout.js — `CRITICAL`

**2038 lines** | Direct deps: 8 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\cytoscape-fcose\node_modules\cose-base\src\CoSELayout.js`

**Verdict:** 8 files directly depend on this. Transitive: 13388.

**Depends on:** CoSEConstants.js, CoSEEdge.js, CoSEGraph.js, CoSEGraphManager.js, CoSENode.js, ConstraintHandler.js, DimensionD.js, FDLayout.js
**Depended by:** cose-base.js, cose.js, cytoscape-cose-bilkent.js, cytoscape-fcose.js, main.js, mermaid.js, mermaid.min.js, workbench.desktop.main.js

---

### prism.js — `CRITICAL`

**1947 lines** | Direct deps: 25 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\prismjs\prism.js`

**Verdict:** 25 files directly depend on this. Transitive: 13388.

**Depends on:** ASCII.js, Element.js, File.js, Function.js, Get.js, HTMLPreElement.js, HTMLScriptElement.js, IE.js
**Depended by:** LexicalCode.dev.js, LexicalCode.prod.js, components.js, prism-aspnet.js, prism-autoloader.js, prism-autoloader.min.js, prism-clojure.js, prism-core.js
**Classes:** of(0m), is(0m), can(0m)

---

### HTMLInputElement.js — `CRITICAL`

**1928 lines** | Direct deps: 16 | Transitive: 13388 | Path: `C:\Users\drago\AppData\Local\Programs\Antigravity\resources\app\node_modules\jsdom\lib\jsdom\living\generated\HTMLInputElement.js`

**Verdict:** 16 files directly depend on this. Transitive: 13388.

**Depends on:** DOMException.js, FileList.js, HTMLElement.js, SelectionMode.js, Set.js, String.js, Symbol.js, Window.js
**Depended by:** HTMLInputElement-impl.js, accessible-name-and-description.js, addon-webgl.js, bundle.min.js, defaultSettingsView-BEpdCv1S.js, dom.umd.js, domprops.js, index.BspFP3mn.js
**Classes:** HTMLInputElement(0m)

---
