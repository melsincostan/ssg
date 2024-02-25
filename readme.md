### Static Site Generator

- Creating the expected file structure (assuming you have the rust project built as `ssg`: `./ssg init`. See the `FrontMatter` struct in `src/actions/build.rs` to see what is supposed to be in the frontmatter.
- Generating the site: `./ssg build`
- cleaning up the folder structure: `./ssg clean`
- The static site is generated in `./static`
- this is very inflexible, it is mostly meant for me to use as a random thing and fit that use case
- the code is a cognitohazard. I need to clean it up at some point.
- building: `cargo build`.
