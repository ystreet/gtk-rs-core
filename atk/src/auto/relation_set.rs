// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Object;
use crate::Relation;
use crate::RelationType;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct RelationSet(Object<ffi::AtkRelationSet, ffi::AtkRelationSetClass>);

    match fn {
        get_type => || ffi::atk_relation_set_get_type(),
    }
}

impl RelationSet {
    #[doc(alias = "atk_relation_set_new")]
    pub fn new() -> RelationSet {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::atk_relation_set_new()) }
    }
}

impl Default for RelationSet {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_RELATION_SET: Option<&RelationSet> = None;

pub trait RelationSetExt: 'static {
    #[doc(alias = "atk_relation_set_add")]
    fn add<P: IsA<Relation>>(&self, relation: &P);

    #[doc(alias = "atk_relation_set_add_relation_by_type")]
    fn add_relation_by_type<P: IsA<Object>>(&self, relationship: RelationType, target: &P);

    #[doc(alias = "atk_relation_set_contains")]
    fn contains(&self, relationship: RelationType) -> bool;

    #[doc(alias = "atk_relation_set_contains_target")]
    fn contains_target<P: IsA<Object>>(&self, relationship: RelationType, target: &P) -> bool;

    #[doc(alias = "atk_relation_set_get_n_relations")]
    fn n_relations(&self) -> i32;

    #[doc(alias = "atk_relation_set_get_relation")]
    fn get_relation(&self, i: i32) -> Option<Relation>;

    #[doc(alias = "atk_relation_set_get_relation_by_type")]
    fn get_relation_by_type(&self, relationship: RelationType) -> Option<Relation>;

    #[doc(alias = "atk_relation_set_remove")]
    fn remove<P: IsA<Relation>>(&self, relation: &P);
}

impl<O: IsA<RelationSet>> RelationSetExt for O {
    fn add<P: IsA<Relation>>(&self, relation: &P) {
        unsafe {
            ffi::atk_relation_set_add(
                self.as_ref().to_glib_none().0,
                relation.as_ref().to_glib_none().0,
            );
        }
    }

    fn add_relation_by_type<P: IsA<Object>>(&self, relationship: RelationType, target: &P) {
        unsafe {
            ffi::atk_relation_set_add_relation_by_type(
                self.as_ref().to_glib_none().0,
                relationship.to_glib(),
                target.as_ref().to_glib_none().0,
            );
        }
    }

    fn contains(&self, relationship: RelationType) -> bool {
        unsafe {
            from_glib(ffi::atk_relation_set_contains(
                self.as_ref().to_glib_none().0,
                relationship.to_glib(),
            ))
        }
    }

    fn contains_target<P: IsA<Object>>(&self, relationship: RelationType, target: &P) -> bool {
        unsafe {
            from_glib(ffi::atk_relation_set_contains_target(
                self.as_ref().to_glib_none().0,
                relationship.to_glib(),
                target.as_ref().to_glib_none().0,
            ))
        }
    }

    fn n_relations(&self) -> i32 {
        unsafe { ffi::atk_relation_set_get_n_relations(self.as_ref().to_glib_none().0) }
    }

    fn get_relation(&self, i: i32) -> Option<Relation> {
        unsafe {
            from_glib_none(ffi::atk_relation_set_get_relation(
                self.as_ref().to_glib_none().0,
                i,
            ))
        }
    }

    fn get_relation_by_type(&self, relationship: RelationType) -> Option<Relation> {
        unsafe {
            from_glib_none(ffi::atk_relation_set_get_relation_by_type(
                self.as_ref().to_glib_none().0,
                relationship.to_glib(),
            ))
        }
    }

    fn remove<P: IsA<Relation>>(&self, relation: &P) {
        unsafe {
            ffi::atk_relation_set_remove(
                self.as_ref().to_glib_none().0,
                relation.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for RelationSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RelationSet")
    }
}
