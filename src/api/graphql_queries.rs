pub const USERS_QUERY: &str = r#"
  query Users {
    users {
      nodes {
        name
        id
      }
    }
  }
"#;

pub const ME_QUERY: &str = r#"
  query Me {
      viewer {
          id
          name
          email
      }
  }
"#;

pub const TEAMS_QUERY: &str = r#"
  query Teams {
    teams {
      nodes {
        id
        name
      }
    }
  }
"#;
