fn should_lock_device(passcode: String, attempts: Vec<String>) -> bool {
    let mut count = 0;
    for attempt in attempts {
        if attempt == passcode {
            count = 0;
        } else {
            count += 1;
        }
        if count == 10 {
            return true;
        }
    }
    false
}

fn main() {
    let passcode = "1111".to_string();
    let at = vec![
        "1111".to_string(),
        "4444".to_string(),
        "9999".to_string(),
        "3333".to_string(),
        "8888".to_string(),
        "2222".to_string(),
        "7777".to_string(),
        "0000".to_string(),
        "6666".to_string(),
        "7285".to_string(),
        "5555".to_string(),
    ];
    dbg!(should_lock_device(passcode, at));
}

#[test]
fn test_input_1() {
    let passcode = "1111".to_string();
    let at = vec![
        "1111".to_string(),
        "4444".to_string(),
        "9999".to_string(),
        "3333".to_string(),
        "8888".to_string(),
        "2222".to_string(),
        "7777".to_string(),
        "0000".to_string(),
        "6666".to_string(),
        "7285".to_string(),
        "5555".to_string(),
    ];
    assert!(should_lock_device(passcode, at));
}

#[test]
fn test_input_2() {
    let passcode = "1234".to_string();
    let at = vec![
        "9999".to_string(),
        "9999".to_string(),
        "9999".to_string(),
        "9999".to_string(),
        "9999".to_string(),
        "9999".to_string(),
        "9999".to_string(),
        "1234".to_string(),
        "9999".to_string(),
        "9999".to_string(),
        "9999".to_string(),
        "9999".to_string(),
    ];
    assert!(!should_lock_device(passcode, at));
}

#[test]
fn test_input_3() {
    let passcode = "2725".to_string();
    let at = vec![
        "3299".to_string(),
        "5574".to_string(),
        "7125".to_string(),
        "3434".to_string(),
        "2890".to_string(),
        "2725".to_string(),
        "1016".to_string(),
        "3996".to_string(),
        "2976".to_string(),
        "5982".to_string(),
        "1026".to_string(),
        "4771".to_string(),
        "1294".to_string(),
        "3014".to_string(),
        "5338".to_string(),
        "8987".to_string(),
    ];
    assert!(should_lock_device(passcode, at));
}
