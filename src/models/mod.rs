pub struct Post {
  pub id: String,
  pub title: String,
}

impl Post {
  pub fn all() -> Vec<Post> {
    vec![
      Post {
        id: String::from("1"),
        title: String::from("First post"),
      },
      Post {
        id: String::from("2"),
        title: String::from("Second post"),
      },
    ]
  }

  pub fn find(id: String) -> Option<Post> {
    match Self::all().iter().find(|post| post.id == id) {
      Some(post) => Some(Post {
        id: post.id.to_owned(),
        title: post.title.to_owned(),
      }),
      None => None,
    }
  }
}
