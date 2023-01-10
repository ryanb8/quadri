use std::hash::{Hash, Hasher};

static ALLOWED_ATTRIBUTE_VALUES: [i8; 2] = [0, 1];

fn is_allowed_value(value: i8) -> bool {
    let mut is_valid = false;
    for av in ALLOWED_ATTRIBUTE_VALUES {
        if value == av {
            is_valid = true;
        }
    }
    is_valid
}

#[derive(Debug, Clone)]
pub struct GamePiece {
    pub sig: String,
    pub ats: Vec<i8>,
}

impl PartialEq for GamePiece {
    fn eq(&self, other: &Self) -> bool {
        self.ats == other.ats
    }
}
impl Eq for GamePiece {}
impl Hash for GamePiece {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sig.hash(state);
        self.ats.hash(state);
    }
}

impl GamePiece {
    pub fn new_from_vec(values: Vec<i8>) -> Result<GamePiece, String> {
        let dim: i8 = values.len().try_into().unwrap();
        if dim != 4 {
            return Err("Need 4 values for a piece".to_string());
        }
        for v in &values {
            if !is_allowed_value(*v) {
                return Err(format!(
                    "Values for pieces must be in {:?}",
                    ALLOWED_ATTRIBUTE_VALUES
                ));
            }
        }
        let name = values
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("");

        Ok(GamePiece {
            sig: name,
            ats: values,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_from_vec_standard() -> Result<(), String> {
        let input_vec = vec![1 as i8, 0 as i8, 1 as i8, 0 as i8];
        let expected = GamePiece {
            sig: "1010".to_string(),
            ats: input_vec.clone(),
        };
        let actual = GamePiece::new_from_vec(input_vec)?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_num_to_alpha_len_errors() {
        let short_vec = vec![0 as i8, 1 as i8];
        let long_vec = vec![0 as i8, 1 as i8, 0 as i8, 0 as i8, 0 as i8];
        assert!(
            GamePiece::new_from_vec(short_vec).is_err(),
            "Shorter than 4 should return error"
        );
        assert!(
            GamePiece::new_from_vec(long_vec).is_err(),
            "Longer than 4 should return error"
        );
    }

    #[test]
    fn test_num_to_alpha_value_errors() {
        let bad_vec = vec![2 as i8, 1 as i8, 0 as i8, 1 as i8];
        assert!(
            GamePiece::new_from_vec(bad_vec).is_err(),
            "Should only support 0 and 1"
        );
    }
}
