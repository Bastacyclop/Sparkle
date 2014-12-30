use sparkle::entity::MetaEntity;
use sparkle::component::ComponentType;
use sparkle::system::Filter;

#[SparkleComponent]
struct AComponent;

#[SparkleComponent]
struct AnotherComponent;

#[test]
fn test_filter_with_multiple_types() {
    let filter = filter!(AComponent, AnotherComponent);

    let mut dummy_entity = MetaEntity::new(0u);
    let acomponent_index = ComponentType::get_index_of::<AComponent>();
    let another_component_index = ComponentType::get_index_of::<AnotherComponent>();

    dummy_entity.component_bits.insert(acomponent_index);
    dummy_entity.component_bits.insert(another_component_index);

    assert!(filter.check(&dummy_entity));

    dummy_entity.component_bits.remove(&another_component_index);

    assert!(!filter.check(&dummy_entity));

    dummy_entity.component_bits.remove(&acomponent_index);

    assert!(!filter.check(&dummy_entity));
}

#[test]
fn test_filter_with_no_types() {
    let filter = Filter::new();

    let mut dummy_entity = MetaEntity::new(0u);
    let acomponent_index = ComponentType::get_index_of::<AComponent>();

    assert!(filter.check(&dummy_entity));

    dummy_entity.component_bits.remove(&acomponent_index);

    assert!(filter.check(&dummy_entity));
}