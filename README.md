# wordpress-v2

Unofficial library for the WordPress REST API

## Example:

```rust
let url = url::Url::parse("https://hard-drive.net").unwrap();
let wp = wordpress_v2::Wordpress::new(url);

// Get the 8 newest posts that are at least 48 hours old
let posts = wp
    .list_posts()
    .before(chrono::Utc::now() - chrono::Duration::days(2))
    .per_page(8)
    .send()
    .await
    .unwrap();

for post in posts {
    println!(
	r#"Post {} at {} with title "{}""#,
	post.id,
	post.date_gmt.unwrap().to_rfc3339(),
	post.title.rendered,
    );
}
```
