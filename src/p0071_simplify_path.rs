pub struct Solution {}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let tokens = path.split('/');
        let mut stack = Vec::new();
        for token in tokens {
            match token {
                "." | "" => continue,
                ".." => { stack.pop(); },
                _ => { stack.push(token); }
            }
        }

        return format!("{}{}", "/",  stack.join("/"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("/home", Solution::simplify_path("/home/".to_string()));
        assert_eq!("/", Solution::simplify_path("/../".to_string()));
        assert_eq!("/home/foo", Solution::simplify_path("/home//foo/".to_string()));
        assert_eq!("/home/bar", Solution::simplify_path("/home//foo/./../bar/".to_string()));
        assert_eq!("/", Solution::simplify_path("/".to_string()));
    }
}
