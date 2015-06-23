// This program takes two vectors, each representing a binary integer with each element of the
// vector being one bit in the integer, and returns a third vector that is the sum of the two
// inputs. The input vectors must have the same size n, and the returned vector should have size
// n+1.

// note that std:collections::bit_vec::BitVec would be more concise than Vec<bool> but BitVec is
// currently unstable
fn binary_add(a: &Vec<bool>, b: &Vec<bool>) -> Result<Vec<bool>, String> {
    if a.len() != b.len() {
        return Err(
            format!(
                "Input vectors have different lengths ({} and {}, respectively).",
                a.len(),
                b.len()
            )
        );
    }

    let mut sum: Vec<bool> = Vec::with_capacity(a.len() + 1);
    for(i, a_value) in a.iter().enumerate() {
        let a_value: bool = *a_value;
        let b_value: bool = b[i];
        let mut bitwise_sum = a_value ^ b_value;
        let mut carry_over = a_value && b_value;
        if i > 0 && sum[i] {
            if bitwise_sum {
                carry_over = true
            } else {
                bitwise_sum = true
            }
        }
        if i == 0 {
            sum.push(bitwise_sum);
        } else {
            sum[i] = bitwise_sum;
        }
        sum.push(carry_over);
    }

    return Ok(sum);
}


#[test]
fn test_different_length_inputs() {
    let a = vec![true, true];
    let b = vec![true, true, true];

    let output = binary_add(&a, &b);
    assert!(output.is_err());
}
#[test]
fn test_empty_inputs() {
    let a = vec![];
    let b = vec![];
    let expected_output = Ok(vec![]);

    let output = binary_add(&a, &b);
    assert!(output == expected_output);
}
#[test]
fn test_valid_input_without_most_significant_carrying() {
    let a = vec![false, false, true, true, false];
    let b = vec![false, true, false, true, false];
    let expected_output = Ok(vec![false, true, true, false, true, false]);

    let output = binary_add(&a, &b);
    assert!(output == expected_output);
}
#[test]
fn test_valid_input_with_most_significant_carrying() {
    let a = vec![true, true, false, true, true];
    let b = vec![true, true, false, false, true];
    let expected_output = Ok(vec![false, true, true, true, false, true]);

    let output = binary_add(&a, &b);
    assert!(output == expected_output);
}
