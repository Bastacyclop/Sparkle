use sparkle::component::StoreMap;

#[SparkleComponent]
struct DummyComponent;

#[test]
fn test_component_insertion() {
    let mut store_map = StoreMap::new();

    for entity in range(0u, 100) {
        store_map.attach_component(&entity, DummyComponent);
        assert!(store_map.get_component::<DummyComponent>(&entity).is_some());
    }
}

#[test]
fn test_component_suppression() {
    let mut store_map = StoreMap::new();

    for entity in range(0u, 100) {
        store_map.attach_component(&entity, DummyComponent);
        store_map.detach_component::<DummyComponent>(&entity);

        assert!(store_map.get_component::<DummyComponent>(&entity).is_none());
    } 
}

#[test]
fn test_has_component() {
    let mut store_map = StoreMap::new();

    for entity in range(0u, 100) {
        store_map.attach_component(&entity, DummyComponent);

        assert!(store_map.has_component::<DummyComponent>(&entity));
    }
}