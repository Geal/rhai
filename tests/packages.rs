use rhai::packages::{Package, StandardPackage};
use rhai::{Engine, EvalAltResult, Module, Scope, INT};

#[test]
fn test_packages() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let ast = engine.compile("x")?;
    let std_pkg = StandardPackage::new();

    let make_call = |x: INT| -> Result<INT, Box<EvalAltResult>> {
        // Create a raw Engine - extremely cheap.
        let mut engine = Engine::new_raw();

        // Load packages - cheap.
        engine.load_package(std_pkg.get());

        // Create custom scope - cheap.
        let mut scope = Scope::new();

        // Push variable into scope - relatively cheap.
        scope.push("x", x);

        // Evaluate script.
        engine.eval_ast_with_scope::<INT>(&mut scope, &ast)
    };

    assert_eq!(make_call(42)?, 42);

    Ok(())
}

#[test]
fn test_packages_with_script() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();
    let ast = engine.compile("fn foo(x) { x + 1 }")?;
    let module = Module::eval_ast_as_new(Scope::new(), &ast, &engine)?;

    engine.load_package(module);

    assert_eq!(engine.eval::<INT>("foo(41)")?, 42);

    Ok(())
}
