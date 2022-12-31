# Statue

Spare yourself from writing

```rust
let work_area = document
    .query_selector("#work-area")
    .unwrap()
    .unwrap()
    .dyn_into::<HtmlDivElement>()
    .unwrap();

let layer_list_div = document
    .query_selector("#layer-list")
    .unwrap()
    .unwrap()
    .dyn_into::<HtmlDivElement>()
    .unwrap();

let save_files_btn = document
    .query_selector("#save-files")
    .unwrap()
    .unwrap()
    .dyn_into::<HtmlButtonElement>()
    .unwrap();
```

and write

```rust
initialize_elements!(
    html: "index.html", elements: {
        let work_area = Single("#work-area");
        let layer_list_div = Single("#layer-list");   
        let save_files_btn = Single("#save-files");
    }
);
```

instead.
