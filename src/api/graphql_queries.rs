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

pub const TEAM: &str = r#"
    query Team($teamId: String!) {
        team(id: $teamId) {
            id
            name
            description
        }
    }
"#;

pub const ISSUES: &str = r#"
    query UserIssues($userId: ID!, $stateName: String, $teamId: ID) {
        issues(filter: { assignee: { id: { eq: $userId } }, state: { name: { eq: $stateName } }, team: { id: { eq: $teamId}} }) {
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
