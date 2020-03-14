use rhai::{Engine, EvalAltResult};

#[test]
fn test_constant() -> Result<(), EvalAltResult> {
    let mut engine = Engine::new();

    assert_eq!(engine.eval::<i64>("const x = 123; x")?, 123);

    match engine.eval::<i64>("const x = 123; x = 42;") {
        Err(EvalAltResult::ErrorAssignmentToConstant(var, _)) if var == "x" => (),
        Err(err) => return Err(err),
        Ok(_) => panic!("expecting compilation error"),
    }

    match engine.eval::<i64>("const x = [1, 2, 3, 4, 5]; x[2] = 42;") {
        Err(EvalAltResult::ErrorAssignmentToConstant(var, _)) if var == "x" => (),
        Err(err) => return Err(err),
        Ok(_) => panic!("expecting compilation error"),
    }

    Ok(())
}
