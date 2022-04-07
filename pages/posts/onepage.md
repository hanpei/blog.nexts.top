---
title: ONEPAGE - 静态blog生成器
date: 2022-04-07 11:20:00
tags:
  - rust
---

# ONEPAGE - 静态 blog 生成器

本站是由[ONEPAGE](https://github.com/hanpei/onepage)
这个项目生成的，一个由`rust`写的静态网站生成器。

其实整体思路是看到了这片文章[Build Your Own Static Site Generator](https://blog.hamaluik.ca/posts/build-your-own-static-site-generator/)，想起之前是 hexo 建站，就打算自己写个简单的替换下。

具体实现其实更像是发现各种合适的`rust crate`，然后把他们串联起来：

- Load files in direction by [walkdir](https://crates.io/crates/walkdir)
- Parse md to html by [pulldown cmark](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/)
- Render into [tera template](https://github.com/Keats/tera)
- styled by [picocss](https://picocss.com/) and [highlightjs](https://highlightjs.org/)
- Create your command-line by [clap](https://crates.io/crates/clap)

## 简单实现步骤

### 加载 markdown page

目前仅有两类页面：`index`和`post`。需要加载进来处理 md 在写入模板内。

```rust
pub trait LoadPage {
    type Item;
    fn load<P: AsRef<Path>>(path: P) -> Result<Self::Item>;
}
```

```rust
// post.rs
impl LoadPage for Post {
    type Item = Post;

    fn load<P: AsRef<Path>>(path: P) -> Result<Self::Item> {
        // read md file
        let raw_content = std::fs::read_to_string(&path)?;

        // 分离front matter和正文，目前front-matter必填，
        // 主要用来定义title/date和tags
        let (fm, md) = Self::read_front_matter(&raw_content, &path)?;

        let title = fm.title.clone();
        // 将markdown格式化为html
        let content = parse_md_to_html(&md);

        // 处理路径，去掉page前缀，方便后面output和url跳转
        let path = path.as_ref().strip_prefix(PAGE_DIR).unwrap().to_path_buf();
        Ok(Post {
            front_matter: fm,
            path: path.clone(),
            url: Path::new("/")
                .join(path)
                .with_extension("html")
                .display()
                .to_string(),
            title,
            content,
        })
    }
}
```

### 处理 markdown 和 html template

写了个`SiteBuilder`来处理`build`流程

```rust
// builder.rs
#[derive(Debug, Default)]
pub struct SiteBuilder {
    // 配置
    pub config: Config,
    // index页面
    pub index: IndexPage,
    // post页面
    pub posts: Posts,
}
```

`config.rs` 目前仅配置各种路径，并不支持自定义

```rust
// config.rs
#[derive(Debug)]
pub struct Config {
    // markdown file path
    pub page_dir: PathBuf,
    // static file path, include css, js, img..
    pub static_dir: PathBuf,
    // output file path
    pub output_dir: PathBuf,
}
```

具体 build 流程就是加载`index`和`post`页面，md 转化成 html，写入`tera`模板渲染。

```rust
// builder.rs
pub fn build(&mut self) -> Result<()> {
    // 加载`index`和`post`页面
    self.load();
    // 创建output目录
    if fs::metadata(&self.config.output_dir).is_ok() {
        fs::remove_dir_all(&self.config.output_dir)?;
    }
    fs::create_dir_all(&self.config.output_dir)?;

    println!("🏃🏻 Building post pages...");
    self.build_posts()?;
    println!("\t- {} post pages built.", self.posts.len());

    println!("🏃🏻 Building index page...");
    self.build_index()?;

    println!("🏃🏻 Copying static files...");
    self.build_statics()?;
    println!("✅ Build success.");
    println!();
    Ok(())
}
```

具体到`post`页面

```rust
//bulder.rs
fn build_posts(&mut self) -> Result<()> {
    let output = self.config.get_output_posts_path();
    fs::create_dir_all(output)?;

    for post in self.posts.as_ref() {
        // 将md转化的html当成content，写入template
        let rendered = templates::render_template(POST_TEMPLATE, post)?;
        let path = post.path.with_extension("html");
        let output = self.config.output_dir.join(path);

        std::fs::write(output, rendered)?;
    }
    // 将post下的image复制到output目录
    self.copy_pages_image()?;

    Ok(())
}
```

### 其他各种

基本流程就是这样，剩下的就是 cli 相关的处理，和自建 server 用来预览。

server 就用[axum](https://crates.io/crates/axum)配置下，起个线程跑起来。
再使用[hotwatch](https://crates.io/crates/hotwatch)监控`/pages`目录，有改动就 reload。

server 端的 reload 比较容易，前端浏览器的**liveload**搜了些资料，通过 websocket 解决，有变动就直接 reload，简单粗暴。

```js
socket.addEventListener('message', function (event) {
  console.log('Message from server ', event.data);
  if (event.data === 'reload') {
    window.location.reload();
  }
});
```

onepage 的具体代码在 [github](https://github.com/hanpei/onepage)。
