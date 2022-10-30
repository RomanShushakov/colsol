use crate::{factorization, find_unknown};


#[test]
fn test_factorization() -> Result<(), String>
{
    let nn_1 = 4i64;
    let mut a_1 = vec![5.0, 6.0, -4.0, 6.0, -4.0, 1.0, 5.0, -4.0, 1.0];
    let maxa_1 = vec![0i64, 1, 3, 6, 9];

    let nn_2 = 5i64;
    let mut a_2 = vec![2.0, 3.0, -2.0, 5.0, -2.0, 10.0, -3.0, 10.0, 4.0, 0.0, 0.0, -1.0];
    let maxa_2 = vec![0, 1, 3, 5, 7, 12];

    let nn_3 = 6i64;
    let mut a_3 = vec![5280.0, 907.5, 907.5, 12932.03125, 232506840.0, 0.0, -453750.0, 262259000.0, 0.0, 
        0.0, 0.0, -453750.0];
    let maxa_3 = vec![0, 1, 2, 3, 4, 7, 12];

    let a_expected_1 = vec![5.0, 2.8, -0.8, 2.142857142857142, -1.142857142857143, 0.2, 0.833333333333333, 
        -1.3333333333333337, 0.35714285714285715];
    let a_expected_2 = vec![2.0, 1.0, -1.0, 1.0, -2.0, 1.0, -3.0, 0.5, -2.0, -2.0, -1.0, -0.5];
    let a_expected_3 = vec![5280.0, 907.5, 907.5, 12932.03125, 5631840.0, 0.0, -500.0, 35384000.0, 0.0, 0.0, 
        0.0, -500.0];

    factorization(&mut a_1, nn_1, &maxa_1)?;

    factorization(&mut a_2, nn_2, &maxa_2)?;

    factorization(&mut a_3, nn_3, &maxa_3)?;

    assert_eq!(a_1, a_expected_1);
    assert_eq!(a_2, a_expected_2);
    assert_eq!(a_3, a_expected_3);

    Ok(())
}


#[test]
fn test_find_unknown() -> Result<(), String>
{
    let nn_1 = 4i64;
    let mut a_1 = vec![5.0, 6.0, -4.0, 6.0, -4.0, 1.0, 5.0, -4.0, 1.0];
    let maxa_1 = vec![0i64, 1, 3, 6, 9];
    let mut v_1 = vec![0.0, 1.0, 0.0, 0.0];

    let nn_2 = 5i64;
    let mut a_2 = vec![2.0, 3.0, -2.0, 5.0, -2.0, 10.0, -3.0, 10.0, 4.0, 0.0, 0.0, -1.0];
    let maxa_2 = vec![0, 1, 3, 5, 7, 12];
    let mut v_2 = vec![0.0, 1.0, 0.0, 0.0, 0.0];

    let nn_3 = 6i64;
    let mut a_3 = vec![5280.0, 907.5, 907.5, 12932.03125, 232506840.0, 0.0, -453750.0, 262259000.0, 0.0, 
        0.0, 0.0, -453750.0];
    let maxa_3 = vec![0, 1, 2, 3, 4, 7, 12];
    let mut v_3 = vec![0.0, -1000.0, 0.0, 0.0, 0.0, 0.0];

    let v_expected_1 = vec![1.6000000000000014, 2.6000000000000023, 2.400000000000002, 1.400000000000001];
    let v_expected_2 = vec![636.0, 619.0, 292.0, 74.0, 34.0];
    let v_expected_3 = vec![0.0, -8.167268641442895, 0.0, 0.0, 0.0, -0.014130680533574497];

    factorization(&mut a_1, nn_1, &maxa_1)?;
    find_unknown(&a_1, &mut v_1, nn_1, &maxa_1);

    factorization(&mut a_2, nn_2, &maxa_2)?;
    find_unknown(&a_2, &mut v_2, nn_2, &maxa_2);

    factorization(&mut a_3, nn_3, &maxa_3)?;
    find_unknown(&a_3, &mut v_3, nn_3, &maxa_3);

    assert_eq!(v_1, v_expected_1);
    assert_eq!(v_2, v_expected_2);
    assert_eq!(v_3, v_expected_3);

    Ok(())
}