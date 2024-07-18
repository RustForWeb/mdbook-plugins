# Tabs

Plugin for rendering content in tabs.

## Example

{{#tabs }}
{{#tab name="Tab 1" }}
**Tab content 1**
{{#endtab }}
{{#tab name="Tab 2" }}
_Tab content 2_
{{#endtab }}
{{#tab name="Tab 3" }}
~~Tab content 3~~
{{#endtab }}
{{#endtabs }}

{{#tabs global="example" }}
{{#tab name="Global tab 1" }}
**Other tab content 1**
{{#endtab }}
{{#tab name="Global tab 2" }}
_Other tab content 2_
{{#endtab }}
{{#tab name="Global tab 3" }}
~~Other tab content 3~~
{{#endtab }}
{{#endtabs }}

{{#tabs global="example" }}
{{#tab name="Global tab 1" }}

```rust
let a = 1 + 2;
```

{{#endtab }}
{{#tab name="Global tab 2" }}

```python
a = 1 + 2
```

{{#endtab }}
{{#tab name="Global tab 3" }}

```js
const a = 1 + 2;
```

{{#endtab }}
{{#endtabs }}

-   [Book source code](https://github.com/RustForWeb/mdbook-plugins/tree/main/book)

## Installation

```shell
cargo install mdbook-tabs
```

## Configuration

Add the preprocessor to `book.toml`.

```toml
[preprocessor.tabs]
```

Add the additional CSS and JS files to the book with the following command.

```shell
mdbook-tabs install
```

Add the additional CSS and JS files to the HTML renderer in `book.toml`.

```toml
[output.html]
additional-css = ["theme/tabs.css"]
additional-js = ["theme/tabs.js"]
```

## Usage

Define tabs as follows:

```markdown
{{#tabs }}
{{#tab name="Tab 1" }}
**Tab content 1**
{{#endtab }}
{{#tab name="Tab 2" }}
_Tab content 2_
{{#endtab }}
{{#tab name="Tab 3" }}
~~Tab content 3~~
{{#endtab }}
{{#endtabs }}
```

The tabs can share a global state by adding a `global` string to the opening tag:

```markdown
{{#tabs global="example" }}
{{#tab name="Tab 1" }}
Some content.
{{#endtab }}
{{#endtabs }}
```
