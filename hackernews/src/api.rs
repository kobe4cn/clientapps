use crate::{Comment, StoryItem, StoryPageData};
use anyhow::Result;

const MAX_TOP_STORIES: usize = 50;

pub async fn get_top_stories(n: usize) -> Result<Vec<StoryItem>> {
    let n = n.min(MAX_TOP_STORIES);
    let url = "https://hacker-news.firebaseio.com/v0/topstories.json";
    let ids = reqwest::get(url).await?.json::<Vec<i64>>().await?;
    let stories_futures = ids.into_iter().take(n).map(get_story_item);
    let stories = futures::future::join_all(stories_futures)
        .await
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();
    Ok(stories)
}

pub async fn get_story_item(id: i64) -> Result<StoryItem> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let story = reqwest::get(&url).await?.json().await?;
    Ok(story)
}

pub async fn get_comment(id: i64) -> Result<Comment> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let comment = reqwest::get(&url).await?.json::<Comment>().await?;
    Ok(comment)
}

pub async fn get_story_comments(item: StoryItem) -> Result<StoryPageData> {
    let comment_futures = item.kids.iter().map(|id| get_comment(*id));
    let comments = futures::future::join_all(comment_futures)
        .await
        .into_iter()
        .filter_map(|r| r.ok())
        .collect::<Vec<Comment>>();
    Ok(StoryPageData { item, comments })
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[tokio::test]
    async fn test_get_top_stories() -> Result<()> {
        let stories = get_top_stories(MAX_TOP_STORIES).await?;
        println!("++++++++++++++++{:?}", stories.len());
        assert_eq!(stories.len(), MAX_TOP_STORIES);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_story_item() -> Result<()> {
        let id = 8863;
        let story = get_story_item(id).await?;
        assert_eq!(story.id, id);
        Ok(())
    }

    #[tokio::test]
    async fn get_comment_by_id() -> Result<()> {
        let story = get_story_item(1).await?;
        let id = story.kids[0];
        let comment = get_comment(id).await?;
        assert_eq!(comment.id, id);
        Ok(())
    }
}
