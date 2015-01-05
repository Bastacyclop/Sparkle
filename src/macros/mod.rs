pub mod expand_component;

#[macro_export]
macro_rules! entity(
    ($em:expr, [$($component:expr),+]) => ({
        let entity = $em.create();
        $(
            $em.attach_component(&entity, $component);
        )+

        entity
    })
);

#[macro_export]
macro_rules! filter(
    ($($component_type:ident),*) => ({
        let mut filter = sparkle::system::Filter::new();
        $(
            filter.insert_mandatory::<$component_type>();
        )*

        filter
    })
);