pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    //Needed to make a new Configuration with just the word `test` to run all tests inside the
    // project (including this library).
    #[test]
    fn inbuilt_assert_eq() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn inbuilt_assert_ne() {
        let result = add(2, 2);
        assert_ne!(result, 3);
    }

    #[test]
    fn inbuilt_assert() {
        assert!(true);
        assert!(
            true,
            "Can send custom messages on failure {}",
            47
        )
    }

    //Tests can panic for a variety of reasons. So the expected panic message can be used to make
    // sure that the correct reason was returned. Note that the entire string does not need to be
    // entered, only a portion of it is just fine.
    #[test]
    #[should_panic (expected = "Failing on purpose")]
    fn expected_panic() {
        panic!("Failing on purpose is bad :(");
    }

    //This is apparently done so the `?` operator can be used.
    #[test]
    fn can_use_result_as_return() -> Result<(), String> {
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 should equal 4"))
        }
    }
}
