use sparkle::entity::Pool;

#[test]
fn test_meta_entity_creation() {
    let mut pool = Pool::new();

    for i in range(0u, 100) {
        assert_eq!(i, pool.get().entity);
    }
}

#[test]
fn test_meta_entity_reuse() {
    let mut pool = Pool::new();
    let dummy_component_index = 7u;

    let mut mentity = pool.get();
    let entity = mentity.entity; 

    mentity.component_bits.insert(dummy_component_index);
    pool.put(mentity);

    let reused = pool.get();
    assert_eq!(entity, reused.entity);
    assert!(reused.component_bits.is_empty());
}