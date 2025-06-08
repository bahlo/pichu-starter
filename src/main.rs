use std::{env, fs};

use chrono::NaiveDate;
use serde::Deserialize;

mod templates;

#[derive(Deserialize, Debug)]
pub(crate) struct Blogpost {
    title: String,
    published: NaiveDate,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Page {
    title: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::args().any(|arg| arg == "--watch") {
        build()?;
        pichu::watch("content", |paths| {
            println!("Rebuilding... (changed: {:?})", paths);
            if let Err(e) = build() {
                eprintln!("Build error: {}", e);
            }
        })?;
    } else {
        build()?;
    }
    Ok(())
}

fn build() -> Result<(), Box<dyn std::error::Error>> {
    fs::remove_dir_all("dist")?;

    let css_hash = pichu::render_sass("assets/scss/main.scss", "dist/main.css")?;

    let ctx = templates::Context {
        title: "Pichu starter".to_string(),
        css_hash,
    };

    pichu::glob("content/blog/*.md")?
        .parse_markdown::<Blogpost>()?
        .sort_by_key_reverse(|post| post.frontmatter.published)
        .render_each(
            |post| ctx.render_blogpost(post),
            |post| format!("dist/blog/{}/index.html", post.basename),
        )?
        .render_all(|posts| ctx.render_blog(posts), "dist/blog/index.html")?;

    pichu::glob("content/*.md")?
        .parse_markdown::<Page>()?
        .render_each(
            |page| ctx.render_page(page),
            |post| format!("dist/{}/index.html", post.basename),
        )?;

    pichu::write("dist/index.html", ctx.render_index().into_string())?;

    pichu::copy_dir("static", "dist")?;

    Ok(())
}
