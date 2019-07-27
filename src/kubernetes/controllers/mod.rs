pub mod container;
pub mod service;

// trait use to generate a BTree Map
pub mod common {
  use std::collections::BTreeMap;

  /**
   * Trait use to help the construction of the Yaml
   */
  pub trait KubeHelper<T, Y> {
    /**
     * Get Tree Map
     */
    fn get_tree_map(&self) -> BTreeMap<T, Y>;
  }
}