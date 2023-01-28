# rust-todo-list

A simple application written in [Rust], implementing a simple todo list
with data persistency.

```ts
No items to show
Last updated at 28/01/2023 15:59:27

1. Add a new item
2. Remove an existing item
3. Set an item as complete/incomplete
4. Move an item up or down the list
5. Show this help message
0. Quit

Select a number: 1
Describe your todo item: Sketch a moonlit sky
Added item #1

[ ] 1. Sketch a moonlit sky
Last updated at 28/01/2023 15:59:43

Select a number: 3
Marked item #1 as completed

[X] 1. Sketch a moonlit sky
Last updated at 28/01/2023 15:59:46

Select a number: 1
Describe your todo item: Pet a cat
Added item #2

[X] 1. Sketch a moonlit sky
[ ] 2. Pet a cat
Last updated at 28/01/2023 15:59:54

Select a number: 1
Describe your todo item: Write a novel
Added item #3

[X] 1. Sketch a moonlit sky
[ ] 2. Pet a cat
[ ] 3. Write a novel
Last updated at 28/01/2023 15:59:59

Select a number: 4
Index of the item to move: 3
Index to move to: 1
Moved #3 to #1

[ ] 1. Write a novel
[X] 2. Sketch a moonlit sky
[ ] 3. Pet a cat
Last updated at 28/01/2023 16:00:01

Select a number: 0
```

When exiting, the user's list is stored in TOML format under the filename `todo_list.toml`:

```toml
last_updated = "2023-01-28T21:00:01.736641400Z"

[[items]]
description = "Write a novel"
completed = false

[[items]]
description = "Sketch a moonlit sky"
completed = true

[[items]]
description = "Pet a cat"
completed = false
```

[Rust]: https://www.rust-lang.org/
