pub const ME: &str = r#"
    query Me {
        viewer {
            id
            name
            email
        }
    }
"#;

// pub const USERS: &str = r#"
//   query Users {
//     users {
//       nodes {
//         name
//         id
//       }
//     }
//   }
// "#;

pub const TEAMS: &str = r#"
    query Teams {
        teams {
            nodes {
                id
                name
            }
        }
    }
"#;

pub const ISSUES: &str = r#"
    query UserIssues($userId: ID!, $stateName: String, $teamName: String) {
        issues(filter: { assignee: { id: { eq: $userId } }, state: { name: { eq: $stateName } }, team: { name: { eq: $teamName}} }) {
            nodes {
                id
                title
                state {
                    id
                    name
                }
                team {
                    id
                    name
                }
            }
        }
    }
"#;

pub const ISSUE: &str = r#"
    query Issue($issueId: String!) {
        issue(id: $issueId) {
            id
            title
            description
            state {
                id
                name
            }
            team {
                id
                name
            }
            comments {
                nodes {
                    id
                    body
                    createdAt
                }
            }
        }
    }
"#;
