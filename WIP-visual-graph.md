# WIP: visual-graph branch — JSON 形状の合意未済

ブランチ: `feat/visual-graph`

## ここまでにコミット済み

```
7fa1940 feat(visual-graph): extract renderer-agnostic graph model from MermaidEmitter
10745b8 refactor: rename "json" emitter to "ir" to reflect what it actually is
bcc98c1 fix(mermaid): restore 'import ' prefix on default and same-name named imports
dc74432 test: split scenario tests into IR-level and Mermaid-rendering layers
```

`bcc98c1` と `10745b8` は `main` 上のコミット (本ブランチを切る前)。`7fa1940` が本ブランチの内容。

### 現状の構成

```
parser → analyzer → serializer
                       ↓
                  SerializedIR
                       ↓
                buildVisualGraph
                       ↓
                  VisualGraph  ─── JsonEmitter   (format: "json")
                       ↓               ↓
                MermaidEmitter   JSON.stringify
                       ↓
                Mermaid 文字列
```

- `format: "ir"` … SerializedIR の JSON dump (旧 "json")
- `format: "json"` … VisualGraph の JSON dump (新)
- `format: "mermaid"` … VisualGraph → Mermaid 文字列
- `format: "markdown"` … mermaid をフェンス整形

Mermaid 出力は byte-for-byte 不変。全 284 テスト通過。

## 未解決: VisualGraph JSON の形状

当初の flat な形状に対し、画像が持つ階層性を表現するには AST のようにネストした構造が必要で、エッジの from / to だけでは所属関係を決定できない、というフィードバックを受けた。

つまり現状の **flat な `nodes[]` + `subgraphs[]` + 各エントリに `parent: id`** は、画像の絵が持つ階層性を JSON で表現できていない、という指摘。

### 議論の到達点

- **所属 (containment)** はネスト構造で表現する。subgraph が `nodes[]` と `subgraphs[]` を再帰的に持ち、`parent` フィールドは廃止。各ノードは木の中で 1 箇所だけに住む。
- **接続 (edges)** は subgraph 境界を跨ぎ、本質的にグラフなので **トップレベルに flat な配列** で持つ。エッジを subgraph 内にネストすると親子関係が曖昧になる。
- **分離 / 合流 (fan-out / fan-in)** は **ノードを複製しない**。同じ id で複数箇所に書くと、どれが本物か / 属性の重複 / 「ノード」と「ノードへの言及」の区別がつかない、という問題が出る。fan-out は同じ `from` を持つ複数エッジ、fan-in は同じ `to` を持つ複数エッジで表現する。

### まだ決まっていないこと

1. **siblings の順序** — 同じ subgraph 内に「ノード」と「子 subgraph」が混在する場合 (例: function 本体)、`nodes[]` と `subgraphs[]` を別配列で持って暗黙に「nodes 全部 → subgraphs 全部」とするか、`children[]` という discriminator 付き mixed 配列で陽に表現するか。

2. **トップレベル synthetic ノード (ModuleSource / ImportIntermediate / ModuleSink)** の置き場所 — top-level `nodes[]` に並べるか、`imports[]` などに分けるか。

3. **VisualSubgraph に `unused` を持たせるか** — function subgraph (= 関数変数の代替表現) が unused になることがあるので必要だが、subgraph という概念に「使われている / 使われていない」属性を持たせるのが妥当か。現状は持たせている。

## 残作業 (合意取れたら着手)

1. `src/visual-graph/model.ts` を編集 — `parent` 廃止、`VisualSubgraph` に `nodes[]` / `subgraphs[]` を追加 (再帰)。 → 一部着手済みで現在 `git status` 上には未コミット差分がある。**ロールバックするか、続けるかは要相談**。
2. `src/visual-graph/builder.ts` を編集 — `parent: string | null` 引数の代わりに「現在のコンテナ」(VisualGraph or VisualSubgraph) を引き回し、`container.nodes.push(...)` / `container.subgraphs.push(...)` で構築。
3. `src/emitter/mermaid.ts` を編集 — flat な `nodesByParent` / `subgraphsByParent` Map ベースの walk を、再帰的な subgraph traversal に。
4. `integration/fixtures/*/expected.json` を再生成 (`vitest -u`)。**Mermaid と IR の snapshot は不変のはず** だが念のため確認。
5. `src/emitter/json.test.ts` の assertion を nested 前提に修正 (例: `graph.nodes` の中に WriteOp はもう含まれない)。

## 現在の作業ツリーの状態

途中で「合意とり切ってない」とユーザに止められたため、`src/visual-graph/model.ts` だけネスト前提に書き換えてある状態。`builder.ts` と `mermaid.ts` は flat 前提のまま (= ビルドが壊れている可能性が高い)。

```bash
git status   # untracked: WIP-visual-graph.md
             # modified:  package.json (exec script — このブランチと無関係)
             # modified:  src/visual-graph/model.ts (nested 化、未コミット)
```

再開時は **まず合意ポイント (上記「まだ決まっていないこと」) を確定** してから、`src/visual-graph/model.ts` の差分を保つか revert するか決め、残作業 2 〜 5 を進める。

## 参考: 現状 (flat) の JSON 例

`integration/fixtures/control-switch-break/expected.json` (commit `7fa1940` 時点):

```json
{
  "nodes": [
    { "id": "n_scope_0_label_4", "kind": "Variable", "parent": null, ... },
    { "id": "wr_ref_1", "kind": "WriteOp", "parent": "s_scope_2", ... }
  ],
  "subgraphs": [
    { "id": "s_scope_1", "kind": "switch", "parent": null, ... },
    { "id": "s_scope_2", "kind": "case", "parent": "s_scope_1", ... }
  ],
  "edges": [...]
}
```

目標 (nested):

```json
{
  "nodes": [
    { "id": "n_scope_0_label_4", "kind": "Variable", ... }
  ],
  "subgraphs": [
    {
      "id": "s_scope_1",
      "kind": "switch",
      "nodes": [],
      "subgraphs": [
        {
          "id": "s_scope_2",
          "kind": "case",
          "caseTest": "\"a\"",
          "nodes": [{ "id": "wr_ref_1", "kind": "WriteOp", ... }],
          "subgraphs": []
        }
      ]
    }
  ],
  "edges": [...]
}
```
