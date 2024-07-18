# Tabs

Plugin for rendering content in tabs.

## Example

{{#tabs global="example" }}
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
{{#tab name="Tab 1" }}
**Other tab content 1**
{{#endtab }}
{{#tab name="Tab 2" }}
_Other tab content 2_
{{#endtab }}
{{#tab name="Tab 3" }}
~~Other tab content 3~~
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

## Usage

TODO
