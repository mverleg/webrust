use ::std::env;
use ::std::path::PathBuf;
use ::std::sync::LazyLock;

//TODO @mark: should this be separate per resource type?
static DOMAIN: LazyLock<String> = LazyLock::new(|| {
    env::var("WEBRUST_DOMAIN").unwrap_or_else(|_| "localhost:8080".to_owned())
});

static CSS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let mut resources = Vec::new();
    for css in ["app.css"] {
        let mut pth = PathBuf::from("");
        pth.push(name);
    }
    resources
});
