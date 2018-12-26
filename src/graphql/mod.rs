use juniper::{EmptyMutation, FieldResult};

pub struct Context;
impl juniper::Context for Context {}

#[derive(GraphQLObject)]
#[graphql(description = "A blog post")]
struct Post {
  id: juniper::ID,
  title: String,
}

impl Post {
  fn from_model(post: &crate::models::Post) -> Post {
    Post {
      id: juniper::ID::from(String::from(post.id.to_owned())),
      title: String::from(post.title.to_owned()),
    }
  }
}

pub struct Query;
graphql_object!(Query: Context |&self| {
  field post(&executor, id: String) -> FieldResult<Option<Post>> {
    match crate::models::Post::find(id) {
      Some(post) => Ok(Some(Post::from_model(&post))),
      None => Err("Post not found")?
    }
  }
});

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;
