# Static site generator thingie
## Landing page with links to latest articles + general information?
- 1 template for this
## Articles
- Templates

# Templates
- Handlebars files, select template in the markdown frontmatter?
    - Using maud would help to have checked templates at compile times but it removes flexibility from just throwing templates in a folder and parsing them at runtime

# Functions

- Automatically optimize images for serving
- Check images for alt text?
- Serving should not be done by this, it should just generate files to be dropped into a directory to be served by nginx

# Status 25.02.2024

It works, can generate what i need it to. The code needs cleanup, lots of it, probably.