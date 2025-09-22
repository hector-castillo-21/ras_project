use ras_project::math;

#[test]
fn test_integration_math() {
    let a = 10;
    let b = 4;

    assert_eq!(math::suma(a, b), 14);
    assert_eq!(math::resta(a, b), 6);
}
