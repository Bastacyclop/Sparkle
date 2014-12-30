use sparkle::component::ComponentType;

#[SparkleComponent]
struct AComponent;

#[SparkleComponent]
struct AnotherComponent;

#[test]
fn test_index_assignation() {
    let acomponent_index = ComponentType::get_index_of::<AComponent>();
    let another_index = ComponentType::get_index_of::<AnotherComponent>();

    assert!(acomponent_index != another_index);
}