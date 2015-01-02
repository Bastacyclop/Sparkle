use sparkle::entity::{Record, Update};

#[test]
fn test_update_insertion() {
    let mut record = Record::new();
    let entity1 = 0u;
    let entity2 = 2u;
    let entity3 = 3u;

    record.add(Update::new_created(entity1));
    record.add(Update::new_created(entity2));
    record.add(Update::new_created(entity3));

    let expected_update_count = 3;
    assert_eq!(expected_update_count, record.get_update_count());
}

#[test]
fn test_unique_updates_with_one_entity() {
    let mut record = Record::new();

    let entity = 0u;
    record.add(Update::new_created(entity));
    record.add(Update::new_changed(entity));
    record.add(Update::new_removed(entity));

    let expected_update_count = 3;
    assert_eq!(expected_update_count, record.get_update_count());
}