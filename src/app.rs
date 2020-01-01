use std::error::Error;
use yew::prelude::*;

pub struct App {
    input: String,
    tweets: Result<Vec<String>, tweet_split::TweetSplitError>,
    tweet_size: usize,
    error_message: Option<String>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    UpdateInput(String),
    UpdateTweetSize(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            input: "".to_string(),
            tweets: Ok(vec![]),
            tweet_size: 280,
            error_message: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            UpdateInput(input) => {
                self.input = input;
                let tweets = tweet_split::split_text(&self.input, self.tweet_size);
                self.tweets = tweets;

                true
            }
            UpdateTweetSize(tweet_size_str) => {
                let tweet_size = tweet_size_str.parse::<usize>();
                match tweet_size {
                    Ok(tweet_size) => {
                        self.error_message = None;
                        self.tweet_size = tweet_size;

                        if !self.input.is_empty() {
                            let tweets_or_error =
                                tweet_split::split_text(&self.input, self.tweet_size);

                            match tweets_or_error {
                                Ok(_) => {
                                    self.tweets = tweets_or_error;
                                    true
                                }
                                Err(e) => {
                                    self.error_message = Some(e.description().to_string());
                                    self.tweets = Err(e);
                                    true
                                }
                            }
                        } else {
                            true
                        }
                    }
                    Err(e) => {
                        self.error_message = Some(e.to_string());
                        true
                    }
                }
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <textarea rows="5" cols="50"
                  value=self.input
                  oninput=self.link.callback(|e: InputData| Msg::UpdateInput(e.value))>
                </textarea>
                <div>
                    { format!("input length (characters): {}", self.input.len()) }
                </div>
                <div>
                    <input
                      value=self.tweet_size
                      oninput=self.link.callback(|e: InputData| Msg::UpdateTweetSize(e.value))>
                </input>
                <input type="range" min="1" max="280" class="slider" id="myRange"
                 value=self.tweet_size
                 oninput=self.link.callback(|e: InputData| Msg::UpdateTweetSize(e.value))>
                </input>
            {
                if let Some(err) = &self.error_message {
                    { format!("{}", err) }
                    }  else { format!("{}" , self.tweet_size) }
            }
                </div>

                {
                    if let Ok(tweets) = &self.tweets {
                        if !tweets.is_empty() {
                        html! {
                        <table>
                            <th>{ "Tweet number" }</th>
                            <th>{ "Length (characters)" }</th>
                            <th>{ "Tweet" }</th>
                            {
                                for tweets.iter().enumerate().map(|(i, tweet)| {
                                    html! {
                                        <tr>
                                            <td>
                                        { format!("{}", i + 1) }
                                            </td>
                                            <td>
                                        { format!("{}", tweet.len()) }
                                            </td>
                                            <td>
                                        { format!("{}", tweet) }
                                            </td>
                                        </tr>
                                    }
                                })
                            }
                        </table>
                        }
                    } else {
                        html! {}
                    }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }
}
