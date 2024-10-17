use std::collections::VecDeque;

use reqwest::StatusCode;
use serde_json::{from_value, Value};

pub fn handle_response(response: Value, long: usize, status: StatusCode) {
    let mut data = VecDeque::new();

    for i in 0..long {
        match response.get(i) {
            Some(value) => {
                let event = from_value::<String>(value.get("type").unwrap().clone()).unwrap();
                let repo =
                    from_value::<String>(value.get("repo").unwrap().get("name").unwrap().clone())
                        .unwrap();
                let payload = value.get("payload").unwrap();
                let message: String;

                match event.as_str() {
                    "CommitCommentEvent" => message = format!("commented on a commit in {}", repo), //1
                    "CreateEvent" => {
                        let _ref =
                            from_value::<String>(payload.get("ref").unwrap().clone()).unwrap();
                        let ref_type =
                            from_value::<String>(payload.get("ref_type").unwrap().clone()).unwrap();
                        message = format!("created a {} ({}) in {}", ref_type, _ref, repo);
                    } //2
                    "DeleteEvent" => {
                        let _ref =
                            from_value::<String>(payload.get("ref").unwrap().clone()).unwrap();
                        let ref_type =
                            from_value::<String>(payload.get("ref_type").unwrap().clone()).unwrap();
                        message = format!("deleted a {} ({}) in {}", ref_type, _ref, repo);
                    } //3
                    "ForkEvent" => {
                        let forkee = from_value::<String>(
                            payload.get("forkee").unwrap().get("name").unwrap().clone(),
                        )
                        .unwrap();
                        message = format!("forked {} to {}", repo, forkee);
                    } //4
                    "GollumEvent" => message = format!("updated the wiki in {}", repo), //5
                    "IssueCommentEvent" => {
                        let issue_num = from_value::<i64>(
                            payload.get("issue").unwrap().get("number").unwrap().clone(),
                        )
                        .unwrap();

                        message = format!("commented on issue {} in {}", issue_num, repo);
                    } //6
                    "IssuesEvent" => {
                        let action =
                            from_value::<String>(payload.get("action").unwrap().clone()).unwrap();
                        let issue_num = from_value::<i64>(
                            payload.get("issue").unwrap().get("number").unwrap().clone(),
                        )
                        .unwrap();

                        message = format!("{} issue {} in {}", action, issue_num, repo);
                    } //7
                    "MemberEvent" => {
                        let action =
                            from_value::<String>(payload.get("action").unwrap().clone()).unwrap();
                        let name = from_value::<String>(
                            payload.get("member").unwrap().get("name").unwrap().clone(),
                        )
                        .unwrap();

                        message = format!("{} {} in {}", action, name, repo);
                    } //8
                    "PublicEvent" => message = format!("change {} into public", repo),  //9
                    "PullRequestEvent" => {
                        let pull_req_num =
                            from_value::<i64>(payload.get("number").unwrap().clone()).unwrap();
                        let action =
                            from_value::<String>(payload.get("action").unwrap().clone()).unwrap();

                        message = format!("{} a pull request {} in {}", action, pull_req_num, repo);
                    } //10
                    "PullRequestReviewEvent" => {
                        let pull_req_num = from_value::<i64>(
                            payload
                                .get("pull_request")
                                .unwrap()
                                .get("number")
                                .unwrap()
                                .clone(),
                        )
                        .unwrap();

                        message = format!(
                            "submitted a review on pull request {} in {}",
                            pull_req_num, repo
                        );
                    } //11
                    "PullRequestReviewCommentEvent" => {
                        let pull_req_num = from_value::<i64>(
                            payload
                                .get("pull_request")
                                .unwrap()
                                .get("number")
                                .unwrap()
                                .clone(),
                        )
                        .unwrap();

                        message = format!(
                            "commented on pull request review {} in {}",
                            pull_req_num, repo
                        );
                    } //12
                    "PullRequestReviewThreadEvent" => {
                        let pull_req_num = from_value::<i64>(
                            payload
                                .get("pull_request")
                                .unwrap()
                                .get("number")
                                .unwrap()
                                .clone(),
                        )
                        .unwrap();

                        message = format!(
                            "replied to a review thread in pull request {} in {}",
                            pull_req_num, repo
                        )
                    } //13
                    "PushEvent" => {
                        let total_commit =
                            from_value::<i32>(payload.get("size").unwrap().clone()).unwrap();
                        let _ref =
                            from_value::<String>(payload.get("ref").unwrap().clone()).unwrap();

                        message =
                            format!("pushed {} commits to {} in {}", total_commit, _ref, repo);
                    } //14
                    "ReleaseEvent" => {
                        let tag_name = from_value::<String>(
                            payload
                                .get("release")
                                .unwrap()
                                .get("tag_name")
                                .unwrap()
                                .clone(),
                        )
                        .unwrap();

                        message = format!("published a release ({}) in {}", tag_name, repo)
                    } //15
                    //i'm still not find API for getting sponsored user
                    "SponsorshipEvent" => {
                        message = String::from("sponsored");
                    } //16
                    "WatchEvent" => message = format!("starred {}", repo), //17
                    _ => message = String::new(),
                }

                data.push_back(message);
            }
            None => {
                //stop loop when no response data remaining
                break;
            }
        }
    }

    if status == StatusCode::NOT_FOUND {
        println!("user not found");
        return ();
    }

    println!("showed from newer to older activities");
    for i in data {
        println!("-->> {}", i);
    }
}
