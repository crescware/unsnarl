# no-to-be

A lint script that detects Vitest `.toBe*` matchers in test files and
reports them as errors.

## Rules

The following matchers are forbidden.

| Matcher              | Replacement                            |
| -------------------- | -------------------------------------- |
| `.toBe()`            | `.toEqual()`                           |
| `.toBeDefined()`     | `expect(exists(v)).toEqual(true)`      |
| `.toBeNull()`        | `.toEqual(null)`                       |
| `.toBeInstanceOf(T)` | `expect(v instanceof T).toEqual(true)` |
| `.toBeTruthy()`      | `.toEqual(true)`                       |
| `.toBeFalsy()`       | `.toEqual(false)`                      |

## Usage

```sh
pnpm check:no-to-be
```

## Disabling a specific line

Place `// no-to-be-disable-next-line` on the line directly above the
offending expression to suppress the error.

```ts
// no-to-be-disable-next-line
expect(value).toBe(expected);
```

A reason may be appended after the directive.

```ts
// no-to-be-disable-next-line compare references
expect(value).toBe(expected);
```

The comment must be on the line immediately above the expression with
no blank line between them. Indentation is ignored.
